// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum OrgNotFoundErrorErrorsCode {
        Admin4042 => "ADMIN-404-2",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgNotFoundErrorErrors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<OrgNotFoundErrorErrorsCode>,
}

/// Organization not found
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrgNotFoundError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<OrgNotFoundErrorErrors>>,
}
