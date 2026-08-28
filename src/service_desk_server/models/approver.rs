// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ApproverApproverDecision {
        Approved => "approved",
        Declined => "declined",
        Pending => "pending",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Approver {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<User>,
    #[serde(rename = "approverDecision", default, skip_serializing_if = "Option::is_none")]
    pub approver_decision: Option<ApproverApproverDecision>,
}
