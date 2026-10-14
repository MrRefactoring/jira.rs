// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatePolicyDocument {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "policyDescription", default, skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "rulesByDomain", default, skip_serializing_if = "Option::is_none")]
    pub rules_by_domain: Option<std::collections::HashMap<String, Vec<StatePolicyRule>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
