// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetPermissionSchemesRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreatePermissionSchemeRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetPermissionSchemeRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePermissionSchemeRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetPermissionSchemeGrantsRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreatePermissionGrantRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetPermissionSchemeGrantRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The PermissionSchemes operations.
pub struct PermissionSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PermissionSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all permission schemes. By default only shortened beans are returned. If you want to include permissions of all the schemes, then specify the permissions expand parameter. Permissions will be included also if you specify any other expand parameter.
    pub fn get_permission_schemes(&self) -> GetPermissionSchemesRequest<'a> {
        GetPermissionSchemesRequest::new(self.client)
    }

    /// Create a new permission scheme. This method can create schemes with a defined permission set, or without.
    pub fn create_permission_scheme(&self) -> CreatePermissionSchemeRequest<'a> {
        CreatePermissionSchemeRequest::new(self.client)
    }

    /// Returns the attribute for a permission scheme specified by permission scheme id and attribute key.
    pub fn get_scheme_attribute(
        &self,
        permission_scheme_id: i64,
        attribute_key: impl Into<String>,
    ) -> GetSchemeAttributeRequest<'a> {
        GetSchemeAttributeRequest::new(self.client, permission_scheme_id, attribute_key)
    }

    /// Updates or inserts the attribute for a permission scheme specified by permission scheme id. The attribute consists of the key and the value. The value will be converted to Boolean using Boolean#valueOf.
    pub fn set_scheme_attribute(
        &self,
        permission_scheme_id: i64,
        key: impl Into<String>,
    ) -> SetSchemeAttributeRequest<'a> {
        SetSchemeAttributeRequest::new(self.client, permission_scheme_id, key)
    }

    /// Returns a permission scheme identified by the given id.
    pub fn get_permission_scheme(&self, scheme_id: i64) -> GetPermissionSchemeRequest<'a> {
        GetPermissionSchemeRequest::new(self.client, scheme_id)
    }

    /// Updates a permission scheme. If the permissions list is present then it will be set in the permission scheme, which basically means it will overwrite any permission grants that existed in the permission scheme. Sending an empty list will remove all permission grants from the permission scheme. To update just the name and description, do not send permissions list at all. To add or remove a single permission grant instead of updating the whole list at once use the {schemeId}/permission/ resource.
    pub fn update_permission_scheme(&self, scheme_id: i64) -> UpdatePermissionSchemeRequest<'a> {
        UpdatePermissionSchemeRequest::new(self.client, scheme_id)
    }

    /// Deletes a permission scheme identified by the given id.
    pub fn delete_permission_scheme(&self, scheme_id: i64) -> DeletePermissionSchemeRequest<'a> {
        DeletePermissionSchemeRequest::new(self.client, scheme_id)
    }

    /// Returns all permission grants of the given permission scheme.
    pub fn get_permission_scheme_grants(&self, scheme_id: i64) -> GetPermissionSchemeGrantsRequest<'a> {
        GetPermissionSchemeGrantsRequest::new(self.client, scheme_id)
    }

    /// Creates a permission grant in a permission scheme.
    pub fn create_permission_grant(&self, scheme_id: i64) -> CreatePermissionGrantRequest<'a> {
        CreatePermissionGrantRequest::new(self.client, scheme_id)
    }

    /// Returns a permission grant identified by the given id.
    pub fn get_permission_scheme_grant(
        &self,
        permission_id: i64,
        scheme_id: i64,
    ) -> GetPermissionSchemeGrantRequest<'a> {
        GetPermissionSchemeGrantRequest::new(self.client, permission_id, scheme_id)
    }

    /// Deletes a permission grant from a permission scheme.
    pub fn delete_permission_scheme_entity(
        &self,
        permission_id: i64,
        scheme_id: i64,
    ) -> DeletePermissionSchemeEntityRequest<'a> {
        DeletePermissionSchemeEntityRequest::new(self.client, permission_id, scheme_id)
    }
}

/// Returns a list of all permission schemes. By default only shortened beans are returned. If you want to include permissions of all the schemes, then specify the permissions expand parameter. Permissions will be included also if you specify any other expand parameter.
#[derive(Clone)]
pub struct GetPermissionSchemesRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetPermissionSchemesRequestExpand>,
}

impl<'a> GetPermissionSchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: GetPermissionSchemesRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/permissionscheme".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionSchemes> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a new permission scheme. This method can create schemes with a defined permission set, or without.
#[derive(Clone)]
pub struct CreatePermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<CreatePermissionSchemeRequestExpand>,
    body: Option<PermissionScheme>,
}

impl<'a> CreatePermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, body: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: CreatePermissionSchemeRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn body(mut self, value: PermissionScheme) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/permissionscheme".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the attribute for a permission scheme specified by permission scheme id and attribute key.
#[derive(Clone)]
pub struct GetSchemeAttributeRequest<'a> {
    client: &'a crate::core::Client,
    permission_scheme_id: i64,
    attribute_key: String,
}

impl<'a> GetSchemeAttributeRequest<'a> {
    fn new(client: &'a crate::core::Client, permission_scheme_id: i64, attribute_key: impl Into<String>) -> Self {
        Self { client, permission_scheme_id, attribute_key: attribute_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/permissionscheme/{}/attribute/{}",
                self.permission_scheme_id,
                crate::core::encode_path_segment(&self.attribute_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionSchemeAttribute> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates or inserts the attribute for a permission scheme specified by permission scheme id. The attribute consists of the key and the value. The value will be converted to Boolean using Boolean#valueOf.
#[derive(Clone)]
pub struct SetSchemeAttributeRequest<'a> {
    client: &'a crate::core::Client,
    permission_scheme_id: i64,
    key: String,
    body: Option<String>,
}

impl<'a> SetSchemeAttributeRequest<'a> {
    fn new(client: &'a crate::core::Client, permission_scheme_id: i64, key: impl Into<String>) -> Self {
        Self { client, permission_scheme_id, key: key.into(), body: None }
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/permissionscheme/{}/attribute/{}",
                self.permission_scheme_id,
                crate::core::encode_path_segment(&self.key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        config.content_type = Some("text/plain".to_owned());

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

/// Returns a permission scheme identified by the given id.
#[derive(Clone)]
pub struct GetPermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetPermissionSchemeRequestExpand>,
    scheme_id: i64,
}

impl<'a> GetPermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id, expand: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: GetPermissionSchemeRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/permissionscheme/{}", self.scheme_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a permission scheme. If the permissions list is present then it will be set in the permission scheme, which basically means it will overwrite any permission grants that existed in the permission scheme. Sending an empty list will remove all permission grants from the permission scheme. To update just the name and description, do not send permissions list at all. To add or remove a single permission grant instead of updating the whole list at once use the {schemeId}/permission/ resource.
#[derive(Clone)]
pub struct UpdatePermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<UpdatePermissionSchemeRequestExpand>,
    scheme_id: i64,
    body: Option<PermissionScheme>,
}

impl<'a> UpdatePermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id, expand: None, body: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: UpdatePermissionSchemeRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn body(mut self, value: PermissionScheme) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/permissionscheme/{}", self.scheme_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a permission scheme identified by the given id.
#[derive(Clone)]
pub struct DeletePermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: i64,
}

impl<'a> DeletePermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/permissionscheme/{}", self.scheme_id),
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

/// Returns all permission grants of the given permission scheme.
#[derive(Clone)]
pub struct GetPermissionSchemeGrantsRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetPermissionSchemeGrantsRequestExpand>,
    scheme_id: i64,
}

impl<'a> GetPermissionSchemeGrantsRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id, expand: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: GetPermissionSchemeGrantsRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/permissionscheme/{}/permission", self.scheme_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionGrants> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a permission grant in a permission scheme.
#[derive(Clone)]
pub struct CreatePermissionGrantRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<CreatePermissionGrantRequestExpand>,
    scheme_id: i64,
    permission_grant: Option<PermissionGrant>,
}

impl<'a> CreatePermissionGrantRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id, expand: None, permission_grant: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: CreatePermissionGrantRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn permission_grant(mut self, value: PermissionGrant) -> Self {
        self.permission_grant = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/permissionscheme/{}/permission", self.scheme_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        let body = match serde_json::to_value(&self.permission_grant)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionGrant> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a permission grant identified by the given id.
#[derive(Clone)]
pub struct GetPermissionSchemeGrantRequest<'a> {
    client: &'a crate::core::Client,
    permission_id: i64,
    expand: Option<GetPermissionSchemeGrantRequestExpand>,
    scheme_id: i64,
}

impl<'a> GetPermissionSchemeGrantRequest<'a> {
    fn new(client: &'a crate::core::Client, permission_id: i64, scheme_id: i64) -> Self {
        Self { client, permission_id, scheme_id, expand: None }
    }

    /// Use expand to include full beans in the response. This parameter accepts a comma-separated list of expandable elements. Use 'permissions' to include permissions in the response.
    #[must_use]
    pub fn expand(mut self, value: GetPermissionSchemeGrantRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/permissionscheme/{}/permission/{}", self.scheme_id, self.permission_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionGrant> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a permission grant from a permission scheme.
#[derive(Clone)]
pub struct DeletePermissionSchemeEntityRequest<'a> {
    client: &'a crate::core::Client,
    permission_id: i64,
    scheme_id: i64,
}

impl<'a> DeletePermissionSchemeEntityRequest<'a> {
    fn new(client: &'a crate::core::Client, permission_id: i64, scheme_id: i64) -> Self {
        Self { client, permission_id, scheme_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/permissionscheme/{}/permission/{}", self.scheme_id, self.permission_id),
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
