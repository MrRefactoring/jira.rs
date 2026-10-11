// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum VisibilityType {
        Group => "group",
        Role => "role",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Visibility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<VisibilityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
