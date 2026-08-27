// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A list of JQL queries to parse.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JqlQueriesToParse {
    /// A list of queries to parse.
    pub queries: Vec<String>,
}
