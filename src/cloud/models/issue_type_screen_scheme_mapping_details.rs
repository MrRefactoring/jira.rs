// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of issue type screen scheme mappings.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeMappingDetails {
    /// The list of issue type to screen scheme mappings. A *default* entry cannot be specified because a default entry is added when an issue type screen scheme is created.
    #[serde(rename = "issueTypeMappings")]
    pub issue_type_mappings: Vec<IssueTypeScreenSchemeMapping>,
}
