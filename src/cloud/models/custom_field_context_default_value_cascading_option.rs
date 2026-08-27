// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueCascadingOptionType {
    #[serde(rename = "option.cascading")]
    OptionCascading,
}

/// The default value for a cascading select custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueCascadingOption {
    /// The ID of the default cascading option.
    #[serde(rename = "cascadingOptionId", default, skip_serializing_if = "Option::is_none")]
    pub cascading_option_id: Option<String>,
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default option.
    #[serde(rename = "optionId")]
    pub option_id: String,
    pub r#type: CustomFieldContextDefaultValueCascadingOptionType,
}
