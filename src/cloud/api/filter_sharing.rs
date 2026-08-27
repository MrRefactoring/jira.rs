// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The FilterSharing operations.
pub struct FilterSharingService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FilterSharingService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the default sharing settings for new filters and dashboards for a user.
    ///
    /// **[Permissions](#permissions) required:** Permission to access Jira.
    pub fn get_default_share_scope(&self) -> GetDefaultShareScopeRequest<'a> {
        GetDefaultShareScopeRequest::new(self.client)
    }

    /// Sets the default sharing for new filters and dashboards for a user.
    ///
    /// **[Permissions](#permissions) required:** Permission to access Jira.
    pub fn set_default_share_scope(&self, default_share_scope: DefaultShareScope) -> SetDefaultShareScopeRequest<'a> {
        SetDefaultShareScopeRequest::new(self.client, default_share_scope)
    }

    /// Returns the share permissions for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None, however, share permissions are only returned for:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn get_share_permissions(&self, id: i64) -> GetSharePermissionsRequest<'a> {
        GetSharePermissionsRequest::new(self.client, id)
    }

    /// Add a share permissions to a filter. If you add a global share permission (one for all logged-in users or the public) it will overwrite all share permissions for the filter.
    ///
    /// Be aware that this operation uses different objects for updating share permissions compared to [Update filter](#api-rest-api-3-filter-id-put).
    ///
    /// **[Permissions](#permissions) required:** *Share dashboards and filters* [global permission](https://confluence.atlassian.com/x/x4dKLg) and the user must own the filter.
    pub fn add_share_permission(
        &self,
        id: i64,
        share_permission_input: SharePermissionInput,
    ) -> AddSharePermissionRequest<'a> {
        AddSharePermissionRequest::new(self.client, id, share_permission_input)
    }

    /// Returns a share permission for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None, however, a share permission is only returned for:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn get_share_permission(&self, id: i64, permission_id: i64) -> GetSharePermissionRequest<'a> {
        GetSharePermissionRequest::new(self.client, id, permission_id)
    }

    /// Deletes a share permission from a filter.
    ///
    /// **[Permissions](#permissions) required:** Permission to access Jira and the user must own the filter.
    pub fn delete_share_permission(&self, id: i64, permission_id: i64) -> DeleteSharePermissionRequest<'a> {
        DeleteSharePermissionRequest::new(self.client, id, permission_id)
    }
}

/// Returns the default sharing settings for new filters and dashboards for a user.
///
/// **[Permissions](#permissions) required:** Permission to access Jira.
pub struct GetDefaultShareScopeRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetDefaultShareScopeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/filter/defaultShareScope".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<DefaultShareScope> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default sharing for new filters and dashboards for a user.
///
/// **[Permissions](#permissions) required:** Permission to access Jira.
pub struct SetDefaultShareScopeRequest<'a> {
    client: &'a crate::core::Client,
    default_share_scope: DefaultShareScope,
}

impl<'a> SetDefaultShareScopeRequest<'a> {
    fn new(client: &'a crate::core::Client, default_share_scope: DefaultShareScope) -> Self {
        Self { client, default_share_scope }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            "/rest/api/3/filter/defaultShareScope".to_owned(),
        );

        let body = match serde_json::to_value(&self.default_share_scope)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<DefaultShareScope> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the share permissions for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None, however, share permissions are only returned for:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct GetSharePermissionsRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetSharePermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/filter/{}/permission", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<SharePermission>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Add a share permissions to a filter. If you add a global share permission (one for all logged-in users or the public) it will overwrite all share permissions for the filter.
///
/// Be aware that this operation uses different objects for updating share permissions compared to [Update filter](#api-rest-api-3-filter-id-put).
///
/// **[Permissions](#permissions) required:** *Share dashboards and filters* [global permission](https://confluence.atlassian.com/x/x4dKLg) and the user must own the filter.
pub struct AddSharePermissionRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    share_permission_input: SharePermissionInput,
}

impl<'a> AddSharePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, share_permission_input: SharePermissionInput) -> Self {
        Self { client, id, share_permission_input }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/filter/{}/permission", self.id),
        );

        let body = match serde_json::to_value(&self.share_permission_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<SharePermission>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a share permission for a filter. A filter can be shared with groups, projects, all logged-in users, or the public. Sharing with all logged-in users or the public is known as a global share permission.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None, however, a share permission is only returned for:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct GetSharePermissionRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    permission_id: i64,
}

impl<'a> GetSharePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, permission_id: i64) -> Self {
        Self { client, id, permission_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/filter/{}/permission/{}", self.id, self.permission_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SharePermission> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a share permission from a filter.
///
/// **[Permissions](#permissions) required:** Permission to access Jira and the user must own the filter.
pub struct DeleteSharePermissionRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    permission_id: i64,
}

impl<'a> DeleteSharePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, permission_id: i64) -> Self {
        Self { client, id, permission_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/filter/{}/permission/{}", self.id, self.permission_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
