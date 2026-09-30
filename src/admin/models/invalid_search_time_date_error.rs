// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum InvalidSearchTimeDateErrorErrorsCode {
        Admin4003 => "ADMIN-400-3",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidSearchTimeDateErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<InvalidSearchTimeDateErrorErrorsCode>,
}

/// Invalid time date
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidSearchTimeDateError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<InvalidSearchTimeDateErrorErrors>>,
}
