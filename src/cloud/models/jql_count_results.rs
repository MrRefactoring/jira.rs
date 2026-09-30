// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JQLCountResults {
    /// Number of issues matching JQL query.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
