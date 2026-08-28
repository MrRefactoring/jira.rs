// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PageDataResponseV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<WorkspaceModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<MetaV2>,
}
