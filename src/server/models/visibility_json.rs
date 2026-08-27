// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum VisibilityJsonType {
        Group => "group",
        Role => "role",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisibilityJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<VisibilityJsonType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
