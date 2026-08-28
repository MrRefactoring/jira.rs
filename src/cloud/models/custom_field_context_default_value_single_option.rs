// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueSingleOptionType {
    #[serde(rename = "option.single")]
    OptionSingle,
}

/// The default value for a single select custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueSingleOption {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default option.
    #[serde(rename = "optionId")]
    pub option_id: String,
    pub r#type: CustomFieldContextDefaultValueSingleOptionType,
}
