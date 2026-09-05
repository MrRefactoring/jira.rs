// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a user or group subscribing to a filter.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FilterSubscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupName>,
    /// The ID of the filter subscription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<DashboardUser>,
}
