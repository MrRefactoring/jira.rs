// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for creating a board column
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BoardColumnPayload {
    /// The maximum issue constraint for the column
    #[serde(rename = "maximumIssueConstraint", default, skip_serializing_if = "Option::is_none")]
    pub maximum_issue_constraint: Option<i64>,
    /// The minimum issue constraint for the column
    #[serde(rename = "minimumIssueConstraint", default, skip_serializing_if = "Option::is_none")]
    pub minimum_issue_constraint: Option<i64>,
    /// The name of the column
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status IDs for the column
    #[serde(rename = "statusIds", default, skip_serializing_if = "Option::is_none")]
    pub status_ids: Option<Vec<ProjectCreateResourceIdentifier>>,
}
