// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PagedListWrapperUserJsonApplicationUser {
    #[serde(rename = "backingListSize", default, skip_serializing_if = "Option::is_none")]
    pub backing_list_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackUserJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UserJson>>,
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "pagingCallback", default, skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackUserJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
