// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of JQL queries to sanitize for the given account IDs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlQueriesToSanitize {
    /// The list of JQL queries to sanitize. Must contain unique values. Maximum of 20 queries.
    pub queries: Vec<JqlQueryToSanitize>,
}
