// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The CustomerRequests operations.
pub struct CustomerRequestsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CustomerRequestsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all comments on a customer request, for a given request Id/key.
    ///
    /// **Permissions:**
    ///
    /// Only comments that the calling user can see are returned.
    pub fn get_request_comments(&self, issue_id_or_key: impl Into<String>) -> GetRequestCommentsRequest<'a> {
        GetRequestCommentsRequest::new(self.client, issue_id_or_key)
    }

    /// Add a public or internal comment on an existing customer request. The currently logged-in user will be the author of the comment. The comment visibility is set by the `public` field.
    ///
    /// **Permissions:**
    ///
    /// Setting comment visibility depends on the calling user's permissions. For example, Agents can create either public or internal comments, Unlicensed users can only create internal comments, and Customers can only create public comments.
    pub fn create_request_comment(&self, issue_id_or_key: impl Into<String>) -> CreateRequestCommentRequest<'a> {
        CreateRequestCommentRequest::new(self.client, issue_id_or_key)
    }

    /// Returns a specific comment of a specific customer request based on the provided comment ID.
    ///
    /// **Permissions:**
    ///
    /// The calling user must have permission to view the comment. For example, customers can only view public comments on requests where they are the reporter or a participant whereas agents can see both internal and public comments.
    pub fn get_request_comment_by_id(
        &self,
        issue_id_or_key: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> GetRequestCommentByIdRequest<'a> {
        GetRequestCommentByIdRequest::new(self.client, issue_id_or_key, comment_id)
    }

    /// Returns all customer requests for the user that is executing the query. That is, the customer requests where the user is the creator of the customer request or has participated in the customer request.
    ///
    /// Returned customer requests are ordered chronologically by the latest activity on each customer request. For example, the latest status transition or comment.
    ///
    /// **Note:**
    /// The total number of issues across all pages that can be returned using paginated search is limited to the maxResultWindow, which is defined by the underlying search engine.
    /// The current value is returned in the `maxResultWindow` property of the response. If not set, it means there is no limit.
    pub fn get_my_customer_requests(&self) -> GetMyCustomerRequestsRequest<'a> {
        GetMyCustomerRequestsRequest::new(self.client)
    }

    /// Creates a customer request in a service project. The service project and request type are required. The fields that are mandatory for the request type are also required. If you need the list of the fields required for the request type, you can get it via this resource: [servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field](#servicedeskapi-servicedesk-{serviceDeskId}-requesttype-{requestTypeId}-field-get)
    ///
    /// **Notes**:
    /// * The fields for a request type may vary based on the permissions of the currently authenticated user:
    ///     * `raiseOnBehalfOf` field - Not available to users who only have the Service Desk Customer permission.
    ///     * `requestParticipants` field - Not available to users who only have the Service Desk Customer permission or if the feature is turned off for customers.
    /// * Schema of `requestFieldValues` field is a map of Jira's field's ID and its value, which are JSON ready objects. The object value will be interpreted with JSON semantics according to the specific field requirements. So a simple field like summary or number customer field might take String / Integer while other fields like Multi User Picker will take a more complex object that has JSON semantics. Refer to [Field input formats](https://developer.atlassian.com/server/jira-servicedesk/rest/intro#fieldformats) reference on what field types take what values.
    pub fn create_customer_request(&self) -> CreateCustomerRequestRequest<'a> {
        CreateCustomerRequestRequest::new(self.client)
    }

    /// Returns the customer request for a given request Id/key.
    pub fn get_customer_request_by_id_or_key(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> GetCustomerRequestByIdOrKeyRequest<'a> {
        GetCustomerRequestByIdOrKeyRequest::new(self.client, issue_id_or_key)
    }

    /// Returns all users participating in a customer request, for a given request Id/key.
    pub fn get_request_participants(&self, issue_id_or_key: impl Into<String>) -> GetRequestParticipantsRequest<'a> {
        GetRequestParticipantsRequest::new(self.client, issue_id_or_key)
    }

    /// Adds users as participants to an existing customer request.
    ///
    /// Note, you can also add participants when creating a request via the `request` resource, by using the `requestParticipants` field.
    ///
    /// **Permissions:**
    ///
    /// The calling user must have permission to manage participants for this customer request.
    pub fn add_request_participants(&self, issue_id_or_key: impl Into<String>) -> AddRequestParticipantsRequest<'a> {
        AddRequestParticipantsRequest::new(self.client, issue_id_or_key)
    }

    /// Removes participants from an existing customer request.
    ///
    /// **Permissions:**
    ///
    /// The calling user must have permission to manage participants for this customer request.
    pub fn remove_request_participants(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> RemoveRequestParticipantsRequest<'a> {
        RemoveRequestParticipantsRequest::new(self.client, issue_id_or_key)
    }

    /// Returns the SLA information for a customer request for a given request Id/key.A request can have zero or more SLA values. Each SLA value can have zero or more "completed cycles" and zero or 1 "ongoing cycles".Each cycle has information on when it started and stopped, and whether it breached the SLA goal.
    ///
    /// **Permissions:**
    ///
    /// The calling user must be an agent.
    pub fn get_sla_information(&self, issue_id_or_key: impl Into<String>) -> GetSlaInformationRequest<'a> {
        GetSlaInformationRequest::new(self.client, issue_id_or_key)
    }

    /// Returns the SLA information for a customer request for a given request Id/key and SLA metric Id.A request can have zero or more SLA values. Each SLA value can have zero or more "completed cycles" and zero or 1 "ongoing cycles".Each cycle has information on when it started and stopped, and whether it breached the SLA goal.
    ///
    /// **Permissions:**
    ///
    /// The calling user must be an agent.
    pub fn get_sla_information_by_id(
        &self,
        issue_id_or_key: impl Into<String>,
        sla_metric_id: impl Into<String>,
    ) -> GetSlaInformationByIdRequest<'a> {
        GetSlaInformationByIdRequest::new(self.client, issue_id_or_key, sla_metric_id)
    }

    /// Returns the status transitions for a customer request for a given request Id/key. The status transitions are returned in chronological order.
    pub fn get_customer_request_status(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> GetCustomerRequestStatusRequest<'a> {
        GetCustomerRequestStatusRequest::new(self.client, issue_id_or_key)
    }
}

/// Returns all comments on a customer request, for a given request Id/key.
///
/// **Permissions:**
///
/// Only comments that the calling user can see are returned.
#[derive(Clone)]
pub struct GetRequestCommentsRequest<'a> {
    client: &'a crate::core::Client,
    internal: Option<String>,
    public: Option<String>,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetRequestCommentsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), internal: None, public: None, start: None, limit: None }
    }

    /// Specifies whether to return internal comments or not. Default: true.
    #[must_use]
    pub fn internal(mut self, value: impl Into<String>) -> Self {
        self.internal = Some(value.into());

        self
    }

    /// Specifies whether to return public comments or not. Default: true.
    #[must_use]
    pub fn public(mut self, value: impl Into<String>) -> Self {
        self.public = Some(value.into());

        self
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
            format!("/rest/servicedeskapi/request/{}/comment", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.internal {
            config.query.push(("internal".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.public {
            config.query.push(("public".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Comment>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Comment>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Add a public or internal comment on an existing customer request. The currently logged-in user will be the author of the comment. The comment visibility is set by the `public` field.
///
/// **Permissions:**
///
/// Setting comment visibility depends on the calling user's permissions. For example, Agents can create either public or internal comments, Unlicensed users can only create internal comments, and Customers can only create public comments.
#[derive(Clone)]
pub struct CreateRequestCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    comment_create: Option<CommentCreate>,
}

impl<'a> CreateRequestCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), comment_create: None }
    }

    #[must_use]
    pub fn comment_create(mut self, value: CommentCreate) -> Self {
        self.comment_create = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/servicedeskapi/request/{}/comment", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        let body = match serde_json::to_value(&self.comment_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Comment> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a specific comment of a specific customer request based on the provided comment ID.
///
/// **Permissions:**
///
/// The calling user must have permission to view the comment. For example, customers can only view public comments on requests where they are the reporter or a participant whereas agents can see both internal and public comments.
#[derive(Clone)]
pub struct GetRequestCommentByIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    comment_id: String,
}

impl<'a> GetRequestCommentByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, comment_id: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), comment_id: comment_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/comment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.comment_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Comment> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all customer requests for the user that is executing the query. That is, the customer requests where the user is the creator of the customer request or has participated in the customer request.
///
/// Returned customer requests are ordered chronologically by the latest activity on each customer request. For example, the latest status transition or comment.
///
/// **Note:**
/// The total number of issues across all pages that can be returned using paginated search is limited to the maxResultWindow, which is defined by the underlying search engine.
/// The current value is returned in the `maxResultWindow` property of the response. If not set, it means there is no limit.
#[derive(Clone)]
pub struct GetMyCustomerRequestsRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    search_term: Option<String>,
    service_desk_id: Option<String>,
    request_ownership: Option<String>,
    request_type_id: Option<String>,
    request_status: Option<String>,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetMyCustomerRequestsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            expand: None,
            search_term: None,
            service_desk_id: None,
            request_ownership: None,
            request_type_id: None,
            request_status: None,
            start: None,
            limit: None,
        }
    }

    /// This is a multi-value parameter indicating which properties of the customer request to expand:
    /// * `serviceDesk` - Return additional details for each service project in the response.
    /// * `requestType` - Return additional details for each request type in the response.
    /// * `participant` - Return the participant details, if any, for each customer request in the response.
    /// * `sla` - Return the SLA information on the given request.
    /// * `status` - Return the status transitions, in chronological order, for each customer request in the response.
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// Filters results to customer requests where the issue summary matches the `searchTerm`. You can use [wildcards](https://confluence.atlassian.com/display/JIRACORECLOUD/Search+syntax+for+text+fields) in the `searchTerm`.
    #[must_use]
    pub fn search_term(mut self, value: impl Into<String>) -> Self {
        self.search_term = Some(value.into());

        self
    }

    /// Filters results to customer requests from a specific service project.
    #[must_use]
    pub fn service_desk_id(mut self, value: impl Into<String>) -> Self {
        self.service_desk_id = Some(value.into());

        self
    }

    /// Filters results to customer requests where the user is the creator and/or participant:
    /// * `OWNED_REQUESTS` - Only return customer requests where the user is the creator.
    /// * `PARTICIPATED_REQUESTS` - Only return customer requests where the user is a participant.
    /// * `ALL_REQUESTS` - Return customer requests where the user is the creator or a participant.
    #[must_use]
    pub fn request_ownership(mut self, value: impl Into<String>) -> Self {
        self.request_ownership = Some(value.into());

        self
    }

    /// Filters results to customer requests of a specific request type. You must also specify the `serviceDeskID` for the service desk that the request type belongs to.
    #[must_use]
    pub fn request_type_id(mut self, value: impl Into<String>) -> Self {
        self.request_type_id = Some(value.into());

        self
    }

    /// Filters results to customer requests that are resolved, unresolved, or either of the two:
    /// * `CLOSED_REQUESTS` - Only return customer requests that are resolved.
    /// * `OPEN_REQUESTS` - Only return customer requests that are unresolved.
    /// * `ALL_REQUESTS` - Returns customer requests that are either resolved or unresolved.
    #[must_use]
    pub fn request_status(mut self, value: impl Into<String>) -> Self {
        self.request_status = Some(value.into());

        self
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
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/request".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.search_term {
            config.query.push(("searchTerm".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.service_desk_id {
            config.query.push(("serviceDeskId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.request_ownership {
            config.query.push(("requestOwnership".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.request_type_id {
            config.query.push(("requestTypeId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.request_status {
            config.query.push(("requestStatus".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<CustomerRequest>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CustomerRequest>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a customer request in a service project. The service project and request type are required. The fields that are mandatory for the request type are also required. If you need the list of the fields required for the request type, you can get it via this resource: [servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field](#servicedeskapi-servicedesk-{serviceDeskId}-requesttype-{requestTypeId}-field-get)
///
/// **Notes**:
/// * The fields for a request type may vary based on the permissions of the currently authenticated user:
///     * `raiseOnBehalfOf` field - Not available to users who only have the Service Desk Customer permission.
///     * `requestParticipants` field - Not available to users who only have the Service Desk Customer permission or if the feature is turned off for customers.
/// * Schema of `requestFieldValues` field is a map of Jira's field's ID and its value, which are JSON ready objects. The object value will be interpreted with JSON semantics according to the specific field requirements. So a simple field like summary or number customer field might take String / Integer while other fields like Multi User Picker will take a more complex object that has JSON semantics. Refer to [Field input formats](https://developer.atlassian.com/server/jira-servicedesk/rest/intro#fieldformats) reference on what field types take what values.
#[derive(Clone)]
pub struct CreateCustomerRequestRequest<'a> {
    client: &'a crate::core::Client,
    request_create: Option<RequestCreate>,
}

impl<'a> CreateCustomerRequestRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, request_create: None }
    }

    #[must_use]
    pub fn request_create(mut self, value: RequestCreate) -> Self {
        self.request_create = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/servicedeskapi/request".to_owned());

        let body = match serde_json::to_value(&self.request_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CustomerRequest> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the customer request for a given request Id/key.
#[derive(Clone)]
pub struct GetCustomerRequestByIdOrKeyRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    issue_id_or_key: String,
}

impl<'a> GetCustomerRequestByIdOrKeyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), expand: None }
    }

    /// This is a multi-value parameter indicating which properties of the customer request to expand:
    /// * `serviceDesk` - Return additional details for each service project in the response.
    /// * `requestType` - Return additional details for each request type in the response.
    /// * `participant` - Return the participant details, if any, for each customer request in the response.
    /// * `sla` - Return the SLA information on the given request.
    /// * `status` - Return the status transitions, in chronological order, for each customer request in the response.
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/request/{}", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CustomerRequest> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all users participating in a customer request, for a given request Id/key.
#[derive(Clone)]
pub struct GetRequestParticipantsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetRequestParticipantsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start: None, limit: None }
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
                "/rest/servicedeskapi/request/{}/participant",
                crate::core::encode_path_segment(&self.issue_id_or_key)
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

/// Adds users as participants to an existing customer request.
///
/// Note, you can also add participants when creating a request via the `request` resource, by using the `requestParticipants` field.
///
/// **Permissions:**
///
/// The calling user must have permission to manage participants for this customer request.
#[derive(Clone)]
pub struct AddRequestParticipantsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
    request_participant_update: Option<RequestParticipantUpdate>,
}

impl<'a> AddRequestParticipantsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            start: None,
            limit: None,
            request_participant_update: None,
        }
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

    #[must_use]
    pub fn request_participant_update(mut self, value: RequestParticipantUpdate) -> Self {
        self.request_participant_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/participant",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.request_participant_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Removes participants from an existing customer request.
///
/// **Permissions:**
///
/// The calling user must have permission to manage participants for this customer request.
#[derive(Clone)]
pub struct RemoveRequestParticipantsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
    request_participant_update: Option<RequestParticipantUpdate>,
}

impl<'a> RemoveRequestParticipantsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            start: None,
            limit: None,
            request_participant_update: None,
        }
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

    #[must_use]
    pub fn request_participant_update(mut self, value: RequestParticipantUpdate) -> Self {
        self.request_participant_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/servicedeskapi/request/{}/participant",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.request_participant_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Returns the SLA information for a customer request for a given request Id/key.A request can have zero or more SLA values. Each SLA value can have zero or more "completed cycles" and zero or 1 "ongoing cycles".Each cycle has information on when it started and stopped, and whether it breached the SLA goal.
///
/// **Permissions:**
///
/// The calling user must be an agent.
#[derive(Clone)]
pub struct GetSlaInformationRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetSlaInformationRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start: None, limit: None }
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
            format!("/rest/servicedeskapi/request/{}/sla", crate::core::encode_path_segment(&self.issue_id_or_key)),
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<SlaInformation>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<SlaInformation>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the SLA information for a customer request for a given request Id/key and SLA metric Id.A request can have zero or more SLA values. Each SLA value can have zero or more "completed cycles" and zero or 1 "ongoing cycles".Each cycle has information on when it started and stopped, and whether it breached the SLA goal.
///
/// **Permissions:**
///
/// The calling user must be an agent.
#[derive(Clone)]
pub struct GetSlaInformationByIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    sla_metric_id: String,
}

impl<'a> GetSlaInformationByIdRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        sla_metric_id: impl Into<String>,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), sla_metric_id: sla_metric_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/sla/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.sla_metric_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SlaInformation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the status transitions for a customer request for a given request Id/key. The status transitions are returned in chronological order.
#[derive(Clone)]
pub struct GetCustomerRequestStatusRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetCustomerRequestStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start: None, limit: None }
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
            format!("/rest/servicedeskapi/request/{}/status", crate::core::encode_path_segment(&self.issue_id_or_key)),
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<CustomerRequestStatus>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CustomerRequestStatus>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
