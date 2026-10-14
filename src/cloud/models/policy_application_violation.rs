// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyApplicationViolation {
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "entityName", default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "remediationHint", default, skip_serializing_if = "Option::is_none")]
    pub remediation_hint: Option<String>,
    #[serde(rename = "ruleTypeId", default, skip_serializing_if = "Option::is_none")]
    pub rule_type_id: Option<String>,
}
