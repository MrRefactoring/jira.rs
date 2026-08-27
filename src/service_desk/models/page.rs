// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(rename = "_expands", default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<Vec<String>>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<PagedLink>,
    /// Indicates if this is the last page of records (true) or not (false).
    #[serde(rename = "isLastPage", default, skip_serializing_if = "Option::is_none")]
    pub is_last_page: Option<bool>,
    /// Number of items to be returned per page, up to the maximum set for these objects in the current implementation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items returned in the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Index of the first item returned in the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    /// Details of the items included in the page.
    pub values: Vec<T>,
}
