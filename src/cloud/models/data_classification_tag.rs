// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The data classification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataClassificationTag {
    /// The color of the data classification object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The description of the data classification object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The guideline of the data classification object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guideline: Option<String>,
    /// The guideline in ADF (Atlassian Document Format) for rich text rendering.
    #[serde(rename = "guidelineADF", default, skip_serializing_if = "Option::is_none")]
    pub guideline_adf: Option<String>,
    /// The ID of the data classification object.
    pub id: String,
    /// The name of the data classification object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rank of the data classification object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// The status of the data classification object.
    pub status: String,
}
