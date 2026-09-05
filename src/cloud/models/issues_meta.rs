// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Meta data describing the `issues` context variable.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuesMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<IssuesJqlMetaData>,
}
