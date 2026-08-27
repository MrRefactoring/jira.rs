// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollingEventPageMeta {
    /// Value for the next cursor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// Number of items on a page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollingEventPage {
    /// 0 or more values of Event are returned
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PollingEventModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PollingEventPageMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageModel>,
}
