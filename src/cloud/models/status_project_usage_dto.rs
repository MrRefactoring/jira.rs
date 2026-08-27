// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The projects using this status.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusProjectUsageDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<StatusProjectUsagePage>,
    /// The status ID.
    #[serde(rename = "statusId", default, skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
}
