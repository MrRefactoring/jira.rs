// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Vote {
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub votes: Option<i64>,
    #[serde(rename = "hasVoted", default, skip_serializing_if = "Option::is_none")]
    pub has_voted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voters: Option<Vec<UserJson>>,
}
