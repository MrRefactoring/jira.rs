// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Links for a resources with self and alternate links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkSelfAltModel {
    /// URL to fetch this resource
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Alternate URL to fetch this resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}
