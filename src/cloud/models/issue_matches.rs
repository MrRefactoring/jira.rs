// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of matched issues or errors for each JQL query, in the order the JQL queries were passed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueMatches {
    pub matches: Vec<IssueMatchesForJQL>,
}
