// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum CdenPolicyValidationFailedErrorErrorsCode {
        Admin5002 => "ADMIN-500-2",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CdenPolicyValidationFailedErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<CdenPolicyValidationFailedErrorErrorsCode>,
}

/// CDEN policy validation failed
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CdenPolicyValidationFailedError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CdenPolicyValidationFailedErrorErrors>>,
}
