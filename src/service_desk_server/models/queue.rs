// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Queue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(rename = "issueCount", default, skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
}
