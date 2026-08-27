// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of the contextual configuration for a custom field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextualConfiguration {
    /// The field configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    /// The ID of the field context the configuration is associated with.
    #[serde(rename = "fieldContextId")]
    pub field_context_id: String,
    /// The ID of the configuration.
    pub id: String,
    /// The field value schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
}
