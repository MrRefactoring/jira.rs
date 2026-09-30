// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of updates for a custom fields.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MultipleCustomFieldValuesUpdateDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<MultipleCustomFieldValuesUpdate>>,
}
