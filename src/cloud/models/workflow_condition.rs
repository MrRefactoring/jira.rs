// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::Serialize;

/// The workflow transition rule conditions tree.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum WorkflowCondition {
    WorkflowSimpleCondition(WorkflowSimpleCondition),
    WorkflowCompoundCondition(Box<WorkflowCompoundCondition>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(WorkflowCondition { WorkflowSimpleCondition, WorkflowCompoundCondition, Other });
