//! The schema audit: what the API sends that the generated types do not describe.
//!
//! Runs the live suite with the `audit` feature, which makes the deserializer report the keys it ignored and every
//! open enum report a value the specification never listed. Each finding is appended to a file as it is made, so a
//! run that fails half way through still reports what it learned.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

type Failure = Box<dyn std::error::Error>;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum Finding {
    UndocumentedKeys { endpoint: String, path: String },
    UndocumentedValue { type_name: String, value: String, documented: Vec<String> },
}

/// The whole hosted surface when nothing was named, and exactly what was named otherwise.
///
/// The self-hosted suites need a container brought up first, and an audit run has only the hosted credentials — left
/// in, they fail on a connection rather than reporting what the types do not describe. The same boundary is drawn for
/// a plain run by the `live` alias in `.cargo/config.toml`; the two are stated apart because a filter given here is
/// meant to override it, and they have to be changed together the day a self-hosted surface is added.
fn default_filter(arguments: &[String]) -> Vec<String> {
    if !arguments.is_empty() {
        return arguments.to_vec();
    }

    ["--skip", "server::", "--skip", "jsm::"].into_iter().map(str::to_owned).collect()
}

pub async fn run(workspace_root: &std::path::Path, arguments: &[String]) -> Result<(), Failure> {
    let output: PathBuf = std::env::temp_dir().join("jira-rs-schema-audit.jsonl");
    let _ = std::fs::remove_file(&output);

    let mut command = Command::new("cargo");

    // Anything extra is a test filter, so it goes after the separator: `cargo xtask audit cloud::issues` audits one
    // suite rather than asking cargo to make sense of the name.
    command
        .current_dir(workspace_root)
        .args(["test", "--test", "live", "--all-features"])
        .args(["--", "--ignored", "--test-threads=1"])
        .args(default_filter(arguments))
        .env("JIRA_AUDIT_OUTPUT", &output);

    println!("▸ running the live suite with the audit on");

    let status = command.status()?;

    let report = read_findings(&output)?;

    println!("\n{report}");

    if let Ok(summary) = std::env::var("GITHUB_STEP_SUMMARY") {
        std::fs::write(summary, &report)?;
    }

    if !status.success() {
        return Err("the live suite failed; the findings above are what it collected before stopping".into());
    }

    Ok(())
}

fn read_findings(output: &std::path::Path) -> Result<String, Failure> {
    let Ok(contents) = std::fs::read_to_string(output) else {
        return Ok("## Schema audit\n\nNothing was recorded: the types describe every response the suite read.".into());
    };

    let mut keys: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut values: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in contents.lines().filter(|line| !line.trim().is_empty()) {
        match serde_json::from_str::<Finding>(line)? {
            Finding::UndocumentedKeys { endpoint, path } => {
                let path = if path.is_empty() { "(root)".to_owned() } else { collapse_indices(&path) };

                keys.entry(anonymise(&endpoint)).or_default().push(path);
            }
            Finding::UndocumentedValue { type_name, value, documented } => {
                values.entry(type_name).or_default().push(format!("`{value}` (documented: {})", documented.join(", ")));
            }
        }
    }

    let mut report = String::from("## Schema audit\n");

    if keys.is_empty() && values.is_empty() {
        report.push_str("\nNothing was recorded: the types describe every response the suite read.\n");

        return Ok(report);
    }

    if !keys.is_empty() {
        report.push_str("\n### Fields the API sends and the types do not describe\n\n");

        for (endpoint, mut paths) in keys {
            paths.sort();
            paths.dedup();
            report.push_str(&format!("- `{endpoint}` — {}\n", paths.join(", ")));
        }
    }

    if !values.is_empty() {
        report.push_str("\n### Values an enum had to grow for\n\n");

        for (type_name, mut found) in values {
            found.sort();
            found.dedup();

            for entry in found {
                report.push_str(&format!("- `{type_name}` — {entry}\n"));
            }
        }
    }

    report.push_str(
        "\nEach of these is a gap in the specification rather than breakage: repair them in the \
generator's patches, then regenerate.\n",
    );

    Ok(report)
}

/// Replaces the identifiers a call carried with the shape of the path it called.
///
/// The endpoint is recorded as it was requested, so it holds a real organization id, a real directory id and a real
/// account id. The report is written to `GITHUB_STEP_SUMMARY`, which on a public repository is public, and it is the
/// shape of the path that says which operation drifted — the identifiers say only whose data was read.
fn anonymise(endpoint: &str) -> String {
    let (method, path) = match endpoint.split_once(' ') {
        Some(halves) => halves,
        None => ("", endpoint),
    };

    let segments: Vec<&str> = path.split('/').collect();

    let shape = segments
        .iter()
        .enumerate()
        .map(|(position, segment)| {
            let previous = position.checked_sub(1).map_or("", |earlier| segments[earlier]);

            if is_identifier(segment, previous) { "{id}" } else { *segment }
        })
        .collect::<Vec<_>>()
        .join("/");

    if method.is_empty() { shape } else { format!("{method} {shape}") }
}

/// Whether a path segment names one thing rather than one kind of thing.
///
/// The `previous` segment settles the one case a shape alone cannot: `3` in `/rest/api/3/issue/10042` is the version
/// of the API and `10042` is an issue, and both are bare numbers.
fn is_identifier(segment: &str, previous: &str) -> bool {
    if segment.is_empty() {
        return false;
    }

    if segment.contains(':') {
        return true;
    }

    if segment.chars().all(|character| character.is_ascii_digit()) {
        return previous != "api";
    }

    let hexadecimal = segment.chars().filter(|character| *character != '-').count();

    segment.chars().all(|character| character.is_ascii_hexdigit() || character == '-') && hexadecimal >= 16
}

/// Collapses the index of an array element, so a finding is reported once rather than once per element.
///
/// `serde_ignored` names the position it walked through, so one undocumented field on a thousand-element list arrives
/// as a thousand findings that differ only in a number. What the report is for is the field.
fn collapse_indices(path: &str) -> String {
    path.split('.')
        .map(|segment| {
            if !segment.is_empty() && segment.chars().all(|character| character.is_ascii_digit()) {
                "*"
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_organization_id_does_not_reach_the_report() {
        assert_eq!(
            anonymise("GET /admin/v1/orgs/3cf0d0b5-eeec-4e1b-8da8-c5b47e8ae609/policies"),
            "GET /admin/v1/orgs/{id}/policies",
        );
    }

    #[test]
    fn account_ids_and_numbers_are_identifiers_too() {
        assert_eq!(anonymise("GET /admin/v2/orgs/x/users/5b6d7f20e6dba529eefdbad9"), "GET /admin/v2/orgs/x/users/{id}",);
        assert_eq!(anonymise("GET /rest/api/3/issue/10042"), "GET /rest/api/3/issue/{id}");
        assert_eq!(
            anonymise("GET /rest/api/3/user/557058:f58131cb-b67d-43c7-b30d-6b58d40bd077"),
            "GET /rest/api/3/user/{id}"
        );
    }

    #[test]
    fn the_version_of_the_api_is_not_an_identifier() {
        assert_eq!(anonymise("GET /rest/api/3/issue/10042"), "GET /rest/api/3/issue/{id}");
        assert_eq!(anonymise("GET /rest/api/2/project/10000"), "GET /rest/api/2/project/{id}");
    }

    #[test]
    fn a_word_that_happens_to_be_hexadecimal_is_left_alone() {
        assert_eq!(anonymise("GET /rest/api/3/dashboard/gadget"), "GET /rest/api/3/dashboard/gadget");
        assert_eq!(anonymise("GET /rest/api/3/field/deadbeef"), "GET /rest/api/3/field/deadbeef");
    }

    #[test]
    fn an_endpoint_without_a_method_still_normalises() {
        assert_eq!(anonymise("/admin/v1/orgs/3cf0d0b5-eeec-4e1b-8da8-c5b47e8ae609"), "/admin/v1/orgs/{id}");
    }

    #[test]
    fn every_element_of_a_list_reports_the_same_field_once() {
        assert_eq!(collapse_indices("data.?.0.attributes.groupDisplayNames"), "data.?.*.attributes.groupDisplayNames");
        assert_eq!(
            collapse_indices("data.?.1013.attributes.groupDisplayNames"),
            "data.?.*.attributes.groupDisplayNames",
        );
        assert_eq!(
            collapse_indices("data.?.relationships.domains.?.related"),
            "data.?.relationships.domains.?.related"
        );
    }
}
