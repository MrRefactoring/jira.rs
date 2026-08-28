// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetNotificationSchemesRequestExpandValue {
        All => "all",
        Field => "field",
        Group => "group",
        NotificationSchemeEvents => "notificationSchemeEvents",
        ProjectRole => "projectRole",
        User => "user",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `all` Returns all expandable information
///  *  `field` Returns information about any custom fields assigned to receive an event
///  *  `group` Returns information about any groups assigned to receive an event
///  *  `notificationSchemeEvents` Returns a list of event associations. This list is returned for all expandable information
///  *  `projectRole` Returns information about any project roles assigned to receive an event
///  *  `user` Returns information about any users assigned to receive an event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetNotificationSchemesRequestExpand {
    One(GetNotificationSchemesRequestExpandValue),
    Many(Vec<GetNotificationSchemesRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetNotificationSchemeRequestExpandValue {
        All => "all",
        Field => "field",
        Group => "group",
        NotificationSchemeEvents => "notificationSchemeEvents",
        ProjectRole => "projectRole",
        User => "user",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `all` Returns all expandable information
///  *  `field` Returns information about any custom fields assigned to receive an event
///  *  `group` Returns information about any groups assigned to receive an event
///  *  `notificationSchemeEvents` Returns a list of event associations. This list is returned for all expandable information
///  *  `projectRole` Returns information about any project roles assigned to receive an event
///  *  `user` Returns information about any users assigned to receive an event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetNotificationSchemeRequestExpand {
    One(GetNotificationSchemeRequestExpandValue),
    Many(Vec<GetNotificationSchemeRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The IssueNotificationSchemes operations.
pub struct IssueNotificationSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueNotificationSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of [notification schemes](https://confluence.atlassian.com/x/8YdKLg) ordered by the display name.
    ///
    /// *Note that you should allow for events without recipients to appear in responses.*
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, the user must have permission to administer at least one project associated with a notification scheme for it to be returned.
    pub fn get_notification_schemes(&self) -> GetNotificationSchemesRequest<'a> {
        GetNotificationSchemesRequest::new(self.client)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) mapping of project that have notification scheme assigned. You can provide either one or multiple notification scheme IDs or project IDs to filter by. If you don't provide any, this will return a list of all mappings. Note that only company-managed (classic) projects are supported. This is because team-managed projects don't have a concept of a default notification scheme. The mappings are ordered by projectId.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_notification_scheme_to_project_mappings(&self) -> GetNotificationSchemeToProjectMappingsRequest<'a> {
        GetNotificationSchemeToProjectMappingsRequest::new(self.client)
    }

    /// Returns a [notification scheme](https://confluence.atlassian.com/x/8YdKLg), including the list of events and the recipients who will receive notifications for those events.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, the user must have permission to administer at least one project associated with the notification scheme.
    pub fn get_notification_scheme(&self, id: i64) -> GetNotificationSchemeRequest<'a> {
        GetNotificationSchemeRequest::new(self.client, id)
    }

    /// Adds notifications to a notification scheme. You can add up to 1000 notifications per request.
    ///
    /// *Deprecated: The notification type `EmailAddress` is no longer supported in Cloud. Refer to the [changelog](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-1031) for more details.*
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn add_notifications(
        &self,
        id: impl Into<String>,
        add_notifications_details: AddNotificationsDetails,
    ) -> AddNotificationsRequest<'a> {
        AddNotificationsRequest::new(self.client, id, add_notifications_details)
    }

    /// Removes a notification from a notification scheme.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn remove_notification_from_notification_scheme(
        &self,
        notification_scheme_id: impl Into<String>,
        notification_id: impl Into<String>,
    ) -> RemoveNotificationFromNotificationSchemeRequest<'a> {
        RemoveNotificationFromNotificationSchemeRequest::new(self.client, notification_scheme_id, notification_id)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of [notification schemes](https://confluence.atlassian.com/x/8YdKLg) ordered by the display name.
///
/// *Note that you should allow for events without recipients to appear in responses.*
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, the user must have permission to administer at least one project associated with a notification scheme for it to be returned.
#[derive(Clone)]
pub struct GetNotificationSchemesRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<String>,
    max_results: Option<String>,
    id: Option<Vec<String>>,
    project_id: Option<Vec<String>>,
    only_default: Option<bool>,
    expand: Option<GetNotificationSchemesRequestExpand>,
}

impl<'a> GetNotificationSchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None, id: None, project_id: None, only_default: None, expand: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: impl Into<String>) -> Self {
        self.start_at = Some(value.into());

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: impl Into<String>) -> Self {
        self.max_results = Some(value.into());

        self
    }

    /// The list of notification schemes IDs to be filtered by
    #[must_use]
    pub fn id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of projects IDs to be filtered by
    #[must_use]
    pub fn project_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.project_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// When set to true, returns only the default notification scheme. If you provide project IDs not associated with the default, returns an empty page. The default value is false.
    #[must_use]
    pub fn only_default(mut self, value: bool) -> Self {
        self.only_default = Some(value);

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `all` Returns all expandable information
    ///  *  `field` Returns information about any custom fields assigned to receive an event
    ///  *  `group` Returns information about any groups assigned to receive an event
    ///  *  `notificationSchemeEvents` Returns a list of event associations. This list is returned for all expandable information
    ///  *  `projectRole` Returns information about any project roles assigned to receive an event
    ///  *  `user` Returns information about any users assigned to receive an event
    #[must_use]
    pub fn expand(mut self, value: GetNotificationSchemesRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/notificationscheme".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.id {
            config.query.push(("id".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.only_default {
            config.query.push(("onlyDefault".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<NotificationScheme>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) mapping of project that have notification scheme assigned. You can provide either one or multiple notification scheme IDs or project IDs to filter by. If you don't provide any, this will return a list of all mappings. Note that only company-managed (classic) projects are supported. This is because team-managed projects don't have a concept of a default notification scheme. The mappings are ordered by projectId.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetNotificationSchemeToProjectMappingsRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<String>,
    max_results: Option<String>,
    notification_scheme_id: Option<Vec<String>>,
    project_id: Option<Vec<String>>,
}

impl<'a> GetNotificationSchemeToProjectMappingsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None, notification_scheme_id: None, project_id: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: impl Into<String>) -> Self {
        self.start_at = Some(value.into());

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: impl Into<String>) -> Self {
        self.max_results = Some(value.into());

        self
    }

    /// The list of notifications scheme IDs to be filtered out
    #[must_use]
    pub fn notification_scheme_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.notification_scheme_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of project IDs to be filtered out
    #[must_use]
    pub fn project_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.project_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/notificationscheme/project".to_owned(),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.notification_scheme_id {
            config.query.push(("notificationSchemeId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<NotificationSchemeAndProjectMapping>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [notification scheme](https://confluence.atlassian.com/x/8YdKLg), including the list of events and the recipients who will receive notifications for those events.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, the user must have permission to administer at least one project associated with the notification scheme.
#[derive(Clone)]
pub struct GetNotificationSchemeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    expand: Option<GetNotificationSchemeRequestExpand>,
}

impl<'a> GetNotificationSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `all` Returns all expandable information
    ///  *  `field` Returns information about any custom fields assigned to receive an event
    ///  *  `group` Returns information about any groups assigned to receive an event
    ///  *  `notificationSchemeEvents` Returns a list of event associations. This list is returned for all expandable information
    ///  *  `projectRole` Returns information about any project roles assigned to receive an event
    ///  *  `user` Returns information about any users assigned to receive an event
    #[must_use]
    pub fn expand(mut self, value: GetNotificationSchemeRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/notificationscheme/{}", self.id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<NotificationScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds notifications to a notification scheme. You can add up to 1000 notifications per request.
///
/// *Deprecated: The notification type `EmailAddress` is no longer supported in Cloud. Refer to the [changelog](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-1031) for more details.*
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct AddNotificationsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    add_notifications_details: AddNotificationsDetails,
}

impl<'a> AddNotificationsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        id: impl Into<String>,
        add_notifications_details: AddNotificationsDetails,
    ) -> Self {
        Self { client, id: id.into(), add_notifications_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/notificationscheme/{}/notification", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.add_notifications_details)? {
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

/// Removes a notification from a notification scheme.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct RemoveNotificationFromNotificationSchemeRequest<'a> {
    client: &'a crate::core::Client,
    notification_scheme_id: String,
    notification_id: String,
}

impl<'a> RemoveNotificationFromNotificationSchemeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        notification_scheme_id: impl Into<String>,
        notification_id: impl Into<String>,
    ) -> Self {
        Self { client, notification_scheme_id: notification_scheme_id.into(), notification_id: notification_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/notificationscheme/{}/notification/{}",
                crate::core::encode_path_segment(&self.notification_scheme_id),
                crate::core::encode_path_segment(&self.notification_id)
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
