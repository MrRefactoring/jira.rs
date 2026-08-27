// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Fields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuetype: Option<IssueTypeJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
