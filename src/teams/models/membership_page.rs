// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MembershipPage {
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub results: Vec<Membership>,
}
