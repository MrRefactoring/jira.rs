// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Queue {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
    /// Fields returned for each request in the queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// ID for the queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The count of customer requests in the queue.
    #[serde(rename = "issueCount", default, skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
    /// JQL query that filters reqeusts for the queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// Short name for the queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
