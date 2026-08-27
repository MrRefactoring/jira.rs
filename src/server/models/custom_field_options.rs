// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
