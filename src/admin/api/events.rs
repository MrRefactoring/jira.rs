// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    pub enum GetEventsRequestProduct {
        Bitbucket => "bitbucket",
        Confluence => "confluence",
        GuardDetect => "guard_detect",
        Jira => "jira",
        Loom => "loom",
    }
}

crate::open_enum! {
    /// The order used to sort events by processing time. Defaults to ascending.
    pub enum PollEventsRequestSortOrder {
        Asc => "asc",
        Desc => "desc",
    }
}

/// The Events operations.
pub struct EventsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> EventsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a filtered list of audit log events for an organization.
    /// Use this endpoint for more granular and detailed querying.
    ///
    /// If you simply need to paginate through all events, consider using the [/events-stream](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-events/#api-v1-orgs-orgid-events-stream-get) endpoint.
    ///
    /// These rate limits for this endpoint be lowered effective end of May 2025 as follows:
    ///  - *Rate limit per user*: *10* requests per minute
    ///  - *Rate limit per API path*: *10* requests per minute
    ///
    ///  Please migrate to the polling API to guarantee uninterrupted service for use cases involving a high request rate.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
    pub fn get_events(&self, org_id: impl Into<String>) -> GetEventsRequest<'a> {
        GetEventsRequest::new(self.client, org_id)
    }

    /// Returns a paginated list of audit logs events for an organization. Use this endpoint if you want to retrieve events in a simple, paginated manner with time-based filtering.
    ///
    /// If you need more advanced filtering, refer to the [/events](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-events/#api-v1-orgs-orgid-events-get) endpoint.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
    pub fn poll_events(&self, org_id: impl Into<String>) -> PollEventsRequest<'a> {
        PollEventsRequest::new(self.client, org_id)
    }

    /// Returns information about a single event by ID.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
    pub fn get_event_by_id(&self, org_id: impl Into<String>, event_id: impl Into<String>) -> GetEventByIdRequest<'a> {
        GetEventByIdRequest::new(self.client, org_id, event_id)
    }

    /// Returns information localized event actions
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
    pub fn get_event_actions(&self, org_id: impl Into<String>) -> GetEventActionsRequest<'a> {
        GetEventActionsRequest::new(self.client, org_id)
    }
}

/// Returns a filtered list of audit log events for an organization.
/// Use this endpoint for more granular and detailed querying.
///
/// If you simply need to paginate through all events, consider using the [/events-stream](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-events/#api-v1-orgs-orgid-events-stream-get) endpoint.
///
/// These rate limits for this endpoint be lowered effective end of May 2025 as follows:
///  - *Rate limit per user*: *10* requests per minute
///  - *Rate limit per API path*: *10* requests per minute
///
///  Please migrate to the polling API to guarantee uninterrupted service for use cases involving a high request rate.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
#[derive(Clone)]
pub struct GetEventsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    cursor: Option<String>,
    q: Option<String>,
    from: Option<String>,
    to: Option<String>,
    action: Option<String>,
    actor: Option<Vec<String>>,
    ip: Option<Vec<String>>,
    product: Option<Vec<GetEventsRequestProduct>>,
    location: Option<String>,
    limit: Option<i64>,
}

impl<'a> GetEventsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            cursor: None,
            q: None,
            from: None,
            to: None,
            action: None,
            actor: None,
            ip: None,
            product: None,
            location: None,
            limit: None,
        }
    }

    /// Sets the starting point for the page of results to return
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// Single query term for searching events.
    #[must_use]
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());

        self
    }

    /// The earliest date and time of the event represented as a UNIX epoch time in milliseconds.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());

        self
    }

    /// The latest date and time of the event represented as a UNIX epoch time in milliseconds.
    #[must_use]
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());

        self
    }

    /// A query filter that returns events of a specific action type.
    #[must_use]
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());

        self
    }

    /// A query filter that returns events by one or more specific actors.
    #[must_use]
    pub fn actor(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.actor = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A query filter that returns events by one or more specific ip addresses.
    #[must_use]
    pub fn ip(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ip = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A query filter that returns events by one or more specific products.
    #[must_use]
    pub fn product(mut self, value: impl IntoIterator<Item = impl Into<GetEventsRequestProduct>>) -> Self {
        self.product = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A query filter that returns events by one or more specific locations. Of format: \[ { "city": "&lt;CityName&gt;", "countryName": "&lt;CountryName&gt;" }, ... \]
    #[must_use]
    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());

        self
    }

    /// The maximum number of events to return per page.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/admin/v1/orgs/{}/events", crate::core::encode_path_segment(&self.org_id)),
        );

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.q {
            config.query.push(("q".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.from {
            config.query.push(("from".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.to {
            config.query.push(("to".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.action {
            config.query.push(("action".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.actor {
            config.query.push(("actor".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.ip {
            config.query.push(("ip".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.product {
            config.query.push(("product".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.location {
            config.query.push(("location".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EventPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a paginated list of audit logs events for an organization. Use this endpoint if you want to retrieve events in a simple, paginated manner with time-based filtering.
///
/// If you need more advanced filtering, refer to the [/events](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-events/#api-v1-orgs-orgid-events-get) endpoint.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
#[derive(Clone)]
pub struct PollEventsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    cursor: Option<String>,
    from: Option<String>,
    to: Option<String>,
    limit: Option<i64>,
    sort_order: Option<PollEventsRequestSortOrder>,
}

impl<'a> PollEventsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), cursor: None, from: None, to: None, limit: None, sort_order: None }
    }

    /// Sets the starting point for the page of results to return. Can be used when last page is reached to poll for new events. The sort order is maintained in the cursor across requests.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// The earliest date and time of the event represented as a UNIX epoch time in milliseconds.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());

        self
    }

    /// The latest date and time of the event represented as a UNIX epoch time in milliseconds.
    #[must_use]
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());

        self
    }

    /// The maximum number of events to return per page.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The order used to sort events by processing time. Defaults to ascending.
    #[must_use]
    pub fn sort_order(mut self, value: impl Into<PollEventsRequestSortOrder>) -> Self {
        self.sort_order = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/admin/v1/orgs/{}/events-stream", crate::core::encode_path_segment(&self.org_id)),
        );

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.from {
            config.query.push(("from".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.to {
            config.query.push(("to".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.sort_order {
            config.query.push(("sortOrder".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PollingEventPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns information about a single event by ID.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
#[derive(Clone)]
pub struct GetEventByIdRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    event_id: String,
}

impl<'a> GetEventByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, event_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), event_id: event_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v1/orgs/{}/events/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.event_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Event> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns information localized event actions
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:events:admin`
#[derive(Clone)]
pub struct GetEventActionsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
}

impl<'a> GetEventActionsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/admin/v1/orgs/{}/event-actions", crate::core::encode_path_segment(&self.org_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EventActions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
