// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ManageabilityUnallowedAllowed {
        False => "false",
    }
}

/// You are not allowed to take or write the action/property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ManageabilityUnallowed {
    pub allowed: ManageabilityUnallowedAllowed,
    pub reason: ManageabilityRestrictionReason,
}
