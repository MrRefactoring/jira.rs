// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ObjectMutabilityResponseKey {
        ForbiddenFieldMutation => "forbidden.fieldMutation",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectMutabilityResponse {
    pub key: ObjectMutabilityResponseKey,
    pub context: ManageabilityRuleObjectMutability,
}
