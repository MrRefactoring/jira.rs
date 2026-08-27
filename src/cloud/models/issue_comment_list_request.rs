// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueCommentListRequest {
    /// The list of comment IDs. A maximum of 1000 IDs can be specified.
    pub ids: Vec<i64>,
}
