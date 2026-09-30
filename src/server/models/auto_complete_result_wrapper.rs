// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoCompleteResultWrapperResults {
    pub value: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoCompleteResultWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AutoCompleteResultWrapperResults>>,
}
