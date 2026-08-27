// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAndReplaceVersion {
    /// An array of custom field IDs (`customFieldId`) and version IDs (`moveTo`) to update when the fields contain the deleted version.
    #[serde(rename = "customFieldReplacementList", default, skip_serializing_if = "Option::is_none")]
    pub custom_field_replacement_list: Option<Vec<CustomFieldReplacement>>,
    /// The ID of the version to update `affectedVersion` to when the field contains the deleted version.
    #[serde(rename = "moveAffectedIssuesTo", default, skip_serializing_if = "Option::is_none")]
    pub move_affected_issues_to: Option<i64>,
    /// The ID of the version to update `fixVersion` to when the field contains the deleted version.
    #[serde(rename = "moveFixIssuesTo", default, skip_serializing_if = "Option::is_none")]
    pub move_fix_issues_to: Option<i64>,
}
