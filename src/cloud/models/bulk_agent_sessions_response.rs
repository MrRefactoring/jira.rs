// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Session-start outcomes, not completion of the agent's work.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkAgentSessionsResponse {
    /// Number of ALREADY\_STARTED items.
    #[serde(rename = "alreadyStartedCount", default, skip_serializing_if = "Option::is_none")]
    pub already_started_count: Option<i64>,
    /// Number of FAILED and IN\_DOUBT items.
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    /// Number of RATE\_LIMITED items.
    #[serde(rename = "rateLimitedCount", default, skip_serializing_if = "Option::is_none")]
    pub rate_limited_count: Option<i64>,
    /// One outcome per requested item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BulkAgentSessionOutcome>>,
    /// Number of STARTED items.
    #[serde(rename = "startedCount", default, skip_serializing_if = "Option::is_none")]
    pub started_count: Option<i64>,
    /// True when startedCount or alreadyStartedCount is greater than zero. Does not indicate that every item succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
