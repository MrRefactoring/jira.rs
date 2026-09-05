// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AddReactionRequest {
    #[serde(rename = "commentId", default, skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<i64>,
    #[serde(rename = "emojiId", default, skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
}
