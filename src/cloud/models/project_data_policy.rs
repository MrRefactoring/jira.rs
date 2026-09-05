// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about data policy.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectDataPolicy {
    /// Whether the project contains any content inaccessible to the requesting application.
    #[serde(rename = "anyContentBlocked", default, skip_serializing_if = "Option::is_none")]
    pub any_content_blocked: Option<bool>,
}
