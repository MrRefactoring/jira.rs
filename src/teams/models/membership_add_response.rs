// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MembershipAddResponse {
    pub errors: Vec<MembershipCodedError>,
    pub members: Vec<Membership>,
}
