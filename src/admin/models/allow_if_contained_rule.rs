// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllowIfContainedRuleNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

/// Applicable when policy type is `ip-allowlist` or `data-residency`
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AllowIfContainedRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#in: Option<Vec<String>>,
    #[serde(rename = "adminApprovalSummary", default, skip_serializing_if = "Option::is_none")]
    pub admin_approval_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consented: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification: Option<Vec<AllowIfContainedRuleNotification>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off: Option<Vec<String>>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
}
