// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApprovalCommentConfig {
    #[serde(rename = "commentsRequiredWhenApprove", default, skip_serializing_if = "Option::is_none")]
    pub comments_required_when_approve: Option<bool>,
    #[serde(rename = "commentsRequiredWhenDecline", default, skip_serializing_if = "Option::is_none")]
    pub comments_required_when_decline: Option<bool>,
}
