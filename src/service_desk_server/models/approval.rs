// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ApprovalFinalDecision {
        Approved => "approved",
        Declined => "declined",
        Pending => "pending",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Approval {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "finalDecision", default, skip_serializing_if = "Option::is_none")]
    pub final_decision: Option<ApprovalFinalDecision>,
    #[serde(rename = "canAnswerApproval", default, skip_serializing_if = "Option::is_none")]
    pub can_answer_approval: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<Approver>>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<Date>,
    #[serde(rename = "completedDate", default, skip_serializing_if = "Option::is_none")]
    pub completed_date: Option<Date>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApprovalCondition>,
}
