// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum InvalidQueryCountErrorErrorsCode {
        Admin40021 => "ADMIN-400-21",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidQueryCountErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<InvalidQueryCountErrorErrorsCode>,
}

/// The number of queries exceeded the limit
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidQueryCountError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<InvalidQueryCountErrorErrors>>,
}
