// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetWorkflow {
    One(WorkflowMapping),
    Many(Vec<WorkflowMapping>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}
