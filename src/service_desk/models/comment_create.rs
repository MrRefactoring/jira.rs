// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentCreate {
    /// Content of the comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Indicates whether the comment is public (true) or private/internal (false).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}
