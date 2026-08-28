// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The results from a JQL query.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoCompleteSuggestions {
    /// The list of suggested item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AutoCompleteSuggestion>>,
}
