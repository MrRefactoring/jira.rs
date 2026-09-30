// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Page<T> {
    #[serde(rename = "isLast")]
    pub is_last: bool,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    #[serde(rename = "startAt")]
    pub start_at: i64,
    pub total: i64,
    pub values: Vec<T>,
}

impl<T> crate::core::Paged for Page<T> {
    type Item = T;

    fn into_step(self, requested: i64) -> crate::core::PageStep<T> {
        crate::core::PageStep::new(self.values, Some(self.start_at), Some(self.is_last), requested)
    }
}
