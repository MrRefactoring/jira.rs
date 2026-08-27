// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A field auto-complete suggestion.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoCompleteSuggestion {
    /// The display name of a suggested item. If `fieldValue` or `predicateValue` are provided, the matching text is highlighted with the HTML bold tag.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The value of a suggested item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
