// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Default value updates grouped by context and issue type.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValuesUpdate {
    /// The default values to update.
    #[serde(rename = "defaultValues", default, skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<IssueTypeDefaultValueUpdate>>,
}
