// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum PolicyNotFoundErrorErrorsCode {
        Admin4045 => "ADMIN-404-5",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyNotFoundErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<PolicyNotFoundErrorErrorsCode>,
}

/// Policy not found
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyNotFoundError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<PolicyNotFoundErrorErrors>>,
}
