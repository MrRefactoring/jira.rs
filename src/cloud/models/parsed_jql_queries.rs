// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of parsed JQL queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ParsedJqlQueries {
    /// A list of parsed JQL queries.
    pub queries: Vec<ParsedJqlQuery>,
}
