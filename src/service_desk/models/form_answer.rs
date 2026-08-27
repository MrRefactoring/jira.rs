// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormAnswer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adf: Option<JsonNode>,
    /// IDs of selected choices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<String>>,
    /// Answer in date format (yyyy-MM-dd)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The IDs of files to be attached to the form that are obtained by calling the ‘attach temporary file’ endpoint on the corresponding service desk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    /// Answer in free text format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Answer in timestamp format (HH:mm)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// IDs of selected users
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}
