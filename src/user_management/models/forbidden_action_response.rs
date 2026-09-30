// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ForbiddenActionResponseKey {
        ForbiddenAction => "forbidden.action",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForbiddenActionResponse {
    pub key: ForbiddenActionResponseKey,
    pub context: ManageabilityUnallowed,
}
