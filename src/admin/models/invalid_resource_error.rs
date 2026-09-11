// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum InvalidResourceErrorErrorsCode {
        Admin4004 => "ADMIN-400-4",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidResourceErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<InvalidResourceErrorErrorsCode>,
}

/// Resource is not valid
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidResourceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<InvalidResourceErrorErrors>>,
}
