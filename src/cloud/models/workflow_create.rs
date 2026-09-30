// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The details of the workflows to create.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowCreate {
    /// The description of the workflow to create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "loopedTransitionContainerLayout", default, skip_serializing_if = "Option::is_none")]
    pub looped_transition_container_layout: Option<WorkflowLayout>,
    /// The name of the workflow to create.
    pub name: String,
    #[serde(rename = "startPointLayout", default, skip_serializing_if = "Option::is_none")]
    pub start_point_layout: Option<WorkflowLayout>,
    /// The statuses associated with this workflow.
    pub statuses: Vec<StatusLayoutUpdate>,
    /// The transitions of this workflow.
    pub transitions: Vec<TransitionUpdateDTO>,
}
