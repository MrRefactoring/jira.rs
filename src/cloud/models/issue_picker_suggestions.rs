// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of issues suggested for use in auto-completion.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssuePickerSuggestions {
    /// A list of issues for an issue type suggested for use in auto-completion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<IssuePickerSuggestionsIssueType>>,
}
