// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ApprovalDecisionRequestDecision {
        Approve => "approve",
        Decline => "decline",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ApprovalDecisionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<ApprovalDecisionRequestDecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "commentPublic", default, skip_serializing_if = "Option::is_none")]
    pub comment_public: Option<bool>,
}
