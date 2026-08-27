// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a workflow status in preview context.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowPreviewStatus {
    #[serde(rename = "approvalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub approval_configuration: Option<ApprovalConfigurationPreview>,
    /// Whether the status is deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WorkflowPreviewLayout>,
    /// The reference of the status.
    #[serde(rename = "statusReference", default, skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
}
