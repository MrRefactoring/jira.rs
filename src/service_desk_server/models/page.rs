// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Page<T> {
    #[serde(rename = "_expands", default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "isLastPage", default, skip_serializing_if = "Option::is_none")]
    pub is_last_page: Option<bool>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<PagedLink>,
    pub values: Vec<T>,
    #[serde(rename = "maxResultWindow", default, skip_serializing_if = "Option::is_none")]
    pub max_result_window: Option<i64>,
}

impl<T> crate::core::Paged for Page<T> {
    type Item = T;

    fn into_step(self, requested: i64) -> crate::core::PageStep<T> {
        crate::core::PageStep::new(self.values, self.start, self.is_last_page, requested)
    }
}
