// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Workflow {
    /// The creation date of the workflow.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The creation date of the workflow.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// The description of the workflow.
    pub description: String,
    /// Whether the workflow has a draft version.
    #[serde(rename = "hasDraftWorkflow", default, skip_serializing_if = "Option::is_none")]
    pub has_draft_workflow: Option<bool>,
    pub id: PublishedWorkflowId,
    /// Whether this is the default workflow.
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<WorkflowOperations>,
    /// The projects the workflow is assigned to, through workflow schemes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectDetails>>,
    /// The workflow schemes the workflow is assigned to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<WorkflowSchemeIdName>>,
    /// The statuses of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<WorkflowStatus>>,
    /// The transitions of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<Transition>>,
    /// The last edited date of the workflow.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The last edited date of the workflow.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
}
