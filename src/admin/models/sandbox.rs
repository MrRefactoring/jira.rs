// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum SandboxType {
        Child => "CHILD",
        None => "NONE",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sandbox {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SandboxType>,
}
