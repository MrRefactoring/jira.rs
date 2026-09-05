// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetCurrentUserRequestExpandValue {
        Groups => "groups",
        ApplicationRoles => "applicationRoles",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about user in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `groups` Returns all groups, including nested groups, the user belongs to.
///  *  `applicationRoles` Returns the application roles the user is assigned to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetCurrentUserRequestExpand {
    One(GetCurrentUserRequestExpandValue),
    Many(Vec<GetCurrentUserRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Myself operations.
pub struct MyselfService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> MyselfService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the value of a preference of the current user.
    ///
    /// Note that these keys are deprecated:
    ///
    ///  *  *jira.user.locale* The locale of the user. By default this is not set and the user takes the locale of the instance.
    ///  *  *jira.user.timezone* The time zone of the user. By default this is not set and the user takes the timezone of the instance.
    ///
    /// These system preferences keys will be deprecated by 15/07/2024. You can still retrieve these keys, but it will not have any impact on Notification behaviour.
    ///
    ///  *  *user.notifications.watcher* Whether the user gets notified when they are watcher.
    ///  *  *user.notifications.assignee* Whether the user gets notified when they are assignee.
    ///  *  *user.notifications.reporter* Whether the user gets notified when they are reporter.
    ///  *  *user.notifications.mentions* Whether the user gets notified when they are mentions.
    ///
    /// Use [ Update a user profile](https://developer.atlassian.com/cloud/admin/user-management/rest/#api-users-account-id-manage-profile-patch) from the user management REST API to manage timezone and locale instead.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_preference(&self, key: impl Into<String>) -> GetPreferenceRequest<'a> {
        GetPreferenceRequest::new(self.client, key)
    }

    /// Creates a preference for the user or updates a preference's value by sending a plain text string. For example, `false`. An arbitrary preference can be created with the value containing up to 255 characters. In addition, the following keys define system preferences that can be set or created:
    ///
    ///  *  *user.notifications.mimetype* The mime type used in notifications sent to the user. Defaults to `html`.
    ///  *  *user.default.share.private* Whether new [ filters](https://confluence.atlassian.com/x/eQiiLQ) are set to private. Defaults to `true`.
    ///  *  *user.keyboard.shortcuts.disabled* Whether keyboard shortcuts are disabled. Defaults to `false`.
    ///  *  *user.autowatch.disabled* Whether the user automatically watches issues they create or add a comment to. By default, not set: the user takes the instance autowatch setting.
    ///  *  *user.notifiy.own.changes* Whether the user gets notified of their own changes.
    ///
    /// Note that these keys are deprecated:
    ///
    ///  *  *jira.user.locale* The locale of the user. By default, not set. The user takes the instance locale.
    ///  *  *jira.user.timezone* The time zone of the user. By default, not set. The user takes the instance timezone.
    ///
    /// These system preferences keys will be deprecated by 15/07/2024. You can still use these keys to create arbitrary preferences, but it will not have any impact on Notification behaviour.
    ///
    ///  *  *user.notifications.watcher* Whether the user gets notified when they are watcher.
    ///  *  *user.notifications.assignee* Whether the user gets notified when they are assignee.
    ///  *  *user.notifications.reporter* Whether the user gets notified when they are reporter.
    ///  *  *user.notifications.mentions* Whether the user gets notified when they are mentions.
    ///
    /// Use [ Update a user profile](https://developer.atlassian.com/cloud/admin/user-management/rest/#api-users-account-id-manage-profile-patch) from the user management REST API to manage timezone and locale instead.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn set_preference(&self, key: impl Into<String>, body: impl Into<String>) -> SetPreferenceRequest<'a> {
        SetPreferenceRequest::new(self.client, key, body)
    }

    /// Deletes a preference of the user, which restores the default value of system defined settings.
    ///
    /// Note that these keys are deprecated:
    ///
    ///  *  *jira.user.locale* The locale of the user. By default, not set. The user takes the instance locale.
    ///  *  *jira.user.timezone* The time zone of the user. By default, not set. The user takes the instance timezone.
    ///
    /// Use [ Update a user profile](https://developer.atlassian.com/cloud/admin/user-management/rest/#api-users-account-id-manage-profile-patch) from the user management REST API to manage timezone and locale instead.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn remove_preference(&self, key: impl Into<String>) -> RemovePreferenceRequest<'a> {
        RemovePreferenceRequest::new(self.client, key)
    }

    /// Returns the locale for the user.
    ///
    /// If the user has no language preference set (which is the default setting) or this resource is accessed anonymous, the browser locale detected by Jira is returned. Jira detects the browser locale using the *Accept-Language* header in the request. However, if this doesn't match a locale available Jira, the site default locale is returned.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_locale(&self) -> GetLocaleRequest<'a> {
        GetLocaleRequest::new(self.client)
    }

    /// Returns details for the current user.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_current_user(&self) -> GetCurrentUserRequest<'a> {
        GetCurrentUserRequest::new(self.client)
    }
}

/// Returns the value of a preference of the current user.
///
/// Note that these keys are deprecated:
///
///  *  *jira.user.locale* The locale of the user. By default this is not set and the user takes the locale of the instance.
///  *  *jira.user.timezone* The time zone of the user. By default this is not set and the user takes the timezone of the instance.
///
/// These system preferences keys will be deprecated by 15/07/2024. You can still retrieve these keys, but it will not have any impact on Notification behaviour.
///
///  *  *user.notifications.watcher* Whether the user gets notified when they are watcher.
///  *  *user.notifications.assignee* Whether the user gets notified when they are assignee.
///  *  *user.notifications.reporter* Whether the user gets notified when they are reporter.
///  *  *user.notifications.mentions* Whether the user gets notified when they are mentions.
///
/// Use [ Update a user profile](https://developer.atlassian.com/cloud/admin/user-management/rest/#api-users-account-id-manage-profile-patch) from the user management REST API to manage timezone and locale instead.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetPreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: String,
}

impl<'a> GetPreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client, key: impl Into<String>) -> Self {
        Self { client, key: key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/mypreferences".to_owned());

        config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(self.key.clone())));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<String> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a preference for the user or updates a preference's value by sending a plain text string. For example, `false`. An arbitrary preference can be created with the value containing up to 255 characters. In addition, the following keys define system preferences that can be set or created:
///
///  *  *user.notifications.mimetype* The mime type used in notifications sent to the user. Defaults to `html`.
///  *  *user.default.share.private* Whether new [ filters](https://confluence.atlassian.com/x/eQiiLQ) are set to private. Defaults to `true`.
///  *  *user.keyboard.shortcuts.disabled* Whether keyboard shortcuts are disabled. Defaults to `false`.
///  *  *user.autowatch.disabled* Whether the user automatically watches issues they create or add a comment to. By default, not set: the user takes the instance autowatch setting.
///  *  *user.notifiy.own.changes* Whether the user gets notified of their own changes.
///
/// Note that these keys are deprecated:
///
///  *  *jira.user.locale* The locale of the user. By default, not set. The user takes the instance locale.
///  *  *jira.user.timezone* The time zone of the user. By default, not set. The user takes the instance timezone.
///
/// These system preferences keys will be deprecated by 15/07/2024. You can still use these keys to create arbitrary preferences, but it will not have any impact on Notification behaviour.
///
///  *  *user.notifications.watcher* Whether the user gets notified when they are watcher.
///  *  *user.notifications.assignee* Whether the user gets notified when they are assignee.
///  *  *user.notifications.reporter* Whether the user gets notified when they are reporter.
///  *  *user.notifications.mentions* Whether the user gets notified when they are mentions.
///
/// Use [ Update a user profile](https://developer.atlassian.com/cloud/admin/user-management/rest/#api-users-account-id-manage-profile-patch) from the user management REST API to manage timezone and locale instead.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct SetPreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: String,
    body: String,
}

impl<'a> SetPreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client, key: impl Into<String>, body: impl Into<String>) -> Self {
        Self { client, key: key.into(), body: body.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/mypreferences".to_owned());

        config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(self.key.clone())));

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

/// Deletes a preference of the user, which restores the default value of system defined settings.
///
/// Note that these keys are deprecated:
///
///  *  *jira.user.locale* The locale of the user. By default, not set. The user takes the instance locale.
///  *  *jira.user.timezone* The time zone of the user. By default, not set. The user takes the instance timezone.
///
/// Use [ Update a user profile](https://developer.atlassian.com/cloud/admin/user-management/rest/#api-users-account-id-manage-profile-patch) from the user management REST API to manage timezone and locale instead.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct RemovePreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: String,
}

impl<'a> RemovePreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client, key: impl Into<String>) -> Self {
        Self { client, key: key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/3/mypreferences".to_owned());

        config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(self.key.clone())));

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

/// Returns the locale for the user.
///
/// If the user has no language preference set (which is the default setting) or this resource is accessed anonymous, the browser locale detected by Jira is returned. Jira detects the browser locale using the *Accept-Language* header in the request. However, if this doesn't match a locale available Jira, the site default locale is returned.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetLocaleRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetLocaleRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/mypreferences/locale".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Locale> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns details for the current user.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetCurrentUserRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetCurrentUserRequestExpand>,
}

impl<'a> GetCurrentUserRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about user in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `groups` Returns all groups, including nested groups, the user belongs to.
    ///  *  `applicationRoles` Returns the application roles the user is assigned to.
    #[must_use]
    pub fn expand(mut self, value: GetCurrentUserRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/myself".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<DashboardUser> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
