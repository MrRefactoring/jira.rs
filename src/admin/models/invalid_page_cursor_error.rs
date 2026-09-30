// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum InvalidPageCursorErrorErrorsCode {
        Admin4001 => "ADMIN-400-1",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidPageCursorErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<InvalidPageCursorErrorErrorsCode>,
}

/// Invalid page cursor
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidPageCursorError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<InvalidPageCursorErrorErrors>>,
}
