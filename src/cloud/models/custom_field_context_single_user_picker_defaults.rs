// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextSingleUserPickerDefaultsType {
    #[serde(rename = "single.user.select")]
    SingleUserSelect,
}

/// Defaults for a User Picker (single) custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextSingleUserPickerDefaults {
    /// The ID of the default user.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    pub r#type: CustomFieldContextSingleUserPickerDefaultsType,
    #[serde(rename = "userFilter")]
    pub user_filter: UserFilter,
}
