// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AdditionalComment {
    /// Content of the comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
