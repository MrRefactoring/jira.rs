// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Links for a Paginated response
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPageModel {
    /// URL to fetch this Page
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// URL to fetch the Previous Page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// URL to fetch the Next Page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}
