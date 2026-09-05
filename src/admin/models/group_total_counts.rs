// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Total group counts across the organization.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GroupTotalCounts {
    /// The total number of groups in the organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<i64>,
    /// The total number of groups that are synced with an identity provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced: Option<i64>,
    /// The total number of groups that are managed by an external source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<i64>,
}
