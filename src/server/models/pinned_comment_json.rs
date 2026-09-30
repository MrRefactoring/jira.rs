// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PinnedCommentJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentJson>,
    #[serde(rename = "pinnedBy", default, skip_serializing_if = "Option::is_none")]
    pub pinned_by: Option<String>,
    #[serde(rename = "pinnedDate", default, skip_serializing_if = "Option::is_none")]
    pub pinned_date: Option<String>,
}
