// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The details of a UI modification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UiModificationDetails {
    /// List of contexts of the UI modification. The maximum number of contexts is 1000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<UiModificationContextDetails>>,
    /// The data of the UI modification. The maximum size of the data is 50000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The description of the UI modification. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the UI modification.
    pub id: String,
    /// The name of the UI modification. The maximum length is 255 characters.
    pub name: String,
    /// The URL of the UI modification.
    #[serde(rename = "self")]
    pub self_: String,
}
