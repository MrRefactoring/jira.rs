// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Cursor pagination result for PublicApiTeam
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TeamPaginationResult {
    /// The cursor token for the next page of results. Continue querying with the returned cursor until it is null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The list of teams
    pub entities: Vec<Team>,
}
