// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// List of issues and JQL queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssuesAndJQLQueries {
    /// A list of up to 50 issue IDs.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<i64>,
    /// A list of up to 10 JQL queries.
    pub jqls: Vec<String>,
}
