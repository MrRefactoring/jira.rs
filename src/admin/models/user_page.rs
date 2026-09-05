// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserPageMeta {
    /// Total number of users in this Query
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserPage {
    /// 0 or more values of Users are returned
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<User>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<UserPageMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageModel>,
}
