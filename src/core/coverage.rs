use std::collections::HashSet;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

pub const OUTPUT_VARIABLE: &str = "JIRA_COVERAGE_OUTPUT";

fn store() -> &'static Mutex<HashSet<String>> {
    static STORE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

    STORE.get_or_init(|| Mutex::new(HashSet::new()))
}

pub fn record(endpoint: &str) {
    let Ok(mut seen) = store().lock() else { return };

    if !seen.insert(endpoint.to_owned()) {
        return;
    }

    append_to_output(endpoint);
}

fn append_to_output(endpoint: &str) {
    let Ok(path) = std::env::var(OUTPUT_VARIABLE) else { return };
    let opened = std::fs::OpenOptions::new().create(true).append(true).open(path);

    if let Ok(mut file) = opened {
        let _ = writeln!(file, "{endpoint}");
    }
}
