// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A page of workflows.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusWorkflowUsagePage {
    /// Page token for the next page of issue type usages.
    #[serde(rename = "nextPageToken", default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of statuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<StatusWorkflowUsageWorkflow>>,
}
