use std::collections::{BTreeMap, BTreeSet};
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
    let ledger = read_ledger(&ledger_path(workspace_root, name))?;
    let called = called_operations(&shipped, &recorded);
    let report = report(name, &shipped, &called, &ledger);

    println!("\n{report}");

    if let Ok(summary) = std::env::var("GITHUB_STEP_SUMMARY") {
        std::fs::write(summary, &report)?;
    }

    if !status.success() {
        return Err("the suite failed; the coverage above is what it reached before stopping".into());
    }

    hold_the_ratchet(name, &shipped, &called, &ledger)
}

fn ledger_path(workspace_root: &Path, rig: &str) -> std::path::PathBuf {
    workspace_root.join("tests/live/uncovered").join(format!("{rig}.txt"))
}

fn hold_the_ratchet(
    rig: &str,
    shipped: &[&Operation],
    called: &BTreeSet<String>,
    ledger: &Ledger,
) -> Result<(), Failure> {
    let key = |operation: &Operation| format!("{} {}", operation.method, operation.path);
    let known: BTreeSet<String> = shipped.iter().map(|operation| key(operation)).collect();

    let unaccounted: Vec<String> = shipped
        .iter()
        .filter(|operation| !called.contains(&operation.operation) && !ledger.holds(&key(operation)))
        .map(|operation| format!("  {} — {}", key(operation), operation.operation))
        .collect();

    let stale: Vec<String> = shipped
        .iter()
        .filter(|operation| called.contains(&operation.operation) && ledger.holds(&key(operation)))
        .map(|operation| format!("  {}", key(operation)))
        .collect();

    let unknown: Vec<String> =
        ledger.keys().filter(|listed| !known.contains(*listed)).map(|listed| format!("  {listed}")).collect();

    let mut refusal = String::new();

    if !unaccounted.is_empty() {
        refusal.push_str(&format!(
            "Operations this rig ships that no suite calls and {}.txt does not record: {}. Write a test, or record \
             the operation as `untested` while it waits for one, and as `unreachable` with a reason where this \
             instance cannot answer it at all.\n{}\n\n",
            rig,
            unaccounted.len(),
            unaccounted.join("\n"),
        ));
    }

    if !stale.is_empty() {
        refusal.push_str(&format!(
            "Operations recorded in {}.txt that a suite called anyway: {}. Remove these lines — a ledger that names \
             what is already covered stops saying anything.\n{}\n\n",
            rig,
            stale.len(),
            stale.join("\n"),
        ));
    }

    if !unknown.is_empty() {
        refusal.push_str(&format!(
            "Operations recorded in {}.txt that this rig does not ship: {}. A regenerated surface renamed or \
             dropped them.\n{}\n",
            rig,
            unknown.len(),
            unknown.join("\n"),
        ));
    }

    if refusal.is_empty() { Ok(()) } else { Err(refusal.into()) }
}

fn called_operations(shipped: &[&Operation], recorded: &str) -> BTreeSet<String> {
    recorded
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|call| attribute(shipped, call))
        .map(|operation| operation.operation.clone())
        .collect()
}

#[derive(Default)]
struct Ledger {
    unreachable: BTreeMap<String, String>,
    untested: BTreeSet<String>,
}

impl Ledger {
    fn holds(&self, key: &str) -> bool {
        self.unreachable.contains_key(key) || self.untested.contains(key)
    }

    fn keys(&self) -> impl Iterator<Item = &String> {
        self.unreachable.keys().chain(self.untested.iter())
    }
}

fn read_ledger(path: &Path) -> Result<Ledger, Failure> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Ok(Ledger::default());
    };

    let mut ledger = Ledger::default();

    for (number, line) in contents.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let at = format!("{}:{}", path.display(), number + 1);
        let (kind, rest) = line.split_once(' ').ok_or_else(|| format!("{at}: a line names a kind and an operation"))?;
        let (operation, reason) = match rest.split_once(" — ") {
            Some((operation, reason)) => (operation.trim(), Some(reason.trim())),
            None => (rest.trim(), None),
        };

        match kind {
            "unreachable" => {
                let reason = reason.ok_or_else(|| {
                    format!(
                        "{at}: an unreachable operation records why this instance cannot answer it, after an em \
                         dash. \"Hard to test\" is not a reason — the rig is disposable and the suites may ruin it."
                    )
                })?;

                ledger.unreachable.insert(operation.to_owned(), reason.to_owned());
            }
            "untested" => {
                ledger.untested.insert(operation.to_owned());
            }
            other => return Err(format!("{at}: \"{other}\" is neither `unreachable` nor `untested`.").into()),
        }
    }

    Ok(ledger)
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

fn report(rig: &str, shipped: &[&Operation], called: &BTreeSet<String>, ledger: &Ledger) -> String {
    let mut uncovered: Vec<&&Operation> =
        shipped.iter().filter(|operation| !called.contains(&operation.operation)).collect();

    uncovered.sort_by(|left, right| left.path.cmp(&right.path).then(left.method.cmp(&right.method)));

    let total = shipped.len();
    let covered = total - uncovered.len();
    let percent = covered * 100 / total;

    let mut report = format!("## Coverage — {rig}\n\n{covered} of {total} operations called ({percent}%).\n");

    if uncovered.is_empty() {
        report.push_str("\nEvery operation this rig ships was called.\n");

        return report;
    }

    report.push_str("\n### Not called\n\n");

    for operation in uncovered {
        let key = format!("{} {}", operation.method, operation.path);
        let note = match ledger.unreachable.get(&key) {
            Some(reason) => format!(" — unreachable: {reason}"),
            None if ledger.untested.contains(&key) => " — untested".to_owned(),
            None => " — neither called nor recorded".to_owned(),
        };

        report.push_str(&format!("- `{key}` — {}{note}\n", operation.operation));
    }

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

        let called = called_operations(&shipped, "GET /rest/api/2/myself\n");
        let report = report("server", &shipped, &called, &Ledger::default());

        assert!(report.contains("1 of 2 operations called (50%)"), "{report}");
        assert!(report.contains("getDashboards"), "{report}");
        assert!(report.contains("neither called nor recorded"), "{report}");
    }

    #[test]
    fn a_request_matching_nothing_is_counted_apart() {
        let shipped = [operation("GET", "/rest/api/2/myself", "getMyself")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let called = called_operations(&shipped, "GET /rest/api/2/myself\nGET /rest/api/2/somethingElse\n");
        let report = report("server", &shipped, &called, &Ledger::default());

        assert!(report.contains("Every operation this rig ships was called."), "{report}");
    }

    #[test]
    fn an_operation_neither_called_nor_recorded_holds_the_ratchet() {
        let shipped = [operation("GET", "/rest/api/2/myself", "getMyself")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let refused = hold_the_ratchet("server", &shipped, &BTreeSet::new(), &Ledger::default())
            .expect_err("an operation nobody calls and nobody recorded fails the run");

        assert!(refused.to_string().contains("does not record: 1"), "{refused}");
    }

    #[test]
    fn a_recorded_operation_lets_the_run_pass() {
        let shipped = [operation("GET", "/rest/api/2/myself", "getMyself")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let mut ledger = Ledger::default();
        ledger.untested.insert("GET /rest/api/2/myself".to_owned());

        assert!(hold_the_ratchet("server", &shipped, &BTreeSet::new(), &ledger).is_ok());
    }

    #[test]
    fn a_recorded_operation_that_is_called_after_all_holds_the_ratchet() {
        let shipped = [operation("GET", "/rest/api/2/myself", "getMyself")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let mut ledger = Ledger::default();
        ledger.untested.insert("GET /rest/api/2/myself".to_owned());

        let called: BTreeSet<String> = ["getMyself".to_owned()].into_iter().collect();
        let refused = hold_the_ratchet("server", &shipped, &called, &ledger)
            .expect_err("a ledger that names what is covered says nothing");

        assert!(refused.to_string().contains("recorded"), "{refused}");
    }

    #[test]
    fn an_operation_the_rig_no_longer_ships_holds_the_ratchet() {
        let shipped = [operation("GET", "/rest/api/2/myself", "getMyself")];
        let shipped: Vec<&Operation> = shipped.iter().collect();

        let mut ledger = Ledger::default();
        ledger.untested.insert("GET /rest/api/2/gone".to_owned());

        let called: BTreeSet<String> = ["getMyself".to_owned()].into_iter().collect();
        let refused = hold_the_ratchet("server", &shipped, &called, &ledger)
            .expect_err("a line naming an operation the rig does not ship is out of step");

        assert!(refused.to_string().contains("does not ship: 1"), "{refused}");
    }
}
