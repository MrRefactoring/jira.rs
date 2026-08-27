// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The version details of the workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowDocumentVersion {
    /// The version UUID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The version number.
    #[serde(rename = "versionNumber", default, skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}
