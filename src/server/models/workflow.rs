// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Workflow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The number of steps in the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(rename = "lastModifiedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "lastModifiedUser", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
}
