// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum TargetUnverifiedResponseKey {
        ForbiddenTargetUnverified => "forbidden.targetUnverified",
    }
}

/// Cannot manage an unverified target account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TargetUnverifiedResponse {
    pub key: TargetUnverifiedResponseKey,
}
