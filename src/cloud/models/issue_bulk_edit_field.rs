// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum IssueBulkEditFieldMultiSelectFieldOptions {
        Add => "ADD",
        Remove => "REMOVE",
        Replace => "REPLACE",
        RemoveAll => "REMOVE_ALL",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueBulkEditField {
    /// Description of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of options related to the field, applicable in contexts where multiple selections are allowed.
    #[serde(rename = "fieldOptions", default, skip_serializing_if = "Option::is_none")]
    pub field_options: Option<Vec<IssueBulkOperationsFieldOption>>,
    /// The unique ID of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates whether the field is mandatory for the operation.
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// Specifies supported actions (like add, replace, remove) on multi-select fields via an enum.
    #[serde(rename = "multiSelectFieldOptions", default, skip_serializing_if = "Option::is_none")]
    pub multi_select_field_options: Option<Vec<IssueBulkEditFieldMultiSelectFieldOptions>>,
    /// The display name of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A URL to fetch additional data for the field
    #[serde(rename = "searchUrl", default, skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    /// The type of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A message indicating why the field is unavailable for editing.
    #[serde(rename = "unavailableMessage", default, skip_serializing_if = "Option::is_none")]
    pub unavailable_message: Option<String>,
}
