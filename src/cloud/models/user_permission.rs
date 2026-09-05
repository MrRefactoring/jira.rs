// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of the permission.
    pub enum UserPermissionType {
        Global => "GLOBAL",
        Project => "PROJECT",
    }
}

/// Details of a permission and its availability to a user.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserPermission {
    /// Indicate whether the permission key is deprecated. Note that deprecated keys cannot be used in the `permissions parameter of Get my permissions. Deprecated keys are not returned by Get all permissions.`
    #[deprecated(note = "Indicate whether the permission key is deprecated.")]
    #[serde(rename = "deprecatedKey", default, skip_serializing_if = "Option::is_none")]
    pub deprecated_key: Option<bool>,
    /// The description of the permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the permission is available to the user in the queried context.
    #[serde(rename = "havePermission", default, skip_serializing_if = "Option::is_none")]
    pub have_permission: Option<bool>,
    /// The ID of the permission. Either `id` or `key` must be specified. Use [Get all permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-permissions/#api-rest-api-3-permissions-get) to get the list of permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the permission. Either `id` or `key` must be specified. Use [Get all permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-permissions/#api-rest-api-3-permissions-get) to get the list of permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UserPermissionType>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for UserPermission {
    const FIELDS: &'static [&'static str] =
        &["deprecatedKey", "description", "havePermission", "id", "key", "name", "type"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
