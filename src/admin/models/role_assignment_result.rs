// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The status of the role assignment attempt.
    pub enum RoleAssignmentResultStatus {
        Invited => "INVITED",
        Error => "ERROR",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleAssignmentResult {
    /// The resource ARI for the product which the role assignment was attempted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The role which was attempted to be assigned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The status of the role assignment attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RoleAssignmentResultStatus>,
    /// If status is ERROR, this field contains the reason for the failure.
    #[serde(rename = "statusReason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}
