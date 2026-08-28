// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Default values to update.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueUpdate {
    #[serde(rename = "defaultValues", default, skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<CustomFieldContextDefaultValue>>,
}
