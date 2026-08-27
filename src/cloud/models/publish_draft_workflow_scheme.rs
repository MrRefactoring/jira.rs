// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about the status mappings for publishing a draft workflow scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublishDraftWorkflowScheme {
    /// Mappings of statuses to new statuses for issue types.
    #[serde(rename = "statusMappings", default, skip_serializing_if = "Option::is_none")]
    pub status_mappings: Option<Vec<StatusMapping>>,
}
