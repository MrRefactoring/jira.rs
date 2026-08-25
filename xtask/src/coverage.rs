use std::path::Path;
use std::process::Command;

type Failure = Box<dyn std::error::Error>;

#[derive(Debug, serde::Deserialize)]
struct Operation {
    surface: String,
    method: String,
    path: String,
    operation: String,
}

struct Rig {
    filter: &'static str,
    surfaces: &'static [&'static str],
}

const RIGS: &[(&str, Rig)] = &[
    ("server", Rig { filter: "server::", surfaces: &["server"] }),
    ("jsm", Rig { filter: "jsm::", surfaces: &["assetsServer", "serviceDeskServer"] }),
];

pub async fn run(workspace_root: &Path, arguments: &[String]) -> Result<(), Failure> {
    let name = arguments.first().map(String::as_str).unwrap_or_default();
    let Some((_, rig)) = RIGS.iter().find(|(known, _)| *known == name) else {
        return Err(String::from(
            "Name the rig to measure: `cargo xtask coverage server` or `cargo xtask coverage jsm`. Only these two \
             are measured, because only a disposable instance may be asked for every one of its endpoints — the \
             hosted tenant the other surfaces run against is real and shared.",
        )
        .into());
    };

    let manifest = std::fs::read_to_string(workspace_root.join("operations.json")).map_err(
        |_| "No operations.json in the workspace root. It is written by the generator, so regenerate the crate.",
    )?;
    let manifest: Vec<Operation> = serde_json::from_str(&manifest)?;
    let shipped: Vec<&Operation> =
        manifest.iter().filter(|operation| rig.surfaces.contains(&operation.surface.as_str())).collect();

    if shipped.is_empty() {
        return Err(format!("The manifest holds no operations for {name}.").into());
    }

    let output = std::env::temp_dir().join(format!("jira-rs-coverage-{name}.txt"));
    let _ = std::fs::remove_file(&output);

    println!("▸ running the {name} suite with every request recorded");

    let status = Command::new("cargo")
        .current_dir(workspace_root)
        .args(["test", "--test", "live", "--all-features"])
        .args(["--", "--ignored", "--test-threads=1", rig.filter])
        .env("JIRA_COVERAGE_OUTPUT", &output)
        .status()?;

    let recorded = std::fs::read_to_string(&output).unwrap_or_default();
    let report = report(name, &shipped, &recorded);

    println!("\n{report}");

    if let Ok(summary) = std::env::var("GITHUB_STEP_SUMMARY") {
        std::fs::write(summary, &report)?;
    }

    if !status.success() {
        return Err("the suite failed; the coverage above is what it reached before stopping".into());
    }

    Ok(())
}

fn placeholders(path: &str) -> usize {
    path.split('/').filter(|segment| segment.starts_with('{')).count()
}

fn matches(template: &str, actual: &str) -> bool {
    let template: Vec<&str> = template.split('/').collect();
    let actual: Vec<&str> = actual.split('/').collect();

    template.len() == actual.len()
        && template
            .iter()
            .zip(&actual)
            .all(|(expected, found)| (expected.starts_with('{') && !found.is_empty()) || expected == found)
}

fn attribute<'a>(shipped: &[&'a Operation], call: &str) -> Option<&'a Operation> {
    let (method, path) = call.split_once(' ')?;
    let path = path.split('?').next().unwrap_or(path);

    shipped
        .iter()
        .filter(|operation| operation.method == method && matches(&operation.path, path))
        .min_by_key(|operation| placeholders(&operation.path))
        .copied()
}

fn report(name: &str, shipped: &[&Operation], recorded: &str) -> String {
    let calls: Vec<&str> = recorded.lines().filter(|line| !line.trim().is_empty()).collect();

    let mut called: Vec<&str> = Vec::new();
    let mut unattributed = 0;

    for call in &calls {
        match attribute(shipped, call) {
            Some(operation) => called.push(&operation.operation),
            None => unattributed += 1,
        }
    }

    let mut uncovered: Vec<&&Operation> =
        shipped.iter().filter(|operation| !called.contains(&operation.operation.as_str())).collect();

    uncovered.sort_by(|left, right| left.path.cmp(&right.path).then(left.method.cmp(&right.method)));

    let total = shipped.len();
    let covered = total - uncovered.len();
    let percent = covered * 100 / total;

    let mut report = format!("## Coverage — {name}\n\n{covered} of {total} operations called ({percent}%).\n");

    if unattributed > 0 {
        report.push_str(&format!(
            "\nRecorded requests that matched no operation this rig ships: {unattributed}. A request the harness builds \
             by hand looks like this, and so does a path the manifest spells differently.\n"
        ));
    }

    if uncovered.is_empty() {
        report.push_str("\nEvery operation this rig ships was called.\n");

        return report;
    }

    report.push_str("\n### Never called\n\n");

    for operation in uncovered {
        report.push_str(&format!("- `{} {}` — {}\n", operation.method, operation.path, operation.operation));
    }

    report.push_str(
        "\nEach line is either a test to write or a reason to record. The instance is disposable and the suites may \
         ruin it, so \"hard to test\" is not one.\n",
    );

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(method: &str, path: &str, name: &str) -> Operation {
        Operation {
            surface: "server".to_owned(),
            method: method.to_owned(),
            path: path.to_owned(),
            operation: name.to_owned(),
        }
    }

    #[test]
    fn a_literal_path_matches_itself() {
        assert!(matches("/rest/api/2/myself", "/rest/api/2/myself"));
        assert!(!matches("/rest/api/2/myself", "/rest/api/2/serverInfo"));
    }

    #[test]
    fn a_placeholder_stands_for_one_segment() {
        assert!(matches("/rest/api/2/issue/{key}", "/rest/api/2/issue/JRS-1"));
        assert!(!matches("/rest/api/2/issue/{key}", "/rest/api/2/issue/JRS-1/comment"));
        assert!(!matches("/rest/api/2/issue/{key}", "/rest/api/2/issue/"));
    }

    #[test]
    fn the_most_literal_template_takes_the_call() {
        let shipped = [
            operation("GET", "/rest/api/2/issue/{key}", "getIssue"),
            operation("GET", "/rest/api/2/issue/picker", "picker"),
        ];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        assert_eq!(attribute(&shipped, "GET /rest/api/2/issue/picker").unwrap().operation, "picker");
        assert_eq!(attribute(&shipped, "GET /rest/api/2/issue/JRS-1").unwrap().operation, "getIssue");
    }

    #[test]
    fn a_query_string_is_not_part_of_the_path() {
        let shipped = [operation("GET", "/rest/api/2/search", "search")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        assert_eq!(attribute(&shipped, "GET /rest/api/2/search?jql=project%3DJRS").unwrap().operation, "search");
    }

    #[test]
    fn the_method_has_to_agree() {
        let shipped = [operation("DELETE", "/rest/api/2/issue/{key}", "deleteIssue")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        assert!(attribute(&shipped, "GET /rest/api/2/issue/JRS-1").is_none());
    }

    #[test]
    fn an_uncalled_operation_is_reported_and_a_called_one_is_not() {
        let shipped = [
            operation("GET", "/rest/api/2/myself", "getMyself"),
            operation("GET", "/rest/api/2/dashboard", "getDashboards"),
        ];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let report = report("server", &shipped, "GET /rest/api/2/myself\n");

        assert!(report.contains("1 of 2 operations called (50%)"), "{report}");
        assert!(report.contains("getDashboards"), "{report}");
        assert!(!report.contains("getMyself"), "{report}");
    }

    #[test]
    fn a_request_matching_nothing_is_counted_apart() {
        let shipped = [operation("GET", "/rest/api/2/myself", "getMyself")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let report = report("server", &shipped, "GET /rest/api/2/myself\nGET /rest/api/2/somethingElse\n");

        assert!(report.contains("Every operation this rig ships was called."), "{report}");
        assert!(report.contains("matched no operation this rig ships: 1"), "{report}");
    }
}
