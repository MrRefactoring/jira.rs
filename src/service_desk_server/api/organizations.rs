// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Organizations operations.
pub struct OrganizationsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> OrganizationsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all the users of a specified organization.
    pub fn get_users_in_organization(&self, organization_id: impl Into<String>) -> GetUsersInOrganizationRequest<'a> {
        GetUsersInOrganizationRequest::new(self.client, organization_id)
    }

    /// Adds users to an organization.
    pub fn add_users_to_organization(&self, organization_id: impl Into<String>) -> AddUsersToOrganizationRequest<'a> {
        AddUsersToOrganizationRequest::new(self.client, organization_id)
    }

    /// Removes users from an organization.
    pub fn remove_users_from_organization(
        &self,
        organization_id: impl Into<String>,
    ) -> RemoveUsersFromOrganizationRequest<'a> {
        RemoveUsersFromOrganizationRequest::new(self.client, organization_id)
    }

    /// Preview the cleanup of empty organizations, with the same support parameters.
    pub fn preview_clean_up_organizations(&self) -> PreviewCleanUpOrganizationsRequest<'a> {
        PreviewCleanUpOrganizationsRequest::new(self.client)
    }

    /// Deletes empty organizations, optionally delete organizations that have no active users, or are not attached to any projects.
    pub fn clean_up_organizations(&self) -> CleanUpOrganizationsRequest<'a> {
        CleanUpOrganizationsRequest::new(self.client)
    }

    /// Returns a list of organizations in the Jira instance.If the user is not an agent, the resource returns a list of organizations the user is a member of.
    pub fn get_organizations(&self) -> GetOrganizationsRequest<'a> {
        GetOrganizationsRequest::new(self.client)
    }

    /// To create an organization Jira administrator global permission or agent permission is required depending on the settings
    pub fn create_organization(&self) -> CreateOrganizationRequest<'a> {
        CreateOrganizationRequest::new(self.client)
    }

    /// Returns an organization for a given organization ID.
    pub fn get_organization(&self, organization_id: impl Into<String>) -> GetOrganizationRequest<'a> {
        GetOrganizationRequest::new(self.client, organization_id)
    }

    /// Deletes an organization for a given organization ID.
    pub fn delete_organization(&self, organization_id: impl Into<String>) -> DeleteOrganizationRequest<'a> {
        DeleteOrganizationRequest::new(self.client, organization_id)
    }
}

/// Returns all the users of a specified organization.
#[derive(Clone)]
pub struct GetUsersInOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetUsersInOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: impl Into<String>) -> Self {
        Self { client, organization_id: organization_id.into(), start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/organization/{}/user",
                crate::core::encode_path_segment(&self.organization_id)
            ),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<User>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<User>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds users to an organization.
#[derive(Clone)]
pub struct AddUsersToOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
    users_organization_update: Option<UsersOrganizationUpdate>,
}

impl<'a> AddUsersToOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: impl Into<String>) -> Self {
        Self { client, organization_id: organization_id.into(), users_organization_update: None }
    }

    #[must_use]
    pub fn users_organization_update(mut self, value: UsersOrganizationUpdate) -> Self {
        self.users_organization_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/organization/{}/user",
                crate::core::encode_path_segment(&self.organization_id)
            ),
        );

        let body = match serde_json::to_value(&self.users_organization_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

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

/// Removes users from an organization.
#[derive(Clone)]
pub struct RemoveUsersFromOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
    users_organization_update: Option<UsersOrganizationUpdate>,
}

impl<'a> RemoveUsersFromOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: impl Into<String>) -> Self {
        Self { client, organization_id: organization_id.into(), users_organization_update: None }
    }

    #[must_use]
    pub fn users_organization_update(mut self, value: UsersOrganizationUpdate) -> Self {
        self.users_organization_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/servicedeskapi/organization/{}/user",
                crate::core::encode_path_segment(&self.organization_id)
            ),
        );

        let body = match serde_json::to_value(&self.users_organization_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

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

/// Preview the cleanup of empty organizations, with the same support parameters.
#[derive(Clone)]
pub struct PreviewCleanUpOrganizationsRequest<'a> {
    client: &'a crate::core::Client,
    delete_detached_organizations: Option<String>,
    delete_organizations_with_inactive_users: Option<String>,
}

impl<'a> PreviewCleanUpOrganizationsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, delete_detached_organizations: None, delete_organizations_with_inactive_users: None }
    }

    /// If true, in addition, preview the deletion of organizations that are not attached to any projects.
    /// Default is false.
    #[must_use]
    pub fn delete_detached_organizations(mut self, value: impl Into<String>) -> Self {
        self.delete_detached_organizations = Some(value.into());

        self
    }

    /// If true, in addition, preview the deletion of organizations that have no active users.
    /// Default is false.
    #[must_use]
    pub fn delete_organizations_with_inactive_users(mut self, value: impl Into<String>) -> Self {
        self.delete_organizations_with_inactive_users = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/servicedeskapi/organization/cleanup".to_owned(),
        );

        if let Some(value) = &self.delete_detached_organizations {
            config
                .query
                .push(("deleteDetachedOrganizations".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.delete_organizations_with_inactive_users {
            config.query.push((
                "deleteOrganizationsWithInactiveUsers".to_owned(),
                crate::core::QueryValue::Scalar(value.clone()),
            ));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<CustomerOrganization>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes empty organizations, optionally delete organizations that have no active users, or are not attached to any projects.
#[derive(Clone)]
pub struct CleanUpOrganizationsRequest<'a> {
    client: &'a crate::core::Client,
    delete_detached_organizations: Option<String>,
    delete_organizations_with_inactive_users: Option<String>,
}

impl<'a> CleanUpOrganizationsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, delete_detached_organizations: None, delete_organizations_with_inactive_users: None }
    }

    /// If true, in addition, delete organizations that are not attached to any projects.
    #[must_use]
    pub fn delete_detached_organizations(mut self, value: impl Into<String>) -> Self {
        self.delete_detached_organizations = Some(value.into());

        self
    }

    /// If true, in addition, delete organizations that have no active users.
    #[must_use]
    pub fn delete_organizations_with_inactive_users(mut self, value: impl Into<String>) -> Self {
        self.delete_organizations_with_inactive_users = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            "/rest/servicedeskapi/organization/cleanup".to_owned(),
        );

        if let Some(value) = &self.delete_detached_organizations {
            config
                .query
                .push(("deleteDetachedOrganizations".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.delete_organizations_with_inactive_users {
            config.query.push((
                "deleteOrganizationsWithInactiveUsers".to_owned(),
                crate::core::QueryValue::Scalar(value.clone()),
            ));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of organizations in the Jira instance.If the user is not an agent, the resource returns a list of organizations the user is a member of.
#[derive(Clone)]
pub struct GetOrganizationsRequest<'a> {
    client: &'a crate::core::Client,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetOrganizationsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/organization".to_owned());

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Organization>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Organization>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// To create an organization Jira administrator global permission or agent permission is required depending on the settings
#[derive(Clone)]
pub struct CreateOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_create: Option<OrganizationCreate>,
}

impl<'a> CreateOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, organization_create: None }
    }

    #[must_use]
    pub fn organization_create(mut self, value: OrganizationCreate) -> Self {
        self.organization_create = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/servicedeskapi/organization".to_owned());

        let body = match serde_json::to_value(&self.organization_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Organization> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns an organization for a given organization ID.
#[derive(Clone)]
pub struct GetOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
}

impl<'a> GetOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: impl Into<String>) -> Self {
        Self { client, organization_id: organization_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/organization/{}", crate::core::encode_path_segment(&self.organization_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Organization> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an organization for a given organization ID.
#[derive(Clone)]
pub struct DeleteOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    organization_id: String,
}

impl<'a> DeleteOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, organization_id: impl Into<String>) -> Self {
        Self { client, organization_id: organization_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/servicedeskapi/organization/{}", crate::core::encode_path_segment(&self.organization_id)),
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
