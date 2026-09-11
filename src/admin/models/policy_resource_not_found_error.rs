// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum PolicyResourceNotFoundErrorErrorsCode {
        Admin4046 => "ADMIN-404-6",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyResourceNotFoundErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<PolicyResourceNotFoundErrorErrorsCode>,
}

/// Policy Resource not found
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyResourceNotFoundError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PolicyResourceNotFoundErrorErrors>>,
}
