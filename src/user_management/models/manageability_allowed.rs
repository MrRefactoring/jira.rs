// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ManageabilityAllowedAllowed {
        True => "true",
    }
}

/// You are allowed to take or write the action/property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ManageabilityAllowed {
    pub allowed: ManageabilityAllowedAllowed,
}
