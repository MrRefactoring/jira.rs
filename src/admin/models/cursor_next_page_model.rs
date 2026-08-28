// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Cursors for REST API pagination
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CursorNextPageModel {
    /// Cursor to fetch next page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}
