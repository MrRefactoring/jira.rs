// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Request operations.
pub struct RequestService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> RequestService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// This method returns all customer requests for the user executing the query.
    ///
    /// The returned customer requests are ordered chronologically by the latest activity on each request. For example, the latest status transition or comment.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to access the specified service desk.
    ///
    /// **Response limitations**: For customers, the list returned will include request they created (or were created on their behalf) or are participating in only.
    pub fn get_customer_requests(&self) -> GetCustomerRequestsRequest<'a> {
        GetCustomerRequestsRequest::new(self.client)
    }

    /// This method creates a customer request in a service desk.
    ///
    /// The JSON request must include the service desk and customer request type, as well as any fields that are required for the request type. A list of the fields required by a customer request type can be obtained using [servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-servicedesk-serviceDeskId-requesttype-requestTypeId-field-get).
    ///
    /// The fields required for a customer request type depend on the user's permissions:
    ///
    ///  *  `raiseOnBehalfOf` is not available to Users who have the customer permission only.
    ///  *  `requestParticipants` is not available to Users who have the customer permission only or if the feature is turned off for customers.
    ///
    /// `requestFieldValues` is a map of Jira field IDs and their values. See [Field input formats](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#fieldformats), for details of each field's JSON semantics and the values they can take.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to create requests in the specified service desk.
    pub fn create_customer_request(&self, request_create: RequestCreate) -> CreateCustomerRequestRequest<'a> {
        CreateCustomerRequestRequest::new(self.client, request_create)
    }

    /// Validates a customer request payload without creating (persisting) a request.
    ///
    /// This endpoint runs exactly the same structural and semantic validations as [Create customer request](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-request-post) — including ProForma form validation — but performs **no mutation**: no issue is created and no side effects (attachments, comments, analytics) run.
    ///
    /// The response is intentionally verbose and structured so that it can be consumed by automated agents (for example an LLM repairing an invalid payload): every failure carries a machine-readable location (field id / form entity) and a human-readable reason. A valid payload returns HTTP 200 with `valid: true`; an invalid payload returns HTTP 400 with `valid: false` together with the field, form and general validation errors.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to create requests in the specified service desk.
    pub fn validate_customer_request(&self, request_create: RequestCreate) -> ValidateCustomerRequestRequest<'a> {
        ValidateCustomerRequestRequest::new(self.client, request_create)
    }

    /// This method returns a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to access the specified service desk.
    ///
    /// **Response limitations**: For customers, only a request they created, was created on their behalf, or they are participating in will be returned.
    ///
    /// **Note:** `requestFieldValues` does not include hidden fields. To get a list of request type fields that includes hidden fields, see [/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field](https://developer.atlassian.com/cloud/jira/service-desk/rest/api-group-servicedesk/#api-rest-servicedeskapi-servicedesk-servicedeskid-requesttype-requesttypeid-field-get)
    pub fn get_customer_request_by_id_or_key(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> GetCustomerRequestByIdOrKeyRequest<'a> {
        GetCustomerRequestByIdOrKeyRequest::new(self.client, issue_id_or_key)
    }

    /// This method returns all approvals on a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn get_approvals(&self, issue_id_or_key: impl Into<String>) -> GetApprovalsRequest<'a> {
        GetApprovalsRequest::new(self.client, issue_id_or_key)
    }

    /// This method returns an approval. Use this method to determine the status of an approval and the list of approvers.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn get_approval_by_id(
        &self,
        issue_id_or_key: impl Into<String>,
        approval_id: i64,
    ) -> GetApprovalByIdRequest<'a> {
        GetApprovalByIdRequest::new(self.client, issue_id_or_key, approval_id)
    }

    /// This method enables a user to **Approve** or **Decline** an approval on a customer request. The approval is assumed to be owned by the user making the call.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: User is assigned to the approval request.
    pub fn answer_approval(
        &self,
        issue_id_or_key: impl Into<String>,
        approval_id: i64,
        approval_decision_request: ApprovalDecisionRequest,
    ) -> AnswerApprovalRequest<'a> {
        AnswerApprovalRequest::new(self.client, issue_id_or_key, approval_id, approval_decision_request)
    }

    /// This method returns all the attachments for a customer requests.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    ///
    /// **Response limitations**: Customers will only get a list of public attachments.
    pub fn get_attachments_for_request(
        &self,
        issue_id_or_key: impl Into<String>,
        start: i64,
        limit: i64,
    ) -> GetAttachmentsForRequestRequest<'a> {
        GetAttachmentsForRequestRequest::new(self.client, issue_id_or_key, start, limit)
    }

    /// This method creates a comment on a customer request using one or more attachment files (uploaded using [servicedeskapi/servicedesk/{serviceDeskId}/attachTemporaryFile](https://developer.atlassian.com/cloud/jira/service-desk/rest/api-group-servicedesk/#api-rest-servicedeskapi-servicedesk-servicedeskid-attachtemporaryfile-post)), with the visibility set by `public`. See
    ///
    ///  *  GET [servicedeskapi/request/{issueIdOrKey}/attachment](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-attachment-get)
    ///  *  GET [servicedeskapi/request/{issueIdOrKey}/comment/{commentId}/attachment](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-comment-commentid-attachment-get)
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to add an attachment.
    ///
    /// **Request limitations**: Customers can set public visibility only.
    pub fn create_comment_with_attachment(
        &self,
        issue_id_or_key: impl Into<String>,
        attachment_create: AttachmentCreate,
    ) -> CreateCommentWithAttachmentRequest<'a> {
        CreateCommentWithAttachmentRequest::new(self.client, issue_id_or_key, attachment_create)
    }

    /// Returns the contents of an attachment.
    ///
    /// To return a thumbnail of the attachment, use [servicedeskapi/request/{issueIdOrKey}/attachment/{attachmentId}/thumbnail](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-attachment-attachmentid-thumbnail-get).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required:** For the issue containing the attachment:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn get_attachment_content(
        &self,
        issue_id_or_key: impl Into<String>,
        attachment_id: i64,
    ) -> GetAttachmentContentRequest<'a> {
        GetAttachmentContentRequest::new(self.client, issue_id_or_key, attachment_id)
    }

    /// Returns the thumbnail of an attachment.
    ///
    /// To return the attachment contents, use [servicedeskapi/request/{issueIdOrKey}/attachment/{attachmentId}](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-attachment-attachmentid-get).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required:** For the issue containing the attachment:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn get_attachment_thumbnail(
        &self,
        issue_id_or_key: impl Into<String>,
        attachment_id: i64,
    ) -> GetAttachmentThumbnailRequest<'a> {
        GetAttachmentThumbnailRequest::new(self.client, issue_id_or_key, attachment_id)
    }

    /// This method returns all comments on a customer request. No permissions error is provided if, for example, the user doesn't have access to the service desk or request, the method simply returns an empty response.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    ///
    /// **Response limitations**: Customers are returned public comments only.
    pub fn get_request_comments(&self, issue_id_or_key: impl Into<String>) -> GetRequestCommentsRequest<'a> {
        GetRequestCommentsRequest::new(self.client, issue_id_or_key)
    }

    /// This method creates a public or private (internal) comment on a customer request, with the comment visibility set by `public`. The user recorded as the author of the comment.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: User has Add Comments permission.
    ///
    /// **Request limitations**: Customers can set comments to public visibility only.
    pub fn create_request_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        comment_create: CommentCreate,
    ) -> CreateRequestCommentRequest<'a> {
        CreateRequestCommentRequest::new(self.client, issue_id_or_key, comment_create)
    }

    /// This method returns details of a customer request's comment.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    ///
    /// **Response limitations**: Customers can only view public comments on requests where they are the reporter or a participant whereas agents can see both internal and public comments.
    pub fn get_request_comment_by_id(
        &self,
        issue_id_or_key: impl Into<String>,
        comment_id: i64,
    ) -> GetRequestCommentByIdRequest<'a> {
        GetRequestCommentByIdRequest::new(self.client, issue_id_or_key, comment_id)
    }

    /// This method returns the notification subscription status of the user making the request. Use this method to determine if the user is subscribed to a customer request's notifications.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn get_subscription_status(&self, issue_id_or_key: impl Into<String>) -> GetSubscriptionStatusRequest<'a> {
        GetSubscriptionStatusRequest::new(self.client, issue_id_or_key)
    }

    /// This method subscribes the user to receiving notifications from a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn subscribe(&self, issue_id_or_key: impl Into<String>) -> SubscribeRequest<'a> {
        SubscribeRequest::new(self.client, issue_id_or_key)
    }

    /// This method unsubscribes the user from notifications from a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn unsubscribe(&self, issue_id_or_key: impl Into<String>) -> UnsubscribeRequest<'a> {
        UnsubscribeRequest::new(self.client, issue_id_or_key)
    }

    /// This method returns a list of all the participants on a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn get_request_participants(&self, issue_id_or_key: impl Into<String>) -> GetRequestParticipantsRequest<'a> {
        GetRequestParticipantsRequest::new(self.client, issue_id_or_key)
    }

    /// This method adds participants to a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to manage participants on the customer request.
    ///
    /// Note, participants can be added when creating a customer request using the [request](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-request-post) resource, by defining the participants in the `requestParticipants` field.
    pub fn add_request_participants(
        &self,
        issue_id_or_key: impl Into<String>,
        request_participant_update: RequestParticipantUpdate,
    ) -> AddRequestParticipantsRequest<'a> {
        AddRequestParticipantsRequest::new(self.client, issue_id_or_key, request_participant_update)
    }

    /// This method removes participants from a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to manage participants on the customer request.
    pub fn remove_request_participants(
        &self,
        issue_id_or_key: impl Into<String>,
        request_participant_update: RequestParticipantUpdate,
    ) -> RemoveRequestParticipantsRequest<'a> {
        RemoveRequestParticipantsRequest::new(self.client, issue_id_or_key, request_participant_update)
    }

    /// This method returns all the SLA records on a customer request. A customer request can have zero or more SLAs. Each SLA can have recordings for zero or more "completed cycles" and zero or 1 "ongoing cycle". Each cycle includes information on when it started and stopped, and whether it breached the SLA goal.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**:
    ///
    ///  *  Agent for the Service Desk containing the queried customer request, AND
    ///  *  Browse Projects permission on the project containing the customer request, including any restrictions imposed by issue security schemes or custom permission schemes on the specific issue.
    pub fn get_sla_information(&self, issue_id_or_key: impl Into<String>) -> GetSlaInformationRequest<'a> {
        GetSlaInformationRequest::new(self.client, issue_id_or_key)
    }

    /// This method returns the details for an SLA on a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**:
    ///
    ///  *  Agent for the Service Desk containing the queried customer request, AND
    ///  *  Browse Projects permission on the project containing the customer request, including any restrictions imposed by issue security schemes or custom permission schemes on the specific issue.
    pub fn get_sla_information_by_id(
        &self,
        issue_id_or_key: impl Into<String>,
        sla_metric_id: i64,
    ) -> GetSlaInformationByIdRequest<'a> {
        GetSlaInformationByIdRequest::new(self.client, issue_id_or_key, sla_metric_id)
    }

    /// This method returns a list of all the statuses a customer Request has achieved. A status represents the state of an issue in its workflow. An issue can have one active status only. The list returns the status history in chronological order, most recent (current) status first.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn get_customer_request_status(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> GetCustomerRequestStatusRequest<'a> {
        GetCustomerRequestStatusRequest::new(self.client, issue_id_or_key)
    }

    /// This method returns a list of transitions, the workflow processes that moves a customer request from one status to another, that the user can perform on a request. Use this method to provide a user with a list if the actions they can take on a customer request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
    pub fn get_customer_transitions(&self, issue_id_or_key: impl Into<String>) -> GetCustomerTransitionsRequest<'a> {
        GetCustomerTransitionsRequest::new(self.client, issue_id_or_key)
    }

    /// This method performs a customer transition for a given request and transition. An optional comment can be included to provide a reason for the transition.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: The user must be able to view the request and have the Transition Issues permission. If a comment is passed the user must have the Add Comments permission.
    pub fn perform_customer_transition(
        &self,
        issue_id_or_key: impl Into<String>,
        customer_transition_execution: CustomerTransitionExecution,
    ) -> PerformCustomerTransitionRequest<'a> {
        PerformCustomerTransitionRequest::new(self.client, issue_id_or_key, customer_transition_execution)
    }
}

/// This method returns all customer requests for the user executing the query.
///
/// The returned customer requests are ordered chronologically by the latest activity on each request. For example, the latest status transition or comment.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to access the specified service desk.
///
/// **Response limitations**: For customers, the list returned will include request they created (or were created on their behalf) or are participating in only.
#[derive(Clone)]
pub struct GetCustomerRequestsRequest<'a> {
    client: &'a crate::core::Client,
    search_term: Option<String>,
    request_ownership: Option<Vec<String>>,
    request_status: Option<String>,
    approval_status: Option<String>,
    organization_id: Option<i64>,
    service_desk_id: Option<i64>,
    request_type_id: Option<i64>,
    expand: Option<Vec<String>>,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetCustomerRequestsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            search_term: None,
            request_ownership: None,
            request_status: None,
            approval_status: None,
            organization_id: None,
            service_desk_id: None,
            request_type_id: None,
            expand: None,
            start: None,
            limit: None,
        }
    }

    /// Filters customer requests where the request summary matches the `searchTerm`. [Wildcards](https://confluence.atlassian.com/display/JIRACORECLOUD/Search+syntax+for+text+fields) can be used in the `searchTerm` parameter.
    #[must_use]
    pub fn search_term(mut self, value: impl Into<String>) -> Self {
        self.search_term = Some(value.into());

        self
    }

    /// Filters customer requests using the following values:
    ///
    ///  *  `OWNED_REQUESTS` returns customer requests where the user is the creator.
    ///  *  `PARTICIPATED_REQUESTS` returns customer requests where the user is a participant.
    ///  *  `ORGANIZATION` returns customer requests for an organization of which the user is a member when used in conjunction with `organizationId`.
    ///  *  `ALL_ORGANIZATIONS` returns customer requests that belong to all organizations of which the user is a member.
    ///  *  `APPROVER` returns customer requests where the user is an approver. Can be used in conjunction with `approvalStatus` to filter pending or complete approvals.
    ///  *  `ALL_REQUESTS` returns all customer requests. **Deprecated and will be removed, as the returned requests may change if more values are added in the future. Instead, explicitly list the desired filtering strategies.**
    ///
    /// Multiple values of the query parameter are supported. For example, `requestOwnership=OWNED_REQUESTS&requestOwnership=PARTICIPATED_REQUESTS` will only return customer requests where the user is the creator or a participant. If not specified, filtering defaults to `OWNED_REQUESTS`, `PARTICIPATED_REQUESTS`, and `ALL_ORGANIZATIONS`.
    #[deprecated(
        note = "**Deprecated and will be removed, as the returned requests may change if more values are added in the future."
    )]
    #[must_use]
    pub fn request_ownership(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.request_ownership = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Filters customer requests where the request is closed, open, or either of the two where:
    ///
    ///  *  `CLOSED_REQUESTS` returns customer requests that are closed.
    ///  *  `OPEN_REQUESTS` returns customer requests that are open.
    ///  *  `ALL_REQUESTS` returns all customer requests.
    #[must_use]
    pub fn request_status(mut self, value: impl Into<String>) -> Self {
        self.request_status = Some(value.into());

        self
    }

    /// Filters results to customer requests based on their approval status:
    ///
    ///  *  `MY_PENDING_APPROVAL` returns customer requests pending the user's approval.
    ///  *  `MY_HISTORY_APPROVAL` returns customer requests where the user was an approver.
    ///
    /// **Note**: Valid only when used with requestOwnership=APPROVER.
    #[must_use]
    pub fn approval_status(mut self, value: impl Into<String>) -> Self {
        self.approval_status = Some(value.into());

        self
    }

    /// Filters customer requests that belong to a specific organization (note that the user must be a member of that organization). **Note**: Valid only when used with requestOwnership=ORGANIZATION.
    #[must_use]
    pub fn organization_id(mut self, value: i64) -> Self {
        self.organization_id = Some(value);

        self
    }

    /// Filters customer requests by service desk.
    #[must_use]
    pub fn service_desk_id(mut self, value: i64) -> Self {
        self.service_desk_id = Some(value);

        self
    }

    /// Filters customer requests by request type. Note that the `serviceDeskId` must be specified for the service desk in which the request type belongs.
    #[must_use]
    pub fn request_type_id(mut self, value: i64) -> Self {
        self.request_type_id = Some(value);

        self
    }

    /// A multi-value parameter indicating which properties of the customer request to expand, where:
    ///
    ///  *  `serviceDesk` returns additional details for each service desk.
    ///  *  `requestType` returns additional details for each request type.
    ///  *  `participant` returns the participant details, if any, for each customer request.
    ///  *  `sla` returns the SLA information on each customer request.
    ///  *  `status` returns the status transitions, in chronological order, for each customer request.
    ///  *  `attachment` returns the attachments for the customer request.
    ///  *  `action` returns the actions that the user can or cannot perform on this customer request.
    ///  *  `comment` returns the comments, if any, for each customer request.
    ///  *  `comment.attachment` returns the attachment details, if any, for each comment.
    ///  *  `comment.renderedBody` (Experimental) returns the rendered body in HTML format (in addition to the raw body) for each comment.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/request".to_owned());

        if let Some(value) = &self.search_term {
            config.query.push(("searchTerm".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.request_ownership {
            config.query.push(("requestOwnership".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.request_status {
            config.query.push(("requestStatus".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.approval_status {
            config.query.push(("approvalStatus".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.organization_id {
            config.query.push(("organizationId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.service_desk_id {
            config.query.push(("serviceDeskId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.request_type_id {
            config.query.push(("requestTypeId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
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

/// This method creates a customer request in a service desk.
///
/// The JSON request must include the service desk and customer request type, as well as any fields that are required for the request type. A list of the fields required by a customer request type can be obtained using [servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-servicedesk-serviceDeskId-requesttype-requestTypeId-field-get).
///
/// The fields required for a customer request type depend on the user's permissions:
///
///  *  `raiseOnBehalfOf` is not available to Users who have the customer permission only.
///  *  `requestParticipants` is not available to Users who have the customer permission only or if the feature is turned off for customers.
///
/// `requestFieldValues` is a map of Jira field IDs and their values. See [Field input formats](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#fieldformats), for details of each field's JSON semantics and the values they can take.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to create requests in the specified service desk.
#[derive(Clone)]
pub struct CreateCustomerRequestRequest<'a> {
    client: &'a crate::core::Client,
    request_create: RequestCreate,
}

impl<'a> CreateCustomerRequestRequest<'a> {
    fn new(client: &'a crate::core::Client, request_create: RequestCreate) -> Self {
        Self { client, request_create }
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

/// Validates a customer request payload without creating (persisting) a request.
///
/// This endpoint runs exactly the same structural and semantic validations as [Create customer request](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-request-post) — including ProForma form validation — but performs **no mutation**: no issue is created and no side effects (attachments, comments, analytics) run.
///
/// The response is intentionally verbose and structured so that it can be consumed by automated agents (for example an LLM repairing an invalid payload): every failure carries a machine-readable location (field id / form entity) and a human-readable reason. A valid payload returns HTTP 200 with `valid: true`; an invalid payload returns HTTP 400 with `valid: false` together with the field, form and general validation errors.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to create requests in the specified service desk.
#[derive(Clone)]
pub struct ValidateCustomerRequestRequest<'a> {
    client: &'a crate::core::Client,
    request_create: RequestCreate,
}

impl<'a> ValidateCustomerRequestRequest<'a> {
    fn new(client: &'a crate::core::Client, request_create: RequestCreate) -> Self {
        Self { client, request_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/servicedeskapi/request/validate".to_owned(),
        );

        let body = match serde_json::to_value(&self.request_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RequestValidationResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to access the specified service desk.
///
/// **Response limitations**: For customers, only a request they created, was created on their behalf, or they are participating in will be returned.
///
/// **Note:** `requestFieldValues` does not include hidden fields. To get a list of request type fields that includes hidden fields, see [/rest/servicedeskapi/servicedesk/{serviceDeskId}/requesttype/{requestTypeId}/field](https://developer.atlassian.com/cloud/jira/service-desk/rest/api-group-servicedesk/#api-rest-servicedeskapi-servicedesk-servicedeskid-requesttype-requesttypeid-field-get)
#[derive(Clone)]
pub struct GetCustomerRequestByIdOrKeyRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    expand: Option<Vec<String>>,
}

impl<'a> GetCustomerRequestByIdOrKeyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), expand: None }
    }

    /// A multi-value parameter indicating which properties of the customer request to expand, where:
    ///
    ///  *  `serviceDesk` returns additional service desk details.
    ///  *  `requestType` returns additional customer request type details.
    ///  *  `participant` returns the participant details.
    ///  *  `sla` returns the SLA information.
    ///  *  `status` returns the status transitions, in chronological order.
    ///  *  `attachment` returns the attachments.
    ///  *  `action` returns the actions that the user can or cannot perform.
    ///  *  `comment` returns the comments.
    ///  *  `comment.attachment` returns the attachment details for each comment.
    ///  *  `comment.renderedBody` (Experimental) return the rendered body in HTML format (in addition to the raw body) for each comment.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/request/{}", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
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

/// This method returns all approvals on a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
#[derive(Clone)]
pub struct GetApprovalsRequest<'a> {
    client: &'a crate::core::Client,
    start: Option<i64>,
    limit: Option<i64>,
    issue_id_or_key: String,
}

impl<'a> GetApprovalsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of approvals to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
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
                "/rest/servicedeskapi/request/{}/approval",
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Approval>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Approval>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns an approval. Use this method to determine the status of an approval and the list of approvers.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
#[derive(Clone)]
pub struct GetApprovalByIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    approval_id: i64,
}

impl<'a> GetApprovalByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, approval_id: i64) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), approval_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/approval/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                self.approval_id
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Approval> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method enables a user to **Approve** or **Decline** an approval on a customer request. The approval is assumed to be owned by the user making the call.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: User is assigned to the approval request.
#[derive(Clone)]
pub struct AnswerApprovalRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    approval_id: i64,
    approval_decision_request: ApprovalDecisionRequest,
}

impl<'a> AnswerApprovalRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        approval_id: i64,
        approval_decision_request: ApprovalDecisionRequest,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), approval_id, approval_decision_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/approval/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                self.approval_id
            ),
        );

        let body = match serde_json::to_value(&self.approval_decision_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Approval> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns all the attachments for a customer requests.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
///
/// **Response limitations**: Customers will only get a list of public attachments.
#[derive(Clone)]
pub struct GetAttachmentsForRequestRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: i64,
    limit: i64,
}

impl<'a> GetAttachmentsForRequestRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, start: i64, limit: i64) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start, limit }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/attachment",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(self.start.to_string())));

        config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(self.limit.to_string())));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Attachment>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method creates a comment on a customer request using one or more attachment files (uploaded using [servicedeskapi/servicedesk/{serviceDeskId}/attachTemporaryFile](https://developer.atlassian.com/cloud/jira/service-desk/rest/api-group-servicedesk/#api-rest-servicedeskapi-servicedesk-servicedeskid-attachtemporaryfile-post)), with the visibility set by `public`. See
///
///  *  GET [servicedeskapi/request/{issueIdOrKey}/attachment](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-attachment-get)
///  *  GET [servicedeskapi/request/{issueIdOrKey}/comment/{commentId}/attachment](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-comment-commentid-attachment-get)
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to add an attachment.
///
/// **Request limitations**: Customers can set public visibility only.
#[derive(Clone)]
pub struct CreateCommentWithAttachmentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    attachment_create: AttachmentCreate,
}

impl<'a> CreateCommentWithAttachmentRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        attachment_create: AttachmentCreate,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), attachment_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/attachment",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        let body = match serde_json::to_value(&self.attachment_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AttachmentCreateResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the contents of an attachment.
///
/// To return a thumbnail of the attachment, use [servicedeskapi/request/{issueIdOrKey}/attachment/{attachmentId}/thumbnail](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-attachment-attachmentid-thumbnail-get).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required:** For the issue containing the attachment:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
#[derive(Clone)]
pub struct GetAttachmentContentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    attachment_id: i64,
}

impl<'a> GetAttachmentContentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, attachment_id: i64) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), attachment_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/attachment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                self.attachment_id
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<bytes::Bytes> {
        self.client.send_bytes(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the thumbnail of an attachment.
///
/// To return the attachment contents, use [servicedeskapi/request/{issueIdOrKey}/attachment/{attachmentId}](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-rest-servicedeskapi-request-issueidorkey-attachment-attachmentid-get).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required:** For the issue containing the attachment:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
#[derive(Clone)]
pub struct GetAttachmentThumbnailRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    attachment_id: i64,
}

impl<'a> GetAttachmentThumbnailRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, attachment_id: i64) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), attachment_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/attachment/{}/thumbnail",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                self.attachment_id
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<bytes::Bytes> {
        self.client.send_bytes(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method returns all comments on a customer request. No permissions error is provided if, for example, the user doesn't have access to the service desk or request, the method simply returns an empty response.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
///
/// **Response limitations**: Customers are returned public comments only.
#[derive(Clone)]
pub struct GetRequestCommentsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    public: Option<bool>,
    internal: Option<bool>,
    expand: Option<Vec<String>>,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetRequestCommentsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            public: None,
            internal: None,
            expand: None,
            start: None,
            limit: None,
        }
    }

    /// Specifies whether to return public comments or not. Default: true.
    #[must_use]
    pub fn public(mut self, value: bool) -> Self {
        self.public = Some(value);

        self
    }

    /// Specifies whether to return internal comments or not. Default: true.
    #[must_use]
    pub fn internal(mut self, value: bool) -> Self {
        self.internal = Some(value);

        self
    }

    /// A multi-value parameter indicating which properties of the comment to expand:
    ///
    ///  *  `attachment` returns the attachment details, if any, for each comment. (If you want to get all attachments for a request, use [servicedeskapi/request/{issueIdOrKey}/attachment](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-request-issueIdOrKey-attachment-get).)
    ///  *  `renderedBody` (Experimental) returns the rendered body in HTML format (in addition to the raw body) for each comment.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned comments. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of comments to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
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

        if let Some(value) = &self.public {
            config.query.push(("public".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.internal {
            config.query.push(("internal".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
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

/// This method creates a public or private (internal) comment on a customer request, with the comment visibility set by `public`. The user recorded as the author of the comment.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: User has Add Comments permission.
///
/// **Request limitations**: Customers can set comments to public visibility only.
#[derive(Clone)]
pub struct CreateRequestCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    comment_create: CommentCreate,
}

impl<'a> CreateRequestCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, comment_create: CommentCreate) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), comment_create }
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

/// This method returns details of a customer request's comment.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
///
/// **Response limitations**: Customers can only view public comments on requests where they are the reporter or a participant whereas agents can see both internal and public comments.
#[derive(Clone)]
pub struct GetRequestCommentByIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    comment_id: i64,
    expand: Option<Vec<String>>,
}

impl<'a> GetRequestCommentByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, comment_id: i64) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), comment_id, expand: None }
    }

    /// A multi-value parameter indicating which properties of the comment to expand:
    ///
    ///  *  `attachment` returns the attachment details, if any, for the comment. (If you want to get all attachments for a request, use [servicedeskapi/request/{issueIdOrKey}/attachment](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-request-issueIdOrKey-attachment-get).)
    ///  *  `renderedBody` (Experimental) returns the rendered body in HTML format (in addition to the raw body) of the comment.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/comment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                self.comment_id
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

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

/// This method returns the notification subscription status of the user making the request. Use this method to determine if the user is subscribed to a customer request's notifications.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
#[derive(Clone)]
pub struct GetSubscriptionStatusRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetSubscriptionStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/notification",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RequestNotificationSubscription> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method subscribes the user to receiving notifications from a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
#[derive(Clone)]
pub struct SubscribeRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> SubscribeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/servicedeskapi/request/{}/notification",
                crate::core::encode_path_segment(&self.issue_id_or_key)
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

/// This method unsubscribes the user from notifications from a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
#[derive(Clone)]
pub struct UnsubscribeRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> UnsubscribeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/servicedeskapi/request/{}/notification",
                crate::core::encode_path_segment(&self.issue_id_or_key)
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

/// This method returns a list of all the participants on a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
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

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of request types to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
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

/// This method adds participants to a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to manage participants on the customer request.
///
/// Note, participants can be added when creating a customer request using the [request](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#api-request-post) resource, by defining the participants in the `requestParticipants` field.
#[derive(Clone)]
pub struct AddRequestParticipantsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    request_participant_update: RequestParticipantUpdate,
}

impl<'a> AddRequestParticipantsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        request_participant_update: RequestParticipantUpdate,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), request_participant_update }
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

        let body = match serde_json::to_value(&self.request_participant_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
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

/// This method removes participants from a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to manage participants on the customer request.
#[derive(Clone)]
pub struct RemoveRequestParticipantsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    request_participant_update: RequestParticipantUpdate,
}

impl<'a> RemoveRequestParticipantsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        request_participant_update: RequestParticipantUpdate,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), request_participant_update }
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

        let body = match serde_json::to_value(&self.request_participant_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
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

/// This method returns all the SLA records on a customer request. A customer request can have zero or more SLAs. Each SLA can have recordings for zero or more "completed cycles" and zero or 1 "ongoing cycle". Each cycle includes information on when it started and stopped, and whether it breached the SLA goal.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**:
///
///  *  Agent for the Service Desk containing the queried customer request, AND
///  *  Browse Projects permission on the project containing the customer request, including any restrictions imposed by issue security schemes or custom permission schemes on the specific issue.
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

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of request types to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
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

/// This method returns the details for an SLA on a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**:
///
///  *  Agent for the Service Desk containing the queried customer request, AND
///  *  Browse Projects permission on the project containing the customer request, including any restrictions imposed by issue security schemes or custom permission schemes on the specific issue.
#[derive(Clone)]
pub struct GetSlaInformationByIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    sla_metric_id: i64,
}

impl<'a> GetSlaInformationByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, sla_metric_id: i64) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), sla_metric_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/sla/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                self.sla_metric_id
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

/// This method returns a list of all the statuses a customer Request has achieved. A status represents the state of an issue in its workflow. An issue can have one active status only. The list returns the status history in chronological order, most recent (current) status first.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
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

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
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

/// This method returns a list of transitions, the workflow processes that moves a customer request from one status to another, that the user can perform on a request. Use this method to provide a user with a list if the actions they can take on a customer request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to view the customer request.
#[derive(Clone)]
pub struct GetCustomerTransitionsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetCustomerTransitionsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50. See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
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
                "/rest/servicedeskapi/request/{}/transition",
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<CustomerTransition>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CustomerTransition>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This method performs a customer transition for a given request and transition. An optional comment can be included to provide a reason for the transition.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: The user must be able to view the request and have the Transition Issues permission. If a comment is passed the user must have the Add Comments permission.
#[derive(Clone)]
pub struct PerformCustomerTransitionRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    customer_transition_execution: CustomerTransitionExecution,
}

impl<'a> PerformCustomerTransitionRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        customer_transition_execution: CustomerTransitionExecution,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), customer_transition_execution }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/transition",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        let body = match serde_json::to_value(&self.customer_transition_execution)? {
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
