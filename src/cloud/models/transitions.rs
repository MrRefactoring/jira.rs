// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of issue transitions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Transitions {
    /// Expand options that include additional transitions details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// List of issue transitions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<IssueTransition>>,
}
