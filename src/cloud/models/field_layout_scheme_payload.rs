// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Deprecated use [fieldAssociationScheme](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-field-schemes/#api-group-field-schemes) instead Defines the payload for the field layout schemes. See [ Field configuration scheme](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-field-configurations/#api-rest-api-3-fieldconfigurationscheme-post).
///
/// [ How to configure a field configuration scheme](https://support.atlassian.com/jira-cloud-administration/docs/configure-a-field-configuration-scheme/).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldLayoutSchemePayload {
    #[serde(rename = "defaultFieldLayout", default, skip_serializing_if = "Option::is_none")]
    pub default_field_layout: Option<ProjectCreateResourceIdentifier>,
    /// The description of the field layout scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// There is a default configuration "fieldlayout" that is applied to all issue types using this scheme that don't have an explicit mapping users can create (or re-use existing) configurations for other issue types and map them to this scheme
    #[serde(rename = "explicitMappings", default, skip_serializing_if = "Option::is_none")]
    pub explicit_mappings: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The name of the field layout scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
}
