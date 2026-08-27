// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReportsForBoard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}
