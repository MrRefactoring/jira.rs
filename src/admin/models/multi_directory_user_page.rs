// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryUserPage {
    /// A page of users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MultiDirectoryUser>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageCursor>,
}
