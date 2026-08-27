// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The statuses referenced in the workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowReferenceStatus {
    #[serde(rename = "approvalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub approval_configuration: Option<ApprovalConfiguration>,
    /// Indicates if the status is deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WorkflowStatusLayout>,
    /// The properties associated with the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The reference of the status.
    #[serde(rename = "statusReference", default, skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
}
