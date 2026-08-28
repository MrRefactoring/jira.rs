// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueMultipleOptionType {
    #[serde(rename = "option.multiple")]
    OptionMultiple,
}

/// The default value for a multi-select custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueMultipleOption {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The list of IDs of the default options.
    #[serde(rename = "optionIds")]
    pub option_ids: Vec<String>,
    pub r#type: CustomFieldContextDefaultValueMultipleOptionType,
}
