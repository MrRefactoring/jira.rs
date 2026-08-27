// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupsSearchPage {
    pub data: Vec<PublicGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageModel>,
}
