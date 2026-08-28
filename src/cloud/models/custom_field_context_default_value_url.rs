// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueURLType {
    #[serde(rename = "url")]
    Url,
}

/// The default value for a URL custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueURL {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    pub r#type: CustomFieldContextDefaultValueURLType,
    /// The default URL.
    pub url: String,
}
