// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Users operations.
pub struct UsersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UsersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Retrieves a user from the directory based on their `userId`.
    pub fn get_user(&self, directory_id: impl Into<String>, user_id: impl Into<String>) -> GetUserRequest<'a> {
        GetUserRequest::new(self.client, directory_id, user_id)
    }

    /// Update the directory-based user information using the user attributes associated with their `userId`. User information  is replaced attribute-by-attribute, with the exception of immutable and read-only  attributes. Existing values of unspecified attributes are cleaned.
    pub fn replace_user(
        &self,
        directory_id: impl Into<String>,
        user_id: impl Into<String>,
        scim_user: ScimUser,
    ) -> ReplaceUserRequest<'a> {
        ReplaceUserRequest::new(self.client, directory_id, user_id, scim_user)
    }

    /// Deleting a user via the SCIM APIs will unlink the user from your identity provider and deactivate the user within Atlassian if they are managed by your organization.
    ///
    /// The deleted user is not available for future requests until created with a new `userId`. If the user is deactivated they can be activated again via [Atlassian Administration](https://admin.atlassian.com/).
    ///
    /// **Note:** Executing this API call will result in the deletion of the SCIM record, and there is no method to reverse these changes except by creating a new SCIM record with [Create a user API](https://developer.atlassian.com/cloud/admin/user-provisioning/rest/api-group-users/#api-scim-directory-directoryid-users-post).
    pub fn delete_user(&self, directory_id: impl Into<String>, user_id: impl Into<String>) -> DeleteUserRequest<'a> {
        DeleteUserRequest::new(self.client, directory_id, user_id)
    }

    /// Updates a user's information in the directory based on their `userId` via `PATCH`. Refer to  [Service Provider Configuration APIs](https://developer.atlassian.com/cloud/admin/user-provisioning/rest/api-group-service-provider-configuration/#api-group-service-provider-configuration) for details on supported operations.
    pub fn patch_user(
        &self,
        directory_id: impl Into<String>,
        user_id: impl Into<String>,
        request_payload_to_patch: RequestPayloadToPatch,
    ) -> PatchUserRequest<'a> {
        PatchUserRequest::new(self.client, directory_id, user_id, request_payload_to_patch)
    }

    /// Get users from the specified directory. Filtering is supported with a single exact match  (`eq`) against the `userName` and `externalId` attributes.
    ///
    ///  **Note**: While this API enables pagination, sorting functionality is not supported.
    pub fn get_users(&self, directory_id: impl Into<String>) -> GetUsersRequest<'a> {
        GetUsersRequest::new(self.client, directory_id)
    }

    /// Creates a user in the directory.
    /// **Note:** An attempt to create an existing user will fail with a 409 (Conflict) error.
    ///
    /// Use this API to manage accounts outside your organization when assigning these users to SCIM groups.
    ///
    /// If there's already a managed Atlassian account associated with the specified email address on the Atlassian platform, the user in your identity provider will be connected or linked to the user in your Atlassian organization.
    pub fn create_user(&self, directory_id: impl Into<String>, scim_user: ScimUser) -> CreateUserRequest<'a> {
        CreateUserRequest::new(self.client, directory_id, scim_user)
    }
}

/// Retrieves a user from the directory based on their `userId`.
pub struct GetUserRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    user_id: String,
    attributes: Option<String>,
    excluded_attributes: Option<String>,
}

impl<'a> GetUserRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            client,
            directory_id: directory_id.into(),
            user_id: user_id.into(),
            attributes: None,
            excluded_attributes: None,
        }
    }

    /// Resource attributes to be included in response. Mutually exclusive with `excludedAttributes`.  Example: `userName,emails.value`
    #[must_use]
    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());

        self
    }

    /// Resource attributes to be excluded from response. Mutually exclusive with `attributes`.  Example: `timezone,emails.type,department`
    #[must_use]
    pub fn excluded_attributes(mut self, value: impl Into<String>) -> Self {
        self.excluded_attributes = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/scim/directory/{}/Users/{}", self.directory_id, self.user_id),
        );

        if let Some(value) = &self.attributes {
            config.query.push(("attributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.excluded_attributes {
            config.query.push(("excludedAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimUser> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update the directory-based user information using the user attributes associated with their `userId`. User information  is replaced attribute-by-attribute, with the exception of immutable and read-only  attributes. Existing values of unspecified attributes are cleaned.
pub struct ReplaceUserRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    user_id: String,
    attributes: Option<String>,
    excluded_attributes: Option<String>,
    scim_user: ScimUser,
}

impl<'a> ReplaceUserRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        directory_id: impl Into<String>,
        user_id: impl Into<String>,
        scim_user: ScimUser,
    ) -> Self {
        Self {
            client,
            directory_id: directory_id.into(),
            user_id: user_id.into(),
            scim_user,
            attributes: None,
            excluded_attributes: None,
        }
    }

    /// Resource attributes to be included in the response. Mutually exclusive from  `excludedAttributes`. Example: `userName,emails.value`
    #[must_use]
    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());

        self
    }

    /// Resource attributes to be excluded in the response. Mutually exclusive from `attributes`.  Example: `timezone,emails.type,department`
    #[must_use]
    pub fn excluded_attributes(mut self, value: impl Into<String>) -> Self {
        self.excluded_attributes = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/scim/directory/{}/Users/{}", self.directory_id, self.user_id),
        );

        if let Some(value) = &self.attributes {
            config.query.push(("attributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.excluded_attributes {
            config.query.push(("excludedAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.scim_user)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimUser> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deleting a user via the SCIM APIs will unlink the user from your identity provider and deactivate the user within Atlassian if they are managed by your organization.
///
/// The deleted user is not available for future requests until created with a new `userId`. If the user is deactivated they can be activated again via [Atlassian Administration](https://admin.atlassian.com/).
///
/// **Note:** Executing this API call will result in the deletion of the SCIM record, and there is no method to reverse these changes except by creating a new SCIM record with [Create a user API](https://developer.atlassian.com/cloud/admin/user-provisioning/rest/api-group-users/#api-scim-directory-directoryid-users-post).
pub struct DeleteUserRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    user_id: String,
}

impl<'a> DeleteUserRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self { client, directory_id: directory_id.into(), user_id: user_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/scim/directory/{}/Users/{}", self.directory_id, self.user_id),
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

/// Updates a user's information in the directory based on their `userId` via `PATCH`. Refer to  [Service Provider Configuration APIs](https://developer.atlassian.com/cloud/admin/user-provisioning/rest/api-group-service-provider-configuration/#api-group-service-provider-configuration) for details on supported operations.
pub struct PatchUserRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    user_id: String,
    attributes: Option<String>,
    excluded_attributes: Option<String>,
    request_payload_to_patch: RequestPayloadToPatch,
}

impl<'a> PatchUserRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        directory_id: impl Into<String>,
        user_id: impl Into<String>,
        request_payload_to_patch: RequestPayloadToPatch,
    ) -> Self {
        Self {
            client,
            directory_id: directory_id.into(),
            user_id: user_id.into(),
            request_payload_to_patch,
            attributes: None,
            excluded_attributes: None,
        }
    }

    /// Resource attributes to be included in the response. Mutually exclusive from  `excludedAttributes`. Example: `userName,emails.value`
    #[must_use]
    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());

        self
    }

    /// Resource attributes to be included in the response. Mutually exclusive from `attributes`.  Example: `timezone,emails.type,department`
    #[must_use]
    pub fn excluded_attributes(mut self, value: impl Into<String>) -> Self {
        self.excluded_attributes = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PATCH,
            format!("/scim/directory/{}/Users/{}", self.directory_id, self.user_id),
        );

        if let Some(value) = &self.attributes {
            config.query.push(("attributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.excluded_attributes {
            config.query.push(("excludedAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.request_payload_to_patch)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimUser> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get users from the specified directory. Filtering is supported with a single exact match  (`eq`) against the `userName` and `externalId` attributes.
///
///  **Note**: While this API enables pagination, sorting functionality is not supported.
pub struct GetUsersRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    attributes: Option<String>,
    excluded_attributes: Option<String>,
    filter: Option<String>,
    start_index: Option<i64>,
    count: Option<i64>,
}

impl<'a> GetUsersRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>) -> Self {
        Self {
            client,
            directory_id: directory_id.into(),
            attributes: None,
            excluded_attributes: None,
            filter: None,
            start_index: None,
            count: None,
        }
    }

    /// Resource attributes to be included in response. Mutually exclusive from `excludedAttributes`.  Example: `userName,emails.value`
    #[must_use]
    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());

        self
    }

    /// Resource attributes to be excluded from response. Mutually exclusive from `attributes`.  Example: `timezone,emails.type,department`
    #[must_use]
    pub fn excluded_attributes(mut self, value: impl Into<String>) -> Self {
        self.excluded_attributes = Some(value.into());

        self
    }

    /// Filter for `userName` or `externalId`. Example: `userName eq "Atlassian"`
    #[must_use]
    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());

        self
    }

    /// A 1-based index of the first query result.
    #[must_use]
    pub fn start_index(mut self, value: i64) -> Self {
        self.start_index = Some(value);

        self
    }

    /// Desired maximum number of query results in the list response page.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/scim/directory/{}/Users", self.directory_id),
        );

        if let Some(value) = &self.attributes {
            config.query.push(("attributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.excluded_attributes {
            config.query.push(("excludedAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.filter {
            config.query.push(("filter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_index {
            config.query.push(("startIndex".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.count {
            config.query.push(("count".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimUserListResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a user in the directory.
/// **Note:** An attempt to create an existing user will fail with a 409 (Conflict) error.
///
/// Use this API to manage accounts outside your organization when assigning these users to SCIM groups.
///
/// If there's already a managed Atlassian account associated with the specified email address on the Atlassian platform, the user in your identity provider will be connected or linked to the user in your Atlassian organization.
pub struct CreateUserRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    attributes: Option<String>,
    excluded_attributes: Option<String>,
    scim_user: ScimUser,
}

impl<'a> CreateUserRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>, scim_user: ScimUser) -> Self {
        Self { client, directory_id: directory_id.into(), scim_user, attributes: None, excluded_attributes: None }
    }

    /// Resource attributes to be included in response. Mutually exclusive from `excludedAttributes`.  Example: `userName,emails.value`
    #[must_use]
    pub fn attributes(mut self, value: impl Into<String>) -> Self {
        self.attributes = Some(value.into());

        self
    }

    /// Resource attributes to be excluded from response. Mutually exclusive from  `attributes`. Example: `timezone,emails.type,department`
    #[must_use]
    pub fn excluded_attributes(mut self, value: impl Into<String>) -> Self {
        self.excluded_attributes = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/scim/directory/{}/Users", self.directory_id),
        );

        if let Some(value) = &self.attributes {
            config.query.push(("attributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.excluded_attributes {
            config.query.push(("excludedAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.scim_user)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimUser> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
