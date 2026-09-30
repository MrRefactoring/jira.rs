// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DomainPage {
    /// 0 or more values of Domain are returned
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DomainModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageModel>,
}
