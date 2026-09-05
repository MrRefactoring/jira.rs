// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryUserSearchPage {
    /// A page of users matching the search criteria.
    pub data: Vec<MultiDirectoryUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageCursor>,
}
