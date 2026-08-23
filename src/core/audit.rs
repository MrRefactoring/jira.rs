//! Collects the gaps between the generated types and what the API actually sends.
//!
//! Only compiled under the `audit` feature, which the crate's own audit run enables and nothing else does.
//!
//! Two kinds of gap are recorded, and they are repaired in different places. An undocumented key is a field the
//! specification never described, and it is fixed by adding the field to the schema the type is generated from. A
//! value outside a documented set is the same gap one level down: the field is described, its list of values is not
//! complete.

use std::io::Write;
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

/// Where each finding is appended as it is made, when the audit run names a file.
///
/// Written through rather than reported at the end: a test binary has no teardown hook that runs after the last
/// test, and a run that panics half way through still has findings worth keeping.
pub const OUTPUT_VARIABLE: &str = "JIRA_AUDIT_OUTPUT";

/// One gap between a generated type and the response it was deserialized from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SchemaDrift {
    /// Keys the API sends that the type does not describe.
    UndocumentedKeys {
        /// The request that produced the response, e.g. `GET /rest/api/3/myself`.
        endpoint: String,
        /// Where in the response body it turned up. Empty for the top level.
        path: String,
    },
    /// A value outside the set an open enum lists.
    UndocumentedValue {
        /// The generated enum that met it.
        type_name: String,
        /// The value the API sent.
        value: String,
        /// The values the specification lists.
        documented: Vec<String>,
    },
}

fn store() -> &'static Mutex<Vec<SchemaDrift>> {
    static STORE: OnceLock<Mutex<Vec<SchemaDrift>>> = OnceLock::new();

    STORE.get_or_init(|| Mutex::new(Vec::new()))
}

fn record(entry: SchemaDrift) {
    let Ok(mut collected) = store().lock() else { return };

    if collected.contains(&entry) {
        return;
    }

    append_to_output(&entry);
    collected.push(entry);
}

fn append_to_output(entry: &SchemaDrift) {
    let Ok(path) = std::env::var(OUTPUT_VARIABLE) else {
        return;
    };
    let Ok(line) = serde_json::to_string(entry) else { return };
    let opened = std::fs::OpenOptions::new().create(true).append(true).open(path);

    if let Ok(mut file) = opened {
        let _ = writeln!(file, "{line}");
    }
}

pub fn record_undocumented_key(endpoint: &str, path: &str) {
    record(SchemaDrift::UndocumentedKeys {
        endpoint: endpoint.to_owned(),
        path: path.to_owned(),
    });
}

pub fn record_undocumented_value(type_name: &str, value: &str, documented: &[&str]) {
    record(SchemaDrift::UndocumentedValue {
        type_name: type_name.to_owned(),
        value: value.to_owned(),
        documented: documented.iter().map(|value| (*value).to_owned()).collect(),
    });
}

/// Everything recorded so far.
pub fn collected() -> Vec<SchemaDrift> {
    store().lock().map(|collected| collected.clone()).unwrap_or_default()
}

/// Forgets everything recorded so far. For tests, which would otherwise leak findings between cases.
pub fn reset() {
    if let Ok(mut collected) = store().lock() {
        collected.clear();
    }
}
