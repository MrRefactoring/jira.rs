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

    /// Returns the default share scope of the logged-in user
    pub fn get_default_share_scope(&self) -> GetDefaultShareScopeRequest<'a> {
        GetDefaultShareScopeRequest::new(self.client)
    }

    /// Sets the default share scope of the logged-in user. Available values are: AUTHENTICATED (for sharing with all logged-in users) and PRIVATE (for no shares).
    pub fn set_default_share_scope(&self) -> SetDefaultShareScopeRequest<'a> {
        SetDefaultShareScopeRequest::new(self.client)
    }

    /// Returns all share permissions of the given filter
    pub fn get_share_permissions(&self, id: impl Into<String>) -> GetSharePermissionsRequest<'a> {
        GetSharePermissionsRequest::new(self.client, id)
    }

    /// Adds a share permissions to the given filter. Adding a global permission removes all previous permissions from the filter
    pub fn add_share_permission(&self, id: impl Into<String>) -> AddSharePermissionRequest<'a> {
        AddSharePermissionRequest::new(self.client, id)
    }

    /// Returns a single share permission of the given filter
    pub fn get_share_permission(
        &self,
        permission_id: impl Into<String>,
        id: impl Into<String>,
    ) -> GetSharePermissionRequest<'a> {
        GetSharePermissionRequest::new(self.client, permission_id, id)
    }

    /// Removes a share permissions from the given filter
    pub fn delete_share_permission(
        &self,
        id: impl Into<String>,
        permission_id: impl Into<String>,
    ) -> DeleteSharePermissionRequest<'a> {
        DeleteSharePermissionRequest::new(self.client, id, permission_id)
    }
}

/// Returns the default share scope of the logged-in user
#[derive(Clone)]
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
            "/rest/api/2/filter/defaultShareScope".to_owned(),
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

/// Sets the default share scope of the logged-in user. Available values are: AUTHENTICATED (for sharing with all logged-in users) and PRIVATE (for no shares).
#[derive(Clone)]
pub struct SetDefaultShareScopeRequest<'a> {
    client: &'a crate::core::Client,
    default_share_scope: Option<DefaultShareScope>,
}

impl<'a> SetDefaultShareScopeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, default_share_scope: None }
    }

    #[must_use]
    pub fn default_share_scope(mut self, value: DefaultShareScope) -> Self {
        self.default_share_scope = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            "/rest/api/2/filter/defaultShareScope".to_owned(),
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

/// Returns all share permissions of the given filter
#[derive(Clone)]
pub struct GetSharePermissionsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetSharePermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/filter/{}/permission", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<FilterPermission>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a share permissions to the given filter. Adding a global permission removes all previous permissions from the filter
#[derive(Clone)]
pub struct AddSharePermissionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    share_permission_input: Option<SharePermissionInput>,
}

impl<'a> AddSharePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), share_permission_input: None }
    }

    #[must_use]
    pub fn share_permission_input(mut self, value: SharePermissionInput) -> Self {
        self.share_permission_input = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/filter/{}/permission", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.share_permission_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<FilterPermission>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a single share permission of the given filter
#[derive(Clone)]
pub struct GetSharePermissionRequest<'a> {
    client: &'a crate::core::Client,
    permission_id: String,
    id: String,
}

impl<'a> GetSharePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client, permission_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, permission_id: permission_id.into(), id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/filter/{}/permission/{}",
                crate::core::encode_path_segment(&self.id),
                crate::core::encode_path_segment(&self.permission_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<FilterPermission> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes a share permissions from the given filter
#[derive(Clone)]
pub struct DeleteSharePermissionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    permission_id: String,
}

impl<'a> DeleteSharePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, permission_id: impl Into<String>) -> Self {
        Self { client, id: id.into(), permission_id: permission_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/filter/{}/permission/{}",
                crate::core::encode_path_segment(&self.id),
                crate::core::encode_path_segment(&self.permission_id)
            ),
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
