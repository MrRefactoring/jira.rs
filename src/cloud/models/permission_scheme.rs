// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a permission scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionScheme {
    /// A description for the permission scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The expand options available for the permission scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the permission scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the permission scheme. Must be unique.
    pub name: String,
    /// The permission scheme to create or update. See [About permission schemes and grants](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-permission-schemes/#about-permission-schemes-and-grants) for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionGrant>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// The URL of the permission scheme.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
