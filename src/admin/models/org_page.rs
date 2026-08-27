// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrgPage {
    /// 0 or more values of `Org` are returned
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrgModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}
