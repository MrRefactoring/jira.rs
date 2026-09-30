// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Page<T> {
    /// The maximum number of items a page can hold.
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    /// The index of the first item in this page.
    #[serde(rename = "startAt")]
    pub start_at: i64,
    /// The number of items across every page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Whether this is the last page.
    #[serde(rename = "isLast")]
    pub is_last: bool,
    pub values: Vec<T>,
}

impl<T> crate::core::Paged for Page<T> {
    type Item = T;

    fn into_step(self, requested: i64) -> crate::core::PageStep<T> {
        crate::core::PageStep::new(self.values, Some(self.start_at), Some(self.is_last), requested)
    }
}
