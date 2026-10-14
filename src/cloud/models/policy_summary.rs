// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicySummary {
    #[serde(rename = "appliedProjectIds", default, skip_serializing_if = "Option::is_none")]
    pub applied_project_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "policyDescription", default, skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
