// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Decision made by the approver.
    pub enum ApproverApproverDecision {
        Approved => "approved",
        Declined => "declined",
        Pending => "pending",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Approver {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<User>,
    /// Decision made by the approver.
    #[serde(rename = "approverDecision", default, skip_serializing_if = "Option::is_none")]
    pub approver_decision: Option<ApproverApproverDecision>,
}
