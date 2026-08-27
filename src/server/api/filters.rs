// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFilterRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFavouriteFiltersRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFilterRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EditFilterRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Filters operations.
pub struct FiltersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FiltersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a new filter, and returns newly created filter. Currently sets permissions just using the users default sharing permissions
    pub fn create_filter(&self) -> CreateFilterRequest<'a> {
        CreateFilterRequest::new(self.client)
    }

    /// Returns the default share scope of the logged-in user
    pub fn get_default_share_scope(&self) -> GetDefaultShareScopeRequest<'a> {
        GetDefaultShareScopeRequest::new(self.client)
    }

    /// Sets the default share scope of the logged-in user. Available values are: AUTHENTICATED (for sharing with all logged-in users) and PRIVATE (for no shares).
    pub fn set_default_share_scope(&self) -> SetDefaultShareScopeRequest<'a> {
        SetDefaultShareScopeRequest::new(self.client)
    }

    /// Returns the favourite filters of the logged-in user
    pub fn get_favourite_filters(&self) -> GetFavouriteFiltersRequest<'a> {
        GetFavouriteFiltersRequest::new(self.client)
    }

    /// Returns a filter given an id
    pub fn get_filter(&self, id: impl Into<String>) -> GetFilterRequest<'a> {
        GetFilterRequest::new(self.client, id)
    }

    /// Updates an existing filter, and returns its new value. The following properties of a filter can be updated: 'jql', 'name', 'description'. Additionally, administrators can also update the 'owner' field. To get, set or unset 'favourite', use rest/api/1.0/filters/{id}/favourite with GET, PUT and DELETE methods instead.
    pub fn edit_filter(&self, id: impl Into<String>) -> EditFilterRequest<'a> {
        EditFilterRequest::new(self.client, id)
    }

    /// Delete a filter
    pub fn delete_filter(&self, id: impl Into<String>) -> DeleteFilterRequest<'a> {
        DeleteFilterRequest::new(self.client, id)
    }

    /// Returns the default columns for the given filter. Currently logged in user will be used as the user making such request.
    pub fn get_filter_columns(&self, id: impl Into<String>) -> GetFilterColumnsRequest<'a> {
        GetFilterColumnsRequest::new(self.client, id)
    }

    /// Sets the default columns for the given filter
    pub fn set_columns(&self, id: impl Into<String>) -> SetColumnsRequest<'a> {
        SetColumnsRequest::new(self.client, id)
    }

    /// Resets the columns for the given filter such that the filter no longer has its own column config
    pub fn reset_columns(&self, id: impl Into<String>) -> ResetColumnsRequest<'a> {
        ResetColumnsRequest::new(self.client, id)
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

/// Creates a new filter, and returns newly created filter. Currently sets permissions just using the users default sharing permissions
pub struct CreateFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<CreateFilterRequestExpand>,
    filter: Option<Filter>,
}

impl<'a> CreateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, filter: None }
    }

    #[must_use]
    pub fn expand(mut self, value: CreateFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn filter(mut self, value: Filter) -> Self {
        self.filter = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/filter".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        let body = match serde_json::to_value(&self.filter)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the default share scope of the logged-in user
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

/// Returns the favourite filters of the logged-in user
pub struct GetFavouriteFiltersRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetFavouriteFiltersRequestExpand>,
}

impl<'a> GetFavouriteFiltersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: GetFavouriteFiltersRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/filter/favourite".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Filter>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a filter given an id
pub struct GetFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetFilterRequestExpand>,
    id: String,
}

impl<'a> GetFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: GetFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/2/filter/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates an existing filter, and returns its new value. The following properties of a filter can be updated: 'jql', 'name', 'description'. Additionally, administrators can also update the 'owner' field. To get, set or unset 'favourite', use rest/api/1.0/filters/{id}/favourite with GET, PUT and DELETE methods instead.
pub struct EditFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<EditFilterRequestExpand>,
    id: String,
    body: Option<Filter>,
}

impl<'a> EditFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), expand: None, body: None }
    }

    #[must_use]
    pub fn expand(mut self, value: EditFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn body(mut self, value: Filter) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, format!("/rest/api/2/filter/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a filter
pub struct DeleteFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, format!("/rest/api/2/filter/{}", self.id));

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

/// Returns the default columns for the given filter. Currently logged in user will be used as the user making such request.
pub struct GetFilterColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetFilterColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/filter/{}/columns", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnLayout>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default columns for the given filter
pub struct SetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    columns: Option<Vec<String>>,
}

impl<'a> SetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), columns: None }
    }

    #[must_use]
    pub fn columns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.columns = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/filter/{}/columns", self.id),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.columns {
            body.insert("columns".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Resets the columns for the given filter such that the filter no longer has its own column config
pub struct ResetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> ResetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/filter/{}/columns", self.id),
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

/// Returns all share permissions of the given filter
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
            format!("/rest/api/2/filter/{}/permission", self.id),
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
            format!("/rest/api/2/filter/{}/permission", self.id),
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
            format!("/rest/api/2/filter/{}/permission/{}", self.id, self.permission_id),
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
            format!("/rest/api/2/filter/{}/permission/{}", self.id, self.permission_id),
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
