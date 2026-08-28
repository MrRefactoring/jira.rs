// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "statusCategory", default, skip_serializing_if = "Option::is_none")]
    pub status_category: Option<StatusCategoryJson>,
    #[serde(rename = "statusColor", default, skip_serializing_if = "Option::is_none")]
    pub status_color: Option<String>,
}
