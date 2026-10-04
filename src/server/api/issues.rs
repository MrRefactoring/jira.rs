// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::Serialize;

/// The expand param is used to include, hidden by default, parts of response. This can be used to include: renderedFields, names, schema, transitions, operations, editmeta, changelog, versionedRepresentations.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum IssuesGetIssueRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(IssuesGetIssueRequestExpand { One, Many, Other });

crate::open_enum! {
    /// A String of true or false indicating that any subtasks should also be deleted. If the issue has no subtasks this parameter is ignored. If the issue has subtasks and this parameter is missing or false, then the issue will not be deleted and an error will be returned.
    pub enum DeleteIssueRequestDeleteSubtasks {
        True => "true",
        False => "false",
    }
}

/// The Issues operations.
pub struct IssuesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssuesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates an issue or a sub-task from a JSON representation.
    /// The fields that can be set on create, in either the fields parameter or the update parameter can be determined using the /rest/api/2/issue/createmeta resource.
    /// If a field is not configured to appear on the create screen, then it will not be in the createmeta, and a field
    /// validation error will occur if it is submitted.
    /// Creating a sub-task is similar to creating a regular issue, with two important differences:
    /// - the issueType field must correspond to a sub-task issue type (you can use /issue/createmeta to discover sub-task issue types), and
    /// - you must provide a parent field in the issue create request containing the id or key of the parent issue.
    /// The updateHistory param adds the project that this issue is created in, to the current user's project history, if set to true (by default, the project history is not updated).
    /// You can view the project history in the Jira application, via the Projects dropdown.
    pub fn create_issue(&self, issue_update_details: IssueUpdateDetails) -> CreateIssueRequest<'a> {
        CreateIssueRequest::new(self.client, issue_update_details)
    }

    /// Archives a list of issues.
    pub fn archive_issues(&self) -> ArchiveIssuesRequest<'a> {
        ArchiveIssuesRequest::new(self.client)
    }

    /// Creates issues or sub-tasks from a JSON representation. Creates many issues in one bulk operation.
    /// Creating a sub-task is similar to creating a regular issue. More details can be found in createIssue section.
    pub fn create_issues(&self, issues_update: IssuesUpdate) -> CreateIssuesRequest<'a> {
        CreateIssuesRequest::new(self.client, issues_update)
    }

    /// Returns the metadata for issue types used for creating issues. Data will not be returned if the user does not have permission to create issues in that project.
    pub fn get_create_issue_meta_issue_types(
        &self,
        project_id_or_key: impl Into<String>,
    ) -> GetCreateIssueMetaIssueTypesRequest<'a> {
        GetCreateIssueMetaIssueTypesRequest::new(self.client, project_id_or_key)
    }

    /// Returns the metadata for issue types used for creating issues. Data will not be returned if the user does not have permission to create issues in that project.
    pub fn get_create_issue_meta_issue_type_id(
        &self,
        project_id_or_key: impl Into<String>,
        issue_type_id: impl Into<String>,
    ) -> GetCreateIssueMetaIssueTypeIdRequest<'a> {
        GetCreateIssueMetaIssueTypeIdRequest::new(self.client, project_id_or_key, issue_type_id)
    }

    /// Create reciprocal remote issue link from a JSON representation. Jira will create two issue links, source -> target and target -> source.
    ///
    /// Available since Jira Data Center 10.7.
    pub fn create_reciprocal_remote_issue_link(&self) -> CreateReciprocalRemoteIssueLinkRequest<'a> {
        CreateReciprocalRemoteIssueLinkRequest::new(self.client)
    }

    /// Returns a full representation of the issue for the given issue key.
    /// An issue JSON consists of the issue key, a collection of fields,
    /// a link to the workflow transition sub-resource, and (optionally) the HTML rendered values of any fields that support it
    /// (e.g. if wiki syntax is enabled for the description or comments).
    /// The fields param (which can be specified multiple times) gives a comma-separated list of fields
    /// to include in the response. This can be used to retrieve a subset of fields.
    /// A particular field can be excluded by prefixing it with a minus.
    /// By default, all (*all) fields are returned in this get-issue resource. Note: the default is different
    /// when doing a jql search -- the default there is just navigable fields (*navigable).
    /// - *all - include all fields
    /// - *navigable - include just navigable fields
    /// - summary,comment - include just the summary and comments
    /// - -comment - include everything except comments (the default is *all for get-issue)
    /// - *all,-comment - include everything except comments
    ///
    /// The `properties` param is similar to `fields` and specifies a comma-separated list of issue
    /// properties to include. Unlike `fields`, properties are not included by default. To include them all
    /// send `?properties=*all`. You can also include only specified properties or exclude some properties
    /// with a minus (-) sign.
    ///
    /// - `*all` - include all properties
    /// - `*all, -prop1` - include all properties except `prop1`
    /// - `prop1, prop1` - include `prop1` and `prop2` properties
    ///
    /// Jira will attempt to identify the issue by the issueIdOrKey path parameter. This can be an issue id,
    /// or an issue key. If the issue cannot be found via an exact match, Jira will also look for the issue in a case-insensitive way,
    /// by looking to see if the issue was moved. In either of these cases, the request will proceed as normal (a 302 or other redirect
    /// will not be returned). The issue key contained in the response will indicate the current value of issue's key.
    ///
    /// The expand param is used to include, hidden by default, parts of response. This can be used to include:
    ///
    /// - renderedFields - field values in HTML format
    /// - names - display name of each field
    /// - schema - schema for each field which describes a type of the field
    /// - transitions - all possible transitions for the given issue
    /// - operations - all possibles operations which may be applied on issue
    /// - editmeta - information about how each field may be edited. It contains field's schema as well.
    /// - changelog - history of all changes of the given issue
    /// - versionedRepresentations -
    /// REST representations of all fields. Some field may contain more recent versions. RESET representations are numbered.
    /// The greatest number always represents the most recent version. It is recommended that the most recent version is used.
    /// version for these fields which provide a more recent REST representation.
    /// After including versionedRepresentations "fields" field become hidden.
    pub fn get_issue(&self, issue_id_or_key: impl Into<String>) -> IssuesGetIssueRequest<'a> {
        IssuesGetIssueRequest::new(self.client, issue_id_or_key)
    }

    /// Edits an issue from a JSON representation. The issue can either be updated by setting explicit the field value(s) or by using an operation to change the field value.
    pub fn edit_issue(
        &self,
        issue_id_or_key: impl Into<String>,
        issue_update_details: IssueUpdateDetails,
    ) -> EditIssueRequest<'a> {
        EditIssueRequest::new(self.client, issue_id_or_key, issue_update_details)
    }

    /// Deletes an issue. If the issue has subtasks you must set the parameter deleteSubtasks=true to delete the issue. You cannot delete an issue without its subtasks also being deleted.
    pub fn delete_issue(&self, issue_id_or_key: impl Into<String>) -> DeleteIssueRequest<'a> {
        DeleteIssueRequest::new(self.client, issue_id_or_key)
    }

    /// Archives an issue.
    pub fn archive_issue(&self, issue_id_or_key: impl Into<String>) -> ArchiveIssueRequest<'a> {
        ArchiveIssueRequest::new(self.client, issue_id_or_key)
    }

    /// Assign an issue to a user.
    pub fn assign_issue(&self, issue_id_or_key: impl Into<String>, user: User) -> AssignIssueRequest<'a> {
        AssignIssueRequest::new(self.client, issue_id_or_key, user)
    }

    /// Pins a comment to the top of the comment list.
    pub fn set_pin_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: bool,
    ) -> SetPinCommentRequest<'a> {
        SetPinCommentRequest::new(self.client, issue_id_or_key, id, body)
    }

    /// Returns the meta data for editing an issue. The fields in the editmeta correspond to the fields in the edit screen for the issue. Fields not in the screen will not be in the editmeta.
    pub fn get_edit_issue_meta(&self, issue_id_or_key: impl Into<String>) -> GetEditIssueMetaRequest<'a> {
        GetEditIssueMetaRequest::new(self.client, issue_id_or_key)
    }

    /// Sends a notification (email) to the list or recipients defined in the request.
    pub fn notify(&self, issue_id_or_key: impl Into<String>, notification: Notification) -> NotifyRequest<'a> {
        NotifyRequest::new(self.client, issue_id_or_key, notification)
    }

    /// Returns all pinned to the issue comments.
    pub fn get_pinned_comments(&self, issue_id_or_key: impl Into<String>) -> GetPinnedCommentsRequest<'a> {
        GetPinnedCommentsRequest::new(self.client, issue_id_or_key)
    }

    /// Restores an archived issue.
    pub fn restore_issue(&self, issue_id_or_key: impl Into<String>) -> RestoreIssueRequest<'a> {
        RestoreIssueRequest::new(self.client, issue_id_or_key)
    }

    /// Returns an issue's subtask list
    pub fn get_sub_tasks(&self, issue_id_or_key: impl Into<String>) -> GetSubTasksRequest<'a> {
        GetSubTasksRequest::new(self.client, issue_id_or_key)
    }

    /// Checks if a subtask can be moved
    pub fn can_move_sub_task(&self, issue_id_or_key: impl Into<String>) -> CanMoveSubTaskRequest<'a> {
        CanMoveSubTaskRequest::new(self.client, issue_id_or_key)
    }

    /// Reorders an issue's subtasks by moving the subtask at index 'from' to index 'to'.
    pub fn move_sub_tasks(
        &self,
        issue_id_or_key: impl Into<String>,
        issue_sub_task_move_position: IssueSubTaskMovePosition,
    ) -> MoveSubTasksRequest<'a> {
        MoveSubTasksRequest::new(self.client, issue_id_or_key, issue_sub_task_move_position)
    }

    /// Get a list of the transitions possible for this issue by the current user, along with fields that are required and their types.
    /// Fields will only be returned if `expand=transitions.fields`.
    /// The fields in the metadata correspond to the fields in the transition screen for that transition.
    /// Fields not in the screen will not be in the metadata.
    pub fn get_transitions(&self, issue_id_or_key: impl Into<String>) -> GetTransitionsRequest<'a> {
        GetTransitionsRequest::new(self.client, issue_id_or_key)
    }

    /// Perform a transition on an issue.
    /// When performing the transition you can update or set other issue fields.
    /// The fields that can be set on transition, in either the fields parameter or the update parameter can be determined using the /rest/api/2/issue/{issueIdOrKey}/transitions?expand=transitions.fields resource.
    /// If a field is not configured to appear on the transition screen, then it will not be in the transition metadata, and a field validation error will occur if it is submitted.
    /// The updateHistory param adds the issues retrieved by this method to the current user's issue history, if set to true (by default, the issue history does not include issues retrieved via the REST API).
    /// You can view the issue history in the Jira application, via the Issues dropdown or by using the lastViewed JQL field in an issue search.
    pub fn do_transition(
        &self,
        issue_id_or_key: impl Into<String>,
        issue_update_details: IssueUpdateDetails,
    ) -> DoTransitionRequest<'a> {
        DoTransitionRequest::new(self.client, issue_id_or_key, issue_update_details)
    }
}

/// Creates an issue or a sub-task from a JSON representation.
/// The fields that can be set on create, in either the fields parameter or the update parameter can be determined using the /rest/api/2/issue/createmeta resource.
/// If a field is not configured to appear on the create screen, then it will not be in the createmeta, and a field
/// validation error will occur if it is submitted.
/// Creating a sub-task is similar to creating a regular issue, with two important differences:
/// - the issueType field must correspond to a sub-task issue type (you can use /issue/createmeta to discover sub-task issue types), and
/// - you must provide a parent field in the issue create request containing the id or key of the parent issue.
/// The updateHistory param adds the project that this issue is created in, to the current user's project history, if set to true (by default, the project history is not updated).
/// You can view the project history in the Jira application, via the Projects dropdown.
#[derive(Clone)]
pub struct CreateIssueRequest<'a> {
    client: &'a crate::core::Client,
    update_history: Option<bool>,
    issue_update_details: IssueUpdateDetails,
}

impl<'a> CreateIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_update_details: IssueUpdateDetails) -> Self {
        Self { client, issue_update_details, update_history: None }
    }

    #[must_use]
    pub fn update_history(mut self, value: bool) -> Self {
        self.update_history = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/issue".to_owned());

        if let Some(value) = &self.update_history {
            config.query.push(("updateHistory".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.issue_update_details)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CreatedIssue> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Archives a list of issues.
#[derive(Clone)]
pub struct ArchiveIssuesRequest<'a> {
    client: &'a crate::core::Client,
    notify_users: Option<String>,
    body: Option<String>,
}

impl<'a> ArchiveIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, notify_users: None, body: None }
    }

    /// Send the email with notification that the issue was updated to users that watch it. Admin or project admin permissions are required to disable the notification.
    #[must_use]
    pub fn notify_users(mut self, value: impl Into<String>) -> Self {
        self.notify_users = Some(value.into());

        self
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/issue/archive".to_owned());

        if let Some(value) = &self.notify_users {
            config.query.push(("notifyUsers".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        config.content_type = Some("text/plain".to_owned());

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

/// Creates issues or sub-tasks from a JSON representation. Creates many issues in one bulk operation.
/// Creating a sub-task is similar to creating a regular issue. More details can be found in createIssue section.
#[derive(Clone)]
pub struct CreateIssuesRequest<'a> {
    client: &'a crate::core::Client,
    issues_update: IssuesUpdate,
}

impl<'a> CreateIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, issues_update: IssuesUpdate) -> Self {
        Self { client, issues_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/issue/bulk".to_owned());

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.issues_update)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CreatedIssues> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the metadata for issue types used for creating issues. Data will not be returned if the user does not have permission to create issues in that project.
#[derive(Clone)]
pub struct GetCreateIssueMetaIssueTypesRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    max_results: Option<i64>,
    start_at: Option<i64>,
}

impl<'a> GetCreateIssueMetaIssueTypesRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), max_results: None, start_at: None }
    }

    /// How many results on the page should be included
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The page offset
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issue/createmeta/{}/issuetypes",
                crate::core::encode_path_segment(&self.project_id_or_key)
            ),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<CreateMetaIssueType>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CreateMetaIssueType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the metadata for issue types used for creating issues. Data will not be returned if the user does not have permission to create issues in that project.
#[derive(Clone)]
pub struct GetCreateIssueMetaIssueTypeIdRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    issue_type_id: String,
    max_results: Option<i64>,
    start_at: Option<i64>,
}

impl<'a> GetCreateIssueMetaIssueTypeIdRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        issue_type_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
            issue_type_id: issue_type_id.into(),
            max_results: None,
            start_at: None,
        }
    }

    /// How many results on the page should be included
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The page offset
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issue/createmeta/{}/issuetypes/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                crate::core::encode_path_segment(&self.issue_type_id)
            ),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<FieldMeta>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<FieldMeta>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create reciprocal remote issue link from a JSON representation. Jira will create two issue links, source -> target and target -> source.
///
/// Available since Jira Data Center 10.7.
#[derive(Clone)]
pub struct CreateReciprocalRemoteIssueLinkRequest<'a> {
    client: &'a crate::core::Client,
    remote_reciprocal_issue_link_create_request: Option<RemoteReciprocalIssueLinkCreateRequest>,
}

impl<'a> CreateReciprocalRemoteIssueLinkRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, remote_reciprocal_issue_link_create_request: None }
    }

    #[must_use]
    pub fn remote_reciprocal_issue_link_create_request(
        mut self,
        value: RemoteReciprocalIssueLinkCreateRequest,
    ) -> Self {
        self.remote_reciprocal_issue_link_create_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/issue/remotelink/reciprocal".to_owned(),
        );

        let body = match serde_json::to_value(&self.remote_reciprocal_issue_link_create_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RemoteReciprocalIssueLinkCreateResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a full representation of the issue for the given issue key.
/// An issue JSON consists of the issue key, a collection of fields,
/// a link to the workflow transition sub-resource, and (optionally) the HTML rendered values of any fields that support it
/// (e.g. if wiki syntax is enabled for the description or comments).
/// The fields param (which can be specified multiple times) gives a comma-separated list of fields
/// to include in the response. This can be used to retrieve a subset of fields.
/// A particular field can be excluded by prefixing it with a minus.
/// By default, all (*all) fields are returned in this get-issue resource. Note: the default is different
/// when doing a jql search -- the default there is just navigable fields (*navigable).
/// - *all - include all fields
/// - *navigable - include just navigable fields
/// - summary,comment - include just the summary and comments
/// - -comment - include everything except comments (the default is *all for get-issue)
/// - *all,-comment - include everything except comments
///
/// The `properties` param is similar to `fields` and specifies a comma-separated list of issue
/// properties to include. Unlike `fields`, properties are not included by default. To include them all
/// send `?properties=*all`. You can also include only specified properties or exclude some properties
/// with a minus (-) sign.
///
/// - `*all` - include all properties
/// - `*all, -prop1` - include all properties except `prop1`
/// - `prop1, prop1` - include `prop1` and `prop2` properties
///
/// Jira will attempt to identify the issue by the issueIdOrKey path parameter. This can be an issue id,
/// or an issue key. If the issue cannot be found via an exact match, Jira will also look for the issue in a case-insensitive way,
/// by looking to see if the issue was moved. In either of these cases, the request will proceed as normal (a 302 or other redirect
/// will not be returned). The issue key contained in the response will indicate the current value of issue's key.
///
/// The expand param is used to include, hidden by default, parts of response. This can be used to include:
///
/// - renderedFields - field values in HTML format
/// - names - display name of each field
/// - schema - schema for each field which describes a type of the field
/// - transitions - all possible transitions for the given issue
/// - operations - all possibles operations which may be applied on issue
/// - editmeta - information about how each field may be edited. It contains field's schema as well.
/// - changelog - history of all changes of the given issue
/// - versionedRepresentations -
/// REST representations of all fields. Some field may contain more recent versions. RESET representations are numbered.
/// The greatest number always represents the most recent version. It is recommended that the most recent version is used.
/// version for these fields which provide a more recent REST representation.
/// After including versionedRepresentations "fields" field become hidden.
#[derive(Clone)]
pub struct IssuesGetIssueRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<IssuesGetIssueRequestExpand>,
    issue_id_or_key: String,
    fields: Option<Vec<String>>,
    update_history: Option<bool>,
    properties: Option<Vec<String>>,
}

impl<'a> IssuesGetIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            expand: None,
            fields: None,
            update_history: None,
            properties: None,
        }
    }

    /// The expand param is used to include, hidden by default, parts of response. This can be used to include: renderedFields, names, schema, transitions, operations, editmeta, changelog, versionedRepresentations.
    #[must_use]
    pub fn expand(mut self, value: IssuesGetIssueRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The list of fields to return for the issue. By default, all fields are returned.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The updateHistory param adds the issues retrieved by this method to the current user's issue history
    #[must_use]
    pub fn update_history(mut self, value: bool) -> Self {
        self.update_history = Some(value);

        self
    }

    /// The list of properties to return for the issue. By default no properties are returned.
    #[must_use]
    pub fn properties(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.properties = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.update_history {
            config.query.push(("updateHistory".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.properties {
            config.query.push(("properties".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Issue> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Edits an issue from a JSON representation. The issue can either be updated by setting explicit the field value(s) or by using an operation to change the field value.
#[derive(Clone)]
pub struct EditIssueRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    notify_users: Option<bool>,
    issue_update_details: IssueUpdateDetails,
}

impl<'a> EditIssueRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        issue_update_details: IssueUpdateDetails,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), issue_update_details, notify_users: None }
    }

    /// Send the email with notification that the issue was updated to users that watch it. Admin or project admin permissions are required to disable the notification.
    #[must_use]
    pub fn notify_users(mut self, value: bool) -> Self {
        self.notify_users = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issue/{}", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.notify_users {
            config.query.push(("notifyUsers".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.issue_update_details)?));

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

/// Deletes an issue. If the issue has subtasks you must set the parameter deleteSubtasks=true to delete the issue. You cannot delete an issue without its subtasks also being deleted.
#[derive(Clone)]
pub struct DeleteIssueRequest<'a> {
    client: &'a crate::core::Client,
    delete_subtasks: Option<DeleteIssueRequestDeleteSubtasks>,
    issue_id_or_key: String,
}

impl<'a> DeleteIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), delete_subtasks: None }
    }

    /// A String of true or false indicating that any subtasks should also be deleted. If the issue has no subtasks this parameter is ignored. If the issue has subtasks and this parameter is missing or false, then the issue will not be deleted and an error will be returned.
    #[must_use]
    pub fn delete_subtasks(mut self, value: impl Into<DeleteIssueRequestDeleteSubtasks>) -> Self {
        self.delete_subtasks = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issue/{}", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.delete_subtasks {
            config.query.push(("deleteSubtasks".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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

/// Archives an issue.
#[derive(Clone)]
pub struct ArchiveIssueRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    notify_users: Option<String>,
}

impl<'a> ArchiveIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), notify_users: None }
    }

    /// Send the email with notification that the issue was updated to users that watch it. Admin or project admin permissions are required to disable the notification.
    #[must_use]
    pub fn notify_users(mut self, value: impl Into<String>) -> Self {
        self.notify_users = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issue/{}/archive", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.notify_users {
            config.query.push(("notifyUsers".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Assign an issue to a user.
#[derive(Clone)]
pub struct AssignIssueRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    user: User,
}

impl<'a> AssignIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, user: User) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), user }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issue/{}/assignee", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.user)?));

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

/// Pins a comment to the top of the comment list.
#[derive(Clone)]
pub struct SetPinCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    id: String,
    body: bool,
}

impl<'a> SetPinCommentRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: bool,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), id: id.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/issue/{}/comment/{}/pin",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(self.body)?));

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

/// Returns the meta data for editing an issue. The fields in the editmeta correspond to the fields in the edit screen for the issue. Fields not in the screen will not be in the editmeta.
#[derive(Clone)]
pub struct GetEditIssueMetaRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetEditIssueMetaRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/editmeta", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueUpdateMetadata> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sends a notification (email) to the list or recipients defined in the request.
#[derive(Clone)]
pub struct NotifyRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    notification: Notification,
}

impl<'a> NotifyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, notification: Notification) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), notification }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/notify", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.notification)?));

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

/// Returns all pinned to the issue comments.
#[derive(Clone)]
pub struct GetPinnedCommentsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetPinnedCommentsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/pinned-comments", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<PinnedCommentJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Restores an archived issue.
#[derive(Clone)]
pub struct RestoreIssueRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    notify_users: Option<String>,
}

impl<'a> RestoreIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), notify_users: None }
    }

    /// Send the email with notification that the issue was updated to users that watch it. Admin or project admin permissions are required to disable the notification.
    #[must_use]
    pub fn notify_users(mut self, value: impl Into<String>) -> Self {
        self.notify_users = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issue/{}/restore", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.notify_users {
            config.query.push(("notifyUsers".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Returns an issue's subtask list
#[derive(Clone)]
pub struct GetSubTasksRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetSubTasksRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/subtask", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<IssueRefJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Checks if a subtask can be moved
#[derive(Clone)]
pub struct CanMoveSubTaskRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> CanMoveSubTaskRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/subtask/move", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<bool> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Reorders an issue's subtasks by moving the subtask at index 'from' to index 'to'.
#[derive(Clone)]
pub struct MoveSubTasksRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    issue_sub_task_move_position: IssueSubTaskMovePosition,
}

impl<'a> MoveSubTasksRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        issue_sub_task_move_position: IssueSubTaskMovePosition,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), issue_sub_task_move_position }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/subtask/move", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        let body = match serde_json::to_value(&self.issue_sub_task_move_position)? {
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

/// Get a list of the transitions possible for this issue by the current user, along with fields that are required and their types.
/// Fields will only be returned if `expand=transitions.fields`.
/// The fields in the metadata correspond to the fields in the transition screen for that transition.
/// Fields not in the screen will not be in the metadata.
#[derive(Clone)]
pub struct GetTransitionsRequest<'a> {
    client: &'a crate::core::Client,
    transition_id: Option<String>,
    issue_id_or_key: String,
}

impl<'a> GetTransitionsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), transition_id: None }
    }

    /// Transition id
    #[must_use]
    pub fn transition_id(mut self, value: impl Into<String>) -> Self {
        self.transition_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/transitions", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.transition_id {
            config.query.push(("transitionId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Transitions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Perform a transition on an issue.
/// When performing the transition you can update or set other issue fields.
/// The fields that can be set on transition, in either the fields parameter or the update parameter can be determined using the /rest/api/2/issue/{issueIdOrKey}/transitions?expand=transitions.fields resource.
/// If a field is not configured to appear on the transition screen, then it will not be in the transition metadata, and a field validation error will occur if it is submitted.
/// The updateHistory param adds the issues retrieved by this method to the current user's issue history, if set to true (by default, the issue history does not include issues retrieved via the REST API).
/// You can view the issue history in the Jira application, via the Issues dropdown or by using the lastViewed JQL field in an issue search.
#[derive(Clone)]
pub struct DoTransitionRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    issue_update_details: IssueUpdateDetails,
}

impl<'a> DoTransitionRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        issue_update_details: IssueUpdateDetails,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), issue_update_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/transitions", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.issue_update_details)?));

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
