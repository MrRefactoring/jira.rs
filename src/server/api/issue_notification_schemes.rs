// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueNotificationSchemes operations.
pub struct IssueNotificationSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueNotificationSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a paginated list of notification schemes. In order to access notification scheme, the calling user is
    /// required to have permissions to administer at least one project associated with the requested notification scheme. Each scheme contains
    /// a list of events and recipient configured to receive notifications for these events. Consumer should allow events without recipients to appear in response.
    /// The list is ordered by the scheme's name.
    /// Follow the documentation of /notificationscheme/{id} resource for all details about returned value.
    pub fn get_notification_schemes(&self) -> GetNotificationSchemesRequest<'a> {
        GetNotificationSchemesRequest::new(self.client)
    }

    /// Returns a full representation of the notification scheme for the given id. This resource will return a
    /// notification scheme containing a list of events and recipient configured to receive notifications for these events. Consumer
    /// should allow events without recipients to appear in response. User accessing
    /// the data is required to have permissions to administer at least one project associated with the requested notification scheme.
    /// Notification recipients can be:
    /// - current assignee - the value of the notificationType is CurrentAssignee
    /// - issue reporter - the value of the notificationType is Reporter
    /// - current user - the value of the notificationType is CurrentUser
    /// - project lead - the value of the notificationType is ProjectLead
    /// - component lead - the value of the notificationType is ComponentLead
    /// - all watchers - the value of the notification type is AllWatchers
    ///
    /// - configured user - the value of the notification type is User. Parameter will contain key of the user. Information about the user will be provided
    /// if **user** expand parameter is used.
    /// - configured group - the value of the notification type is Group. Parameter will contain name of the group. Information about the group will be provided
    /// if **group** expand parameter is used.
    /// - configured email address - the value of the notification type is EmailAddress, additionally information about the email will be provided.
    /// - users or users in groups in the configured custom fields - the value of the notification type is UserCustomField or GroupCustomField. Parameter
    /// will contain id of the custom field. Information about the field will be provided if **field** expand parameter is used.
    /// - configured project role - the value of the notification type is ProjectRole. Parameter will contain project role id. Information about the project role
    /// will be provided if **projectRole** expand parameter is used.
    /// Please see the example for reference.
    /// The events can be Jira system events or events configured by administrator. In case of the system events, data about theirs
    /// ids, names and descriptions is provided. In case of custom events, the template event is included as well.
    pub fn get_notification_scheme(&self, id: i64) -> GetNotificationSchemeRequest<'a> {
        GetNotificationSchemeRequest::new(self.client, id)
    }
}

/// Returns a paginated list of notification schemes. In order to access notification scheme, the calling user is
/// required to have permissions to administer at least one project associated with the requested notification scheme. Each scheme contains
/// a list of events and recipient configured to receive notifications for these events. Consumer should allow events without recipients to appear in response.
/// The list is ordered by the scheme's name.
/// Follow the documentation of /notificationscheme/{id} resource for all details about returned value.
pub struct GetNotificationSchemesRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    max_results: Option<i64>,
    start_at: Option<i64>,
}

impl<'a> GetNotificationSchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, max_results: None, start_at: None }
    }

    /// Optional information to be expanded in the response: group, user, projectRole or field.
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The maximum number of notification schemes to return (max 50).
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The index of the first notification scheme to return (0 based).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/notificationscheme".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PagedResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a full representation of the notification scheme for the given id. This resource will return a
/// notification scheme containing a list of events and recipient configured to receive notifications for these events. Consumer
/// should allow events without recipients to appear in response. User accessing
/// the data is required to have permissions to administer at least one project associated with the requested notification scheme.
/// Notification recipients can be:
/// - current assignee - the value of the notificationType is CurrentAssignee
/// - issue reporter - the value of the notificationType is Reporter
/// - current user - the value of the notificationType is CurrentUser
/// - project lead - the value of the notificationType is ProjectLead
/// - component lead - the value of the notificationType is ComponentLead
/// - all watchers - the value of the notification type is AllWatchers
///
/// - configured user - the value of the notification type is User. Parameter will contain key of the user. Information about the user will be provided
/// if **user** expand parameter is used.
/// - configured group - the value of the notification type is Group. Parameter will contain name of the group. Information about the group will be provided
/// if **group** expand parameter is used.
/// - configured email address - the value of the notification type is EmailAddress, additionally information about the email will be provided.
/// - users or users in groups in the configured custom fields - the value of the notification type is UserCustomField or GroupCustomField. Parameter
/// will contain id of the custom field. Information about the field will be provided if **field** expand parameter is used.
/// - configured project role - the value of the notification type is ProjectRole. Parameter will contain project role id. Information about the project role
/// will be provided if **projectRole** expand parameter is used.
/// Please see the example for reference.
/// The events can be Jira system events or events configured by administrator. In case of the system events, data about theirs
/// ids, names and descriptions is provided. In case of custom events, the template event is included as well.
pub struct GetNotificationSchemeRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    id: i64,
}

impl<'a> GetNotificationSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, expand: None }
    }

    /// Optional information to be expanded in the response: group, user, projectRole or field.
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/notificationscheme/{}", self.id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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
