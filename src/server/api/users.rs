// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// the keys of the projects we are finding assignable users for, comma-separated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FindBulkAssignableUsersRequestProjectKeys {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Users operations.
pub struct UsersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UsersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a user.
    pub fn get_user(&self) -> GetUserRequest<'a> {
        GetUserRequest::new(self.client)
    }

    /// Create user. By default created user will not be notified with email. If password field is not set then password will be randomly generated.
    pub fn create_user(&self, user_write: UserWrite) -> CreateUserRequest<'a> {
        CreateUserRequest::new(self.client, user_write)
    }

    /// Modify user. The 'value' fields present will override the existing value. Fields skipped in request will not be changed.
    pub fn update_user(&self, body: UserWrite) -> UpdateUserRequest<'a> {
        UpdateUserRequest::new(self.client, body)
    }

    /// Removes user and its references (like project roles associations, watches, history). Note: user references will not be removed if multiple User Directories are used and there is a user with the same name existing in another directory (shadowing user).
    pub fn remove_user(&self) -> RemoveUserRequest<'a> {
        RemoveUserRequest::new(self.client)
    }

    /// Returns available accessibility personal settings along with `enabled` property that indicates the currently logged-in user preference.
    pub fn get_a11y_personal_settings(&self) -> GetA11yPersonalSettingsRequest<'a> {
        GetA11yPersonalSettingsRequest::new(self.client)
    }

    /// Validates user anonymization process.
    pub fn validate_user_anonymization(&self) -> ValidateUserAnonymizationRequest<'a> {
        ValidateUserAnonymizationRequest::new(self.client)
    }

    /// Schedules a user anonymization process. Requires system admin permission.
    pub fn schedule_user_anonymization(
        &self,
        user_anonymization_request: UserAnonymizationRequest,
    ) -> ScheduleUserAnonymizationRequest<'a> {
        ScheduleUserAnonymizationRequest::new(self.client, user_anonymization_request)
    }

    /// Returns information about a user anonymization operation progress.
    pub fn get_user_anonymization_progress(&self) -> GetUserAnonymizationProgressRequest<'a> {
        GetUserAnonymizationProgressRequest::new(self.client)
    }

    /// Validates user anonymization re-run process.
    pub fn validate_user_anonymization_rerun(&self) -> ValidateUserAnonymizationRerunRequest<'a> {
        ValidateUserAnonymizationRerunRequest::new(self.client)
    }

    /// Schedules a user anonymization process. Requires system admin permission.
    pub fn schedule_user_anonymization_rerun(
        &self,
        user_anonymization_rerun_request: UserAnonymizationRerunRequest,
    ) -> ScheduleUserAnonymizationRerunRequest<'a> {
        ScheduleUserAnonymizationRerunRequest::new(self.client, user_anonymization_rerun_request)
    }

    /// Removes stale user anonymization task, for scenarios when the node that was executing it is no longer alive. Use it only after making sure that the parent node of the task is actually down, and not just having connectivity issues.
    pub fn unlock_anonymization(&self) -> UnlockAnonymizationRequest<'a> {
        UnlockAnonymizationRequest::new(self.client)
    }

    /// Add user to given application. Admin permission will be required to perform this operation.
    pub fn add_user_to_application(&self) -> AddUserToApplicationRequest<'a> {
        AddUserToApplicationRequest::new(self.client)
    }

    /// Remove user from given application. Admin permission will be required to perform this operation.
    pub fn remove_user_from_application(&self) -> RemoveUserFromApplicationRequest<'a> {
        RemoveUserFromApplicationRequest::new(self.client)
    }

    /// Returns a list of users that match the search string and can be assigned issues for all the given projects.
    pub fn find_bulk_assignable_users(&self) -> FindBulkAssignableUsersRequest<'a> {
        FindBulkAssignableUsersRequest::new(self.client)
    }

    /// Returns a list of users that match the search string. This resource cannot be accessed anonymously. Please note that this resource should be called with an issue key when a list of assignable users is retrieved. For create only a project key should be supplied. The list of assignable users may be incorrect if it's called with the project key for editing.
    pub fn find_assignable_users(&self) -> FindAssignableUsersRequest<'a> {
        FindAssignableUsersRequest::new(self.client)
    }

    /// Converts temporary avatar into a real avatar
    pub fn create_user_avatar_from_temporary(
        &self,
        avatar_cropping: AvatarCropping,
    ) -> CreateUserAvatarFromTemporaryRequest<'a> {
        CreateUserAvatarFromTemporaryRequest::new(self.client, avatar_cropping)
    }

    /// Updates the avatar for the user.
    pub fn update_user_avatar(&self, avatar: Avatar) -> UpdateUserAvatarRequest<'a> {
        UpdateUserAvatarRequest::new(self.client, avatar)
    }

    /// Creates temporary avatar using multipart. The response is sent back as JSON stored in a textarea. This is because the client uses remote iframing to submit avatars using multipart. So we must send them a valid HTML page back from which the client parses the JSON from.
    /// Creating a temporary avatar is part of a 3-step process in uploading a new avatar for a user: upload, crop, confirm. This endpoint allows you to use a multipart upload instead of sending the image directly as the request body.
    /// You *must* use "avatar" as the name of the upload parameter:
    /// curl -c cookiejar.txt -X POST -u admin:admin -H "X-Atlassian-Token: no-check" \
    ///   -F "avatar=@mynewavatar.png;type=image/png" \
    ///   '<http://localhost:8090/jira/rest/api/2/user/avatar/temporary?username=admin>'
    pub fn store_temporary_user_avatar_using_multi_part(
        &self,
        avatar: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> StoreTemporaryUserAvatarUsingMultiPartRequest<'a> {
        StoreTemporaryUserAvatarUsingMultiPartRequest::new(self.client, avatar)
    }

    /// Deletes avatar
    pub fn delete_user_avatar(&self, id: i64) -> DeleteUserAvatarRequest<'a> {
        DeleteUserAvatarRequest::new(self.client, id)
    }

    /// Returns all avatars which are visible for the currently logged in user.
    pub fn get_all_user_avatars(&self) -> GetAllUserAvatarsRequest<'a> {
        GetAllUserAvatarsRequest::new(self.client)
    }

    /// Returns the default columns for the given user. Admin permission will be required to get columns for a user other than the currently logged in user.
    pub fn default_columns(&self) -> DefaultColumnsRequest<'a> {
        DefaultColumnsRequest::new(self.client)
    }

    /// Sets the default columns for the given user. Admin permission will be required to get columns for a user other than the currently logged in user.
    pub fn set_columns_url_encoded(&self) -> SetColumnsUrlEncodedRequest<'a> {
        SetColumnsUrlEncodedRequest::new(self.client)
    }

    /// Reset the default columns for the given user to the system default. Admin permission will be required to get columns for a user other than the currently logged in user.
    pub fn reset_user_columns(&self) -> ResetUserColumnsRequest<'a> {
        ResetUserColumnsRequest::new(self.client)
    }

    /// Returns a list of users that match the search string. This resource cannot be accessed anonymously.
    /// Duplicated means that the user has an account in more than one directory
    /// and either more than one account is active or the only active account does not belong to the directory
    /// with the highest priority.
    /// The data returned by this endpoint is cached for 10 minutes and the cache is flushed when any User Directory
    /// is added, removed, enabled, disabled, or synchronized.
    /// A System Administrator can also flush the cache manually.
    /// Related JAC ticket: <https://jira.atlassian.com/browse/JRASERVER-68797>
    pub fn get_duplicated_users_count(&self) -> GetDuplicatedUsersCountRequest<'a> {
        GetDuplicatedUsersCountRequest::new(self.client)
    }

    /// Returns duplicated users mapped to their directories with an indication if their accounts are active or not.
    /// Duplicated means that the user has an account in more than one directory and either more than one account is active
    /// or the only active account does not belong to the directory with the highest priority.
    /// The data returned by this endpoint is cached for 10 minutes and the cache is flushed when any User Directory
    /// is added, removed, enabled, disabled, or synchronized.
    /// A System Administrator can also flush the cache manually.
    /// Related JAC ticket: <https://jira.atlassian.com/browse/JRASERVER-68797>
    pub fn get_duplicated_users_mapping(&self) -> GetDuplicatedUsersMappingRequest<'a> {
        GetDuplicatedUsersMappingRequest::new(self.client)
    }

    /// Returns a list of all users. This resource cannot be accessed anonymously.
    /// This Api is a streaming-like endpoint. For performance and security  reasons, it is not indicating the total
    /// number of users available in the system. The first call should be done without the cursor parameter.
    /// Subsequent calls should use the value of the next cursor returned in the previous call. Specific values of
    /// cursor are not guaranteed to be valid in the future and are not part of the API, so they should not be used
    /// as a key for caching or storing data. The order in which the users are returned is not defined. It is guaranteed
    /// that the same user will not be returned twice in the sequence of calls. For resiliency reason this endpoint
    /// never returns 404 code, even if called with a cursor parameter that was not returned in the previous call.
    ///
    ///
    /// Available since Jira Data Center 11.0, and in 10.3 LTS.
    pub fn get_user_list(&self) -> GetUserListRequest<'a> {
        GetUserListRequest::new(self.client)
    }

    /// Modify user password.
    pub fn change_user_password(&self, password: Password) -> ChangeUserPasswordRequest<'a> {
        ChangeUserPasswordRequest::new(self.client, password)
    }

    /// Returns a list of users matching query with highlighting.
    pub fn find_users_for_picker(&self) -> FindUsersForPickerRequest<'a> {
        FindUsersForPickerRequest::new(self.client)
    }

    /// Returns the keys of all properties for the user identified by the key or by the id.
    pub fn get_user_property_keys(&self) -> GetUserPropertyKeysRequest<'a> {
        GetUserPropertyKeysRequest::new(self.client)
    }

    /// Returns the value of the property with a given key from the user identified by the key or by the id.
    pub fn get_user_property(&self, property_key: impl Into<String>) -> GetUserPropertyRequest<'a> {
        GetUserPropertyRequest::new(self.client, property_key)
    }

    /// Sets the value of the specified user's property.
    /// You can use this resource to store a custom data against the user identified by the key or by the id. The user
    /// who stores the data is required to have permissions to administer the user.
    pub fn set_user_property(
        &self,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetUserPropertyRequest<'a> {
        SetUserPropertyRequest::new(self.client, property_key, body)
    }

    /// Removes the property from the user identified by the key or by the id. The user who removes the property is required to have permissions to administer the user.
    pub fn delete_user_property(&self, property_key: impl Into<String>) -> DeleteUserPropertyRequest<'a> {
        DeleteUserPropertyRequest::new(self.client, property_key)
    }

    /// Finds users.
    pub fn find_users(&self) -> FindUsersRequest<'a> {
        FindUsersRequest::new(self.client)
    }

    /// Invalidates session of given user.
    pub fn delete_session(&self, username: impl Into<String>) -> DeleteSessionRequest<'a> {
        DeleteSessionRequest::new(self.client, username)
    }

    /// Returns a list of active users that match the search string. This resource cannot be accessed anonymously and requires the Browse Users global permission. Given an issue key this resource will provide a list of users that match the search string and have the browse issue permission for the issue provided.
    pub fn find_users_with_browse_permission(&self) -> FindUsersWithBrowsePermissionRequest<'a> {
        FindUsersWithBrowsePermissionRequest::new(self.client)
    }
}

/// Returns a user.
#[derive(Clone)]
pub struct GetUserRequest<'a> {
    client: &'a crate::core::Client,
    include_deleted: Option<bool>,
    key: Option<String>,
    username: Option<String>,
}

impl<'a> GetUserRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, include_deleted: None, key: None, username: None }
    }

    /// whether deleted users should be returned (flag available to users with global ADMIN rights)
    #[must_use]
    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);

        self
    }

    /// user key
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user".to_owned());

        if let Some(value) = &self.include_deleted {
            config.query.push(("includeDeleted".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create user. By default created user will not be notified with email. If password field is not set then password will be randomly generated.
#[derive(Clone)]
pub struct CreateUserRequest<'a> {
    client: &'a crate::core::Client,
    user_write: UserWrite,
}

impl<'a> CreateUserRequest<'a> {
    fn new(client: &'a crate::core::Client, user_write: UserWrite) -> Self {
        Self { client, user_write }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/user".to_owned());

        let body = match serde_json::to_value(&self.user_write)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UserWrite> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Modify user. The 'value' fields present will override the existing value. Fields skipped in request will not be changed.
#[derive(Clone)]
pub struct UpdateUserRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
    username: Option<String>,
    body: UserWrite,
}

impl<'a> UpdateUserRequest<'a> {
    fn new(client: &'a crate::core::Client, body: UserWrite) -> Self {
        Self { client, body, key: None, username: None }
    }

    /// user key
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/user".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UserWrite> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes user and its references (like project roles associations, watches, history). Note: user references will not be removed if multiple User Directories are used and there is a user with the same name existing in another directory (shadowing user).
#[derive(Clone)]
pub struct RemoveUserRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
    username: Option<String>,
}

impl<'a> RemoveUserRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, key: None, username: None }
    }

    /// user key
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/2/user".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Returns available accessibility personal settings along with `enabled` property that indicates the currently logged-in user preference.
#[derive(Clone)]
pub struct GetA11yPersonalSettingsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetA11yPersonalSettingsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/user/a11y/personal-settings".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<A11yPersonalSetting>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Validates user anonymization process.
#[derive(Clone)]
pub struct ValidateUserAnonymizationRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    user_key: Option<String>,
}

impl<'a> ValidateUserAnonymizationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, user_key: None }
    }

    /// Parameter used to include parts of the response.
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The key of the user to validate anonymization for.
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/anonymization".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UserAnonymizationValidation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Schedules a user anonymization process. Requires system admin permission.
#[derive(Clone)]
pub struct ScheduleUserAnonymizationRequest<'a> {
    client: &'a crate::core::Client,
    user_anonymization_request: UserAnonymizationRequest,
}

impl<'a> ScheduleUserAnonymizationRequest<'a> {
    fn new(client: &'a crate::core::Client, user_anonymization_request: UserAnonymizationRequest) -> Self {
        Self { client, user_anonymization_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/user/anonymization".to_owned());

        let body = match serde_json::to_value(&self.user_anonymization_request)? {
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

/// Returns information about a user anonymization operation progress.
#[derive(Clone)]
pub struct GetUserAnonymizationProgressRequest<'a> {
    client: &'a crate::core::Client,
    task_id: Option<i64>,
}

impl<'a> GetUserAnonymizationProgressRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, task_id: None }
    }

    /// The id of a user anonymization task you wish to obtain details on.
    #[must_use]
    pub fn task_id(mut self, value: i64) -> Self {
        self.task_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/user/anonymization/progress".to_owned(),
        );

        if let Some(value) = &self.task_id {
            config.query.push(("taskId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

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

/// Validates user anonymization re-run process.
#[derive(Clone)]
pub struct ValidateUserAnonymizationRerunRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    old_user_key: Option<String>,
    old_user_name: Option<String>,
    user_key: Option<String>,
}

impl<'a> ValidateUserAnonymizationRerunRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, old_user_key: None, old_user_name: None, user_key: None }
    }

    /// Parameter used to include parts of the response.
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// User key before anonymization, only needed when current value is anonymized. If there is no old key, e.g. because the user was already created using the new key generation strategy, provide a value equal to the current key.
    #[must_use]
    pub fn old_user_key(mut self, value: impl Into<String>) -> Self {
        self.old_user_key = Some(value.into());

        self
    }

    /// User name before anonymization, only needed when the current value is anonymized. If there is no old name, provide a value equal to the current name.
    #[must_use]
    pub fn old_user_name(mut self, value: impl Into<String>) -> Self {
        self.old_user_name = Some(value.into());

        self
    }

    /// The key of the user to validate anonymization for.
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/user/anonymization/rerun".to_owned(),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.old_user_key {
            config.query.push(("oldUserKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.old_user_name {
            config.query.push(("oldUserName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UserAnonymizationValidation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Schedules a user anonymization process. Requires system admin permission.
#[derive(Clone)]
pub struct ScheduleUserAnonymizationRerunRequest<'a> {
    client: &'a crate::core::Client,
    user_anonymization_rerun_request: UserAnonymizationRerunRequest,
}

impl<'a> ScheduleUserAnonymizationRerunRequest<'a> {
    fn new(client: &'a crate::core::Client, user_anonymization_rerun_request: UserAnonymizationRerunRequest) -> Self {
        Self { client, user_anonymization_rerun_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/user/anonymization/rerun".to_owned(),
        );

        let body = match serde_json::to_value(&self.user_anonymization_rerun_request)? {
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

/// Removes stale user anonymization task, for scenarios when the node that was executing it is no longer alive. Use it only after making sure that the parent node of the task is actually down, and not just having connectivity issues.
#[derive(Clone)]
pub struct UnlockAnonymizationRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UnlockAnonymizationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            "/rest/api/2/user/anonymization/unlock".to_owned(),
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

/// Add user to given application. Admin permission will be required to perform this operation.
#[derive(Clone)]
pub struct AddUserToApplicationRequest<'a> {
    client: &'a crate::core::Client,
    application_key: Option<String>,
    username: Option<String>,
}

impl<'a> AddUserToApplicationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, application_key: None, username: None }
    }

    /// application key
    #[must_use]
    pub fn application_key(mut self, value: impl Into<String>) -> Self {
        self.application_key = Some(value.into());

        self
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/user/application".to_owned());

        if let Some(value) = &self.application_key {
            config.query.push(("applicationKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Remove user from given application. Admin permission will be required to perform this operation.
#[derive(Clone)]
pub struct RemoveUserFromApplicationRequest<'a> {
    client: &'a crate::core::Client,
    application_key: Option<String>,
    username: Option<String>,
}

impl<'a> RemoveUserFromApplicationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, application_key: None, username: None }
    }

    /// application key
    #[must_use]
    pub fn application_key(mut self, value: impl Into<String>) -> Self {
        self.application_key = Some(value.into());

        self
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/2/user/application".to_owned());

        if let Some(value) = &self.application_key {
            config.query.push(("applicationKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Returns a list of users that match the search string and can be assigned issues for all the given projects.
#[derive(Clone)]
pub struct FindBulkAssignableUsersRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    project_keys: Option<FindBulkAssignableUsersRequestProjectKeys>,
    username: Option<String>,
}

impl<'a> FindBulkAssignableUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, project_keys: None, username: None }
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// the keys of the projects we are finding assignable users for, comma-separated
    #[must_use]
    pub fn project_keys(mut self, value: FindBulkAssignableUsersRequestProjectKeys) -> Self {
        self.project_keys = Some(value);

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/user/assignable/multiProjectSearch".to_owned(),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.project_keys {
            config.query.push(("projectKeys".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users that match the search string. This resource cannot be accessed anonymously. Please note that this resource should be called with an issue key when a list of assignable users is retrieved. For create only a project key should be supplied. The list of assignable users may be incorrect if it's called with the project key for editing.
#[derive(Clone)]
pub struct FindAssignableUsersRequest<'a> {
    client: &'a crate::core::Client,
    issue_key: Option<String>,
    max_results: Option<i64>,
    project: Option<String>,
    action_descriptor_id: Option<i64>,
    username: Option<String>,
}

impl<'a> FindAssignableUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, issue_key: None, max_results: None, project: None, action_descriptor_id: None, username: None }
    }

    /// the issue key for the issue being edited we need to find assignable users for.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    #[must_use]
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());

        self
    }

    #[must_use]
    pub fn action_descriptor_id(mut self, value: i64) -> Self {
        self.action_descriptor_id = Some(value);

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/assignable/search".to_owned());

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.project {
            config.query.push(("project".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.action_descriptor_id {
            config.query.push(("actionDescriptorId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Converts temporary avatar into a real avatar
#[derive(Clone)]
pub struct CreateUserAvatarFromTemporaryRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
    avatar_cropping: AvatarCropping,
}

impl<'a> CreateUserAvatarFromTemporaryRequest<'a> {
    fn new(client: &'a crate::core::Client, avatar_cropping: AvatarCropping) -> Self {
        Self { client, avatar_cropping, username: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/user/avatar".to_owned());

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.avatar_cropping)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Avatar> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the avatar for the user.
#[derive(Clone)]
pub struct UpdateUserAvatarRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
    avatar: Avatar,
}

impl<'a> UpdateUserAvatarRequest<'a> {
    fn new(client: &'a crate::core::Client, avatar: Avatar) -> Self {
        Self { client, avatar, username: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/user/avatar".to_owned());

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.avatar)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Avatar> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates temporary avatar using multipart. The response is sent back as JSON stored in a textarea. This is because the client uses remote iframing to submit avatars using multipart. So we must send them a valid HTML page back from which the client parses the JSON from.
/// Creating a temporary avatar is part of a 3-step process in uploading a new avatar for a user: upload, crop, confirm. This endpoint allows you to use a multipart upload instead of sending the image directly as the request body.
/// You *must* use "avatar" as the name of the upload parameter:
/// curl -c cookiejar.txt -X POST -u admin:admin -H "X-Atlassian-Token: no-check" \
///   -F "avatar=@mynewavatar.png;type=image/png" \
///   '<http://localhost:8090/jira/rest/api/2/user/avatar/temporary?username=admin>'
#[derive(Clone)]
pub struct StoreTemporaryUserAvatarUsingMultiPartRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
    avatar: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> StoreTemporaryUserAvatarUsingMultiPartRequest<'a> {
    fn new(client: &'a crate::core::Client, avatar: impl IntoIterator<Item = crate::core::Attachment>) -> Self {
        Self { client, avatar: avatar.into_iter().collect(), username: None, content_type: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The media type of the bytes being sent, e.g. `image/png`.
    #[must_use]
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/user/avatar/temporary".to_owned());

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body =
            Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("avatar", self.avatar.clone())));

        config.content_type = self.content_type.clone().or(None);

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

/// Deletes avatar
#[derive(Clone)]
pub struct DeleteUserAvatarRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    username: Option<String>,
}

impl<'a> DeleteUserAvatarRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, username: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/user/avatar/{}", self.id),
        );

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Returns all avatars which are visible for the currently logged in user.
#[derive(Clone)]
pub struct GetAllUserAvatarsRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
}

impl<'a> GetAllUserAvatarsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, username: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/avatars".to_owned());

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetAllUserAvatars> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the default columns for the given user. Admin permission will be required to get columns for a user other than the currently logged in user.
#[derive(Clone)]
pub struct DefaultColumnsRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
}

impl<'a> DefaultColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, username: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/columns".to_owned());

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnOptions>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default columns for the given user. Admin permission will be required to get columns for a user other than the currently logged in user.
#[derive(Clone)]
pub struct SetColumnsUrlEncodedRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
    columns: Option<Vec<String>>,
}

impl<'a> SetColumnsUrlEncodedRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, username: None, columns: None }
    }

    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    #[must_use]
    pub fn columns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.columns = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/user/columns".to_owned());

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.username {
            body.insert("username".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.columns {
            body.insert("columns".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        config.content_type = Some("application/x-www-form-urlencoded".to_owned());

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

/// Reset the default columns for the given user to the system default. Admin permission will be required to get columns for a user other than the currently logged in user.
#[derive(Clone)]
pub struct ResetUserColumnsRequest<'a> {
    client: &'a crate::core::Client,
    username: Option<String>,
}

impl<'a> ResetUserColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, username: None }
    }

    /// username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/2/user/columns".to_owned());

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Returns a list of users that match the search string. This resource cannot be accessed anonymously.
/// Duplicated means that the user has an account in more than one directory
/// and either more than one account is active or the only active account does not belong to the directory
/// with the highest priority.
/// The data returned by this endpoint is cached for 10 minutes and the cache is flushed when any User Directory
/// is added, removed, enabled, disabled, or synchronized.
/// A System Administrator can also flush the cache manually.
/// Related JAC ticket: <https://jira.atlassian.com/browse/JRASERVER-68797>
#[derive(Clone)]
pub struct GetDuplicatedUsersCountRequest<'a> {
    client: &'a crate::core::Client,
    flush: Option<bool>,
}

impl<'a> GetDuplicatedUsersCountRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, flush: None }
    }

    /// if set to true forces cache flush, user must be sysadmin for this parameter to have an effect.
    #[must_use]
    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/duplicated/count".to_owned());

        if let Some(value) = &self.flush {
            config.query.push(("flush".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns duplicated users mapped to their directories with an indication if their accounts are active or not.
/// Duplicated means that the user has an account in more than one directory and either more than one account is active
/// or the only active account does not belong to the directory with the highest priority.
/// The data returned by this endpoint is cached for 10 minutes and the cache is flushed when any User Directory
/// is added, removed, enabled, disabled, or synchronized.
/// A System Administrator can also flush the cache manually.
/// Related JAC ticket: <https://jira.atlassian.com/browse/JRASERVER-68797>
#[derive(Clone)]
pub struct GetDuplicatedUsersMappingRequest<'a> {
    client: &'a crate::core::Client,
    flush: Option<bool>,
}

impl<'a> GetDuplicatedUsersMappingRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, flush: None }
    }

    /// if set to true forces cache flush, user must be sysadmin for this parameter to have an effect.
    #[must_use]
    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/duplicated/list".to_owned());

        if let Some(value) = &self.flush {
            config.query.push(("flush".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Avatar> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of all users. This resource cannot be accessed anonymously.
/// This Api is a streaming-like endpoint. For performance and security  reasons, it is not indicating the total
/// number of users available in the system. The first call should be done without the cursor parameter.
/// Subsequent calls should use the value of the next cursor returned in the previous call. Specific values of
/// cursor are not guaranteed to be valid in the future and are not part of the API, so they should not be used
/// as a key for caching or storing data. The order in which the users are returned is not defined. It is guaranteed
/// that the same user will not be returned twice in the sequence of calls. For resiliency reason this endpoint
/// never returns 404 code, even if called with a cursor parameter that was not returned in the previous call.
///
///
/// Available since Jira Data Center 11.0, and in 10.3 LTS.
#[derive(Clone)]
pub struct GetUserListRequest<'a> {
    client: &'a crate::core::Client,
    cursor: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetUserListRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, cursor: None, max_results: None }
    }

    /// The position in the stream to continue iterating over all users.
    #[must_use]
    pub fn cursor(mut self, value: i64) -> Self {
        self.cursor = Some(value);

        self
    }

    /// The maximum number of users to return per page (defaults to 2000).
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/list".to_owned());

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StreamPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Modify user password.
#[derive(Clone)]
pub struct ChangeUserPasswordRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
    username: Option<String>,
    password: Password,
}

impl<'a> ChangeUserPasswordRequest<'a> {
    fn new(client: &'a crate::core::Client, password: Password) -> Self {
        Self { client, password, key: None, username: None }
    }

    /// user key
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/user/password".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.password)? {
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

/// Returns a list of users matching query with highlighting.
#[derive(Clone)]
pub struct FindUsersForPickerRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    query: Option<String>,
    exclude: Option<Vec<String>>,
    show_avatar: Option<bool>,
}

impl<'a> FindUsersForPickerRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, query: None, exclude: None, show_avatar: None }
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// A string used to search username, Name or e-mail address
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// List of users to be excluded from the search results
    #[must_use]
    pub fn exclude(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// If true, then avatars are included in the results
    #[must_use]
    pub fn show_avatar(mut self, value: bool) -> Self {
        self.show_avatar = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/picker".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.exclude {
            config.query.push(("exclude".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.show_avatar {
            config.query.push(("showAvatar".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UserPickerResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the keys of all properties for the user identified by the key or by the id.
#[derive(Clone)]
pub struct GetUserPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    user_key: Option<String>,
    username: Option<String>,
}

impl<'a> GetUserPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, user_key: None, username: None }
    }

    /// Key of the user whose properties are to be returned
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose properties are to be returned
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/properties".to_owned());

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityPropertiesKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the user identified by the key or by the id.
#[derive(Clone)]
pub struct GetUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    user_key: Option<String>,
    username: Option<String>,
}

impl<'a> GetUserPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into(), user_key: None, username: None }
    }

    /// Key of the user whose property is to be returned
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose property is to be returned
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of the specified user's property.
/// You can use this resource to store a custom data against the user identified by the key or by the id. The user
/// who stores the data is required to have permissions to administer the user.
#[derive(Clone)]
pub struct SetUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    user_key: Option<String>,
    username: Option<String>,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetUserPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, property_key: property_key.into(), body, user_key: None, username: None }
    }

    /// Key of the user whose property is to be set
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose property is to be set
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Removes the property from the user identified by the key or by the id. The user who removes the property is required to have permissions to administer the user.
#[derive(Clone)]
pub struct DeleteUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    user_key: Option<String>,
    username: Option<String>,
}

impl<'a> DeleteUserPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into(), user_key: None, username: None }
    }

    /// Key of the user whose property is to be removed
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose property is to be removed
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Finds users.
#[derive(Clone)]
pub struct FindUsersRequest<'a> {
    client: &'a crate::core::Client,
    include_inactive: Option<bool>,
    max_results: Option<i64>,
    include_active: Option<bool>,
    start_at: Option<i64>,
    username: Option<String>,
}

impl<'a> FindUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, include_inactive: None, max_results: None, include_active: None, start_at: None, username: None }
    }

    /// If true, then inactive users are included in the results (default false)
    #[must_use]
    pub fn include_inactive(mut self, value: bool) -> Self {
        self.include_inactive = Some(value);

        self
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// If true, then active users are included in the results (default true)
    #[must_use]
    pub fn include_active(mut self, value: bool) -> Self {
        self.include_active = Some(value);

        self
    }

    /// The index of the first user to return (0-based). Please note that the startAt parameter will be deprecated in a future release of Jira 10.3.x
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// A query string used to search username, name or e-mail address
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/search".to_owned());

        if let Some(value) = &self.include_inactive {
            config.query.push(("includeInactive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.include_active {
            config.query.push(("includeActive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Invalidates session of given user.
#[derive(Clone)]
pub struct DeleteSessionRequest<'a> {
    client: &'a crate::core::Client,
    username: String,
}

impl<'a> DeleteSessionRequest<'a> {
    fn new(client: &'a crate::core::Client, username: impl Into<String>) -> Self {
        Self { client, username: username.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/user/session/{}", crate::core::encode_path_segment(&self.username)),
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

/// Returns a list of active users that match the search string. This resource cannot be accessed anonymously and requires the Browse Users global permission. Given an issue key this resource will provide a list of users that match the search string and have the browse issue permission for the issue provided.
#[derive(Clone)]
pub struct FindUsersWithBrowsePermissionRequest<'a> {
    client: &'a crate::core::Client,
    project_key: Option<String>,
    issue_key: Option<String>,
    max_results: Option<i64>,
    username: Option<String>,
}

impl<'a> FindUsersWithBrowsePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, project_key: None, issue_key: None, max_results: None, username: None }
    }

    /// the optional project key to search for users with if no issueKey is supplied.
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// the issue key for the issue being edited we need to find viewable users for.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// the username filter, no users returned if left blank
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/viewissue/search".to_owned());

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
