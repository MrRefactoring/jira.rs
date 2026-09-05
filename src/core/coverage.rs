//! Records the endpoint of every call the process makes, which is how a live run counts what it reached.
//!
//! Only compiled under the `coverage` feature. Each endpoint is written once, as `METHOD path`, to the file the
//! environment variable names; without the variable nothing is written.

use std::collections::HashSet;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

/// The environment variable naming the file endpoints are appended to.
pub const OUTPUT_VARIABLE: &str = "JIRA_COVERAGE_OUTPUT";

fn store() -> &'static Mutex<HashSet<String>> {
    static STORE: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

    STORE.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Notes that `endpoint` was called, the first time it is.
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
