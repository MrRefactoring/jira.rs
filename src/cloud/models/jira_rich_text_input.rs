// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraRichTextInput {
    #[serde(rename = "adfValue", default, skip_serializing_if = "Option::is_none")]
    pub adf_value: Option<std::collections::HashMap<String, serde_json::Value>>,
}
