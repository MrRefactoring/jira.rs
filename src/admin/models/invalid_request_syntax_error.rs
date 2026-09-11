// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum InvalidRequestSyntaxErrorErrorsCode {
        Admin40019 => "ADMIN-400-19",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidRequestSyntaxErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<InvalidRequestSyntaxErrorErrorsCode>,
}

/// Request syntax is not valid
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidRequestSyntaxError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<InvalidRequestSyntaxErrorErrors>>,
}
