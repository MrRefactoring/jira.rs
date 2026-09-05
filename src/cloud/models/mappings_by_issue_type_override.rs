// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The mappings for migrating issues from old statuses to new statuses when switching from one workflow scheme to another. This field is required if any statuses in the current project's workflows would no longer exist in the target workflow scheme. Each mapping defines how to update issues from an old status to the corresponding new status in the issue’s new workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MappingsByIssueTypeOverride {
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    #[serde(rename = "statusMappings", default, skip_serializing_if = "Option::is_none")]
    pub status_mappings: Option<Vec<WorkflowAssociationStatusMapping>>,
}
