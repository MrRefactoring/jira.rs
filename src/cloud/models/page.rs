// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A page of items.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Page<T> {
    /// Whether this is the last page.
    #[serde(rename = "isLast")]
    pub is_last: bool,
    /// The maximum number of items that could be returned.
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    /// If there is another page of results, the URL of the next page.
    #[serde(rename = "nextPage", default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    /// The URL of the page.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The index of the first item returned.
    #[serde(rename = "startAt")]
    pub start_at: i64,
    /// The number of items returned.
    pub total: i64,
    /// The list of items.
    pub values: Vec<T>,
}

impl<T> crate::core::Paged for Page<T> {
    type Item = T;

    fn into_step(self, requested: i64) -> crate::core::PageStep<T> {
        crate::core::PageStep::new(self.values, Some(self.start_at), Some(self.is_last), requested)
    }
}
