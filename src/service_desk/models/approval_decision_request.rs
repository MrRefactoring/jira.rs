// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Response to the approval request.
    pub enum ApprovalDecisionRequestDecision {
        Approve => "approve",
        Decline => "decline",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ApprovalDecisionRequest {
    /// Response to the approval request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<ApprovalDecisionRequestDecision>,
}
