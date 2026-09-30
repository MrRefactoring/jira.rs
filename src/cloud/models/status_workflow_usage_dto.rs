// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Workflows using the status.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusWorkflowUsageDTO {
    /// The status ID.
    #[serde(rename = "statusId", default, skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<StatusWorkflowUsagePage>,
}
