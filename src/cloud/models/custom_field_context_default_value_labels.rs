// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueLabelsType {
    #[serde(rename = "labels")]
    Labels,
}

/// Default value for a labels custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueLabels {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default labels value.
    pub labels: Vec<String>,
    pub r#type: CustomFieldContextDefaultValueLabelsType,
}
