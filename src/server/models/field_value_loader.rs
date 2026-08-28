// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldValueLoader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparator: Option<std::collections::HashMap<String, serde_json::Value>>,
}
