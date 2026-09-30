// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteAndReplaceVersion {
    #[serde(rename = "customFieldReplacementList", default, skip_serializing_if = "Option::is_none")]
    pub custom_field_replacement_list: Option<Vec<CustomFieldReplacement>>,
    #[serde(rename = "moveAffectedIssuesTo", default, skip_serializing_if = "Option::is_none")]
    pub move_affected_issues_to: Option<i64>,
    #[serde(rename = "moveFixIssuesTo", default, skip_serializing_if = "Option::is_none")]
    pub move_fix_issues_to: Option<i64>,
}
