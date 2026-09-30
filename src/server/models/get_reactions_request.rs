// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetReactionsRequest {
    #[serde(rename = "commentIds", default, skip_serializing_if = "Option::is_none")]
    pub comment_ids: Option<Vec<i64>>,
}
