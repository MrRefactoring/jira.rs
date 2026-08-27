// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of names changed in the record event.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangedValue {
    /// The value of the field before the change.
    #[serde(rename = "changedFrom", default, skip_serializing_if = "Option::is_none")]
    pub changed_from: Option<String>,
    /// The value of the field after the change.
    #[serde(rename = "changedTo", default, skip_serializing_if = "Option::is_none")]
    pub changed_to: Option<String>,
    /// The name of the field changed.
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
}
