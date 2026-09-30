// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A change item.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChangeDetails {
    /// The name of the field changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The ID of the field changed.
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// The type of the field changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fieldtype: Option<String>,
    /// The details of the original value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The details of the original value as a string.
    #[serde(rename = "fromString", default, skip_serializing_if = "Option::is_none")]
    pub from_string: Option<String>,
    /// The details of the new value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The details of the new value as a string.
    #[serde(rename = "toString", default, skip_serializing_if = "Option::is_none")]
    pub to_string: Option<String>,
}
