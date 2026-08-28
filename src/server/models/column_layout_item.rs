// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ColumnLayoutItem {
    #[serde(rename = "columnHeadingKey", default, skip_serializing_if = "Option::is_none")]
    pub column_heading_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "navigableField", default, skip_serializing_if = "Option::is_none")]
    pub navigable_field: Option<NavigableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}
