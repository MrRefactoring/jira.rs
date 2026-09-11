// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum InternalErrorErrorsCode {
        Admin5001 => "ADMIN-500-1",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InternalErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<InternalErrorErrorsCode>,
}

/// Internal error
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InternalError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<InternalErrorErrors>>,
}
