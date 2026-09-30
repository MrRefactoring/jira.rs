// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of updates for a custom field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectCustomFieldValues {
    /// The list of custom field update details.
    #[serde(rename = "updateValueList", default, skip_serializing_if = "Option::is_none")]
    pub update_value_list: Option<Vec<ConnectCustomFieldValue>>,
}
