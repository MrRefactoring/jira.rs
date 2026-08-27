// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The statuses stored for the specified version.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowDocumentStatusDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<WorkflowScope>,
    #[serde(rename = "statusCategory", default, skip_serializing_if = "Option::is_none")]
    pub status_category: Option<String>,
    #[serde(rename = "statusReference", default, skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
}
