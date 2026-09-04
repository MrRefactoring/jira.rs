// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// How far the work has come, as time logged against time estimated.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueProgress {
    /// Seconds logged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// Seconds logged plus seconds still estimated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Logged as a share of the total, 0 to 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
}
