// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowPreview {
    /// The description of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "loopedTransitionContainerLayout", default, skip_serializing_if = "Option::is_none")]
    pub looped_transition_container_layout: Option<WorkflowPreviewLayout>,
    /// The name of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The project and issue type context for this workflow query.
    #[serde(rename = "queryContext", default, skip_serializing_if = "Option::is_none")]
    pub query_context: Option<Vec<ProjectIssueTypeQueryContext>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<WorkflowPreviewScope>,
    #[serde(rename = "startPointLayout", default, skip_serializing_if = "Option::is_none")]
    pub start_point_layout: Option<WorkflowPreviewLayout>,
    /// The statuses referenced in this workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<WorkflowPreviewStatus>>,
    /// The transitions of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<TransitionPreview>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<WorkflowDocumentVersion>,
}
