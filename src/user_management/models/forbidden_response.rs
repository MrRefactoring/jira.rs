// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ForbiddenResponseKey {
        Forbidden => "forbidden",
    }
}

/// You are not authorized to access this resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenResponse {
    pub key: ForbiddenResponseKey,
}
