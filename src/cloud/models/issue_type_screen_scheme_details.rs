// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The details of an issue type screen scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeDetails {
    /// The description of the issue type screen scheme. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The IDs of the screen schemes for the issue type IDs and *default*. A *default* entry is required to create an issue type screen scheme, it defines the mapping for all issue types without a screen scheme.
    #[serde(rename = "issueTypeMappings")]
    pub issue_type_mappings: Vec<IssueTypeScreenSchemeMapping>,
    /// The name of the issue type screen scheme. The name must be unique. The maximum length is 255 characters.
    pub name: String,
}
