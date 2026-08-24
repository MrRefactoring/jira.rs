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
        .args(arguments)
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
                keys.entry(endpoint).or_default().push(if path.is_empty() { "(root)".to_owned() } else { path });
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
