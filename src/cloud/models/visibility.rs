// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Whether visibility of this item is restricted to a group or role.
    pub enum VisibilityType {
        Group => "group",
        Role => "role",
    }
}

/// The group or role to which this item is visible.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Visibility {
    /// The ID of the group or the name of the role that visibility of this item is restricted to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Whether visibility of this item is restricted to a group or role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<VisibilityType>,
    /// The name of the group or role that visibility of this item is restricted to. Please note that the name of a group is mutable, to reliably identify a group use `identifier`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
