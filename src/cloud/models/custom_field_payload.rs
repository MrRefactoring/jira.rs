// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The strategy to use when there is a conflict with an existing custom field. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters
    pub enum CustomFieldPayloadOnConflict {
        Fail => "FAIL",
        Use => "USE",
        New => "NEW",
    }
}

crate::open_enum! {
    /// Allows an overwrite to declare the new Custom Field to be created as a GLOBAL-scoped field. Leave this as empty or null to use the project's default scope.
    pub enum CustomFieldPayloadScope {
        Global => "GLOBAL",
        Template => "TEMPLATE",
        Project => "PROJECT",
    }
}

/// Defines the payload for the custom field definitions. See <https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-fields/#api-rest-api-3-field-post>
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldPayload {
    /// The type of the custom field
    #[serde(rename = "cfType", default, skip_serializing_if = "Option::is_none")]
    pub cf_type: Option<String>,
    /// The description of the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The strategy to use when there is a conflict with an existing custom field. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters
    #[serde(rename = "onConflict", default, skip_serializing_if = "Option::is_none")]
    pub on_conflict: Option<CustomFieldPayloadOnConflict>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    /// Allows an overwrite to declare the new Custom Field to be created as a GLOBAL-scoped field. Leave this as empty or null to use the project's default scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<CustomFieldPayloadScope>,
    /// The searcher key of the custom field
    #[serde(rename = "searcherKey", default, skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<String>,
}
