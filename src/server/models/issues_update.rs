// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssuesUpdate {
    #[serde(rename = "issueUpdates", default, skip_serializing_if = "Option::is_none")]
    pub issue_updates: Option<Vec<IssueUpdate>>,
}
