// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GroupTypeCounts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GroupType>,
    /// The number of groups of this type in the organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
