// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JiraWorkflow {
    /// The creation date of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The description of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates if the workflow can be edited.
    #[serde(rename = "isEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    #[serde(rename = "loopedTransitionContainerLayout", default, skip_serializing_if = "Option::is_none")]
    pub looped_transition_container_layout: Option<WorkflowLayout>,
    /// The name of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<WorkflowScope>,
    #[serde(rename = "startPointLayout", default, skip_serializing_if = "Option::is_none")]
    pub start_point_layout: Option<WorkflowLayout>,
    /// The statuses referenced in this workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<WorkflowReferenceStatus>>,
    /// If there is a current [asynchronous task](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#async-operations) operation for this workflow.
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// The transitions of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<WorkflowTransitions>>,
    /// The last edited date of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<DocumentVersion>,
}
