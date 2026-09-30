// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A page of issue types.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusProjectIssueTypeUsagePage {
    /// Page token for the next page of issue type usages.
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of issue types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<StatusProjectIssueTypeUsage>>,
}
