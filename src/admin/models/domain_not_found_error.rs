// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum DomainNotFoundErrorErrorsCode {
        Admin4043 => "ADMIN-404-3",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DomainNotFoundErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<DomainNotFoundErrorErrorsCode>,
}

/// Domain not found
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DomainNotFoundError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<DomainNotFoundErrorErrors>>,
}
