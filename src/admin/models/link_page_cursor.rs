// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Links for a paginated response, for use in a cursor parameter.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPageCursor {
    /// Cursor to fetch this page.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Cursor to fetch the previous page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// Cursor to fetch the next page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}
