// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Contains details about a version approver.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionApprover {
    /// The Atlassian account ID of the approver.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A description of why the user is declining the approval.
    #[serde(rename = "declineReason", default, skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<String>,
    /// A description of what the user is approving within the specified version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The status of the approval, which can be *PENDING*, *APPROVED*, or *DECLINED*
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
