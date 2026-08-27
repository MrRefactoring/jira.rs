// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleListWrapperGroupJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackGroupJson>,
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "pagingCallback", default, skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackGroupJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
