// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The status of the group assignment attempt.
    pub enum GroupAssignmentResultStatus {
        Invited => "INVITED",
        Error => "ERROR",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GroupAssignmentResult {
    /// The groupId for which the group assignment was attempted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The status of the group assignment attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GroupAssignmentResultStatus>,
    /// If status is ERROR, this field contains the reason for the failure.
    #[serde(rename = "statusReason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}
