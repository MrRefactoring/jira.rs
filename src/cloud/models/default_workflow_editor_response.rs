// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum DefaultWorkflowEditorResponseValue {
        New => "NEW",
        Legacy => "LEGACY",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefaultWorkflowEditorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<DefaultWorkflowEditorResponseValue>,
}
