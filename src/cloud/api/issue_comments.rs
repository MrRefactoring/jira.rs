// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetCommentsByIdsRequestExpandValue {
        RenderedBody => "renderedBody",
        Properties => "properties",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `renderedBody` Returns the comment body rendered in HTML.
///  *  `properties` Returns the comment's properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetCommentsByIdsRequestExpand {
    One(GetCommentsByIdsRequestExpandValue),
    Many(Vec<GetCommentsByIdsRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field. Accepts *created* to sort comments by their created date.
    pub enum GetCommentsRequestOrderBy {
        Created => "created",
        CreatedDescending => "-created",
        CreatedAscending => "+created",
    }
}

crate::open_enum! {
    pub enum GetCommentsRequestExpandValue {
        RenderedBody => "renderedBody",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetCommentsRequestExpand {
    One(GetCommentsRequestExpandValue),
    Many(Vec<GetCommentsRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum AddCommentRequestExpandValue {
        RenderedBody => "renderedBody",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AddCommentRequestExpand {
    One(AddCommentRequestExpandValue),
    Many(Vec<AddCommentRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetCommentRequestExpandValue {
        RenderedBody => "renderedBody",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetCommentRequestExpand {
    One(GetCommentRequestExpandValue),
    Many(Vec<GetCommentRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum UpdateCommentRequestExpandValue {
        RenderedBody => "renderedBody",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UpdateCommentRequestExpand {
    One(UpdateCommentRequestExpandValue),
    Many(Vec<UpdateCommentRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The IssueComments operations.
pub struct IssueCommentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueCommentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of comments specified by a list of comment IDs.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Comments are returned where the user:
    ///
    ///  *  has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.
    pub fn get_comments_by_ids(
        &self,
        issue_comment_list_request: IssueCommentListRequest,
    ) -> GetCommentsByIdsRequest<'a> {
        GetCommentsByIdsRequest::new(self.client, issue_comment_list_request)
    }

    /// Returns all comments for an issue.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Comments are included in the response where the user has:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is role visibility is restricted to.
    pub fn get_comments(&self, issue_id_or_key: impl Into<String>) -> GetCommentsRequest<'a> {
        GetCommentsRequest::new(self.client, issue_id_or_key)
    }

    /// Adds a comment to an issue.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Browse projects* and *Add comments* [ project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn add_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        comment_input: CommentInput,
    ) -> AddCommentRequest<'a> {
        AddCommentRequest::new(self.client, issue_id_or_key, comment_input)
    }

    /// Returns a comment.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.
    pub fn get_comment(&self, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> GetCommentRequest<'a> {
        GetCommentRequest::new(self.client, issue_id_or_key, id)
    }

    /// Updates a comment.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  *Edit all comments*[ project permission](https://confluence.atlassian.com/x/yodKLg) to update any comment or *Edit own comments* to update comment created by the user.
    ///  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.
    ///
    /// **WARNING:** Child comments inherit visibility from their parent comment. Attempting to update a child comment's visibility will result in a 400 (Bad Request) error.
    pub fn update_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: CommentInput,
    ) -> UpdateCommentRequest<'a> {
        UpdateCommentRequest::new(self.client, issue_id_or_key, id, body)
    }

    /// Deletes a comment.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  *Delete all comments*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any comment or *Delete own comments* to delete comment created by the user,
    ///  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.
    pub fn delete_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
    ) -> DeleteCommentRequest<'a> {
        DeleteCommentRequest::new(self.client, issue_id_or_key, id)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of comments specified by a list of comment IDs.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Comments are returned where the user:
///
///  *  has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.
pub struct GetCommentsByIdsRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetCommentsByIdsRequestExpand>,
    issue_comment_list_request: IssueCommentListRequest,
}

impl<'a> GetCommentsByIdsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_comment_list_request: IssueCommentListRequest) -> Self {
        Self { client, issue_comment_list_request, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `renderedBody` Returns the comment body rendered in HTML.
    ///  *  `properties` Returns the comment's properties.
    #[must_use]
    pub fn expand(mut self, value: GetCommentsByIdsRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/comment/list".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        let body = match serde_json::to_value(&self.issue_comment_list_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
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

/// Returns all comments for an issue.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Comments are included in the response where the user has:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is role visibility is restricted to.
pub struct GetCommentsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start_at: Option<i64>,
    max_results: Option<i64>,
    order_by: Option<GetCommentsRequestOrderBy>,
    expand: Option<GetCommentsRequestExpand>,
}

impl<'a> GetCommentsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            start_at: None,
            max_results: None,
            order_by: None,
            expand: None,
        }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field. Accepts *created* to sort comments by their created date.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetCommentsRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
    #[must_use]
    pub fn expand(mut self, value: GetCommentsRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/issue/{}/comment", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageOfComments> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a comment to an issue.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Browse projects* and *Add comments* [ project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct AddCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    expand: Option<AddCommentRequestExpand>,
    comment_input: CommentInput,
}

impl<'a> AddCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, comment_input: CommentInput) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), comment_input, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
    #[must_use]
    pub fn expand(mut self, value: AddCommentRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/issue/{}/comment", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        let body = match serde_json::to_value(&self.comment_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Comment> {
        // A plain string is wiki markup, which v3 cannot read. v2 takes it, converts it, and the re-read
        // below hands the caller the document v3 made of it.
        if let Some(CommentInputBody::Variant1(markup)) = &self.comment_input.body {
            let mut write = crate::core::RequestConfig::new(
                crate::core::Method::POST,
                format!("/rest/api/2/issue/{}/comment", self.issue_id_or_key),
            );

            write.body = Some(crate::core::Body::Json(serde_json::json!({
                "body": markup,
                "visibility": self.comment_input.visibility,
            })));

            let created: serde_json::Value = self.client.send(&write).await?;
            let id = created["id"].as_str().unwrap_or_default().to_owned();

            let mut read = crate::core::RequestConfig::new(
                crate::core::Method::GET,
                format!("/rest/api/3/issue/{}/comment/{}", self.issue_id_or_key, id),
            );

            if let Some(expand) = &self.expand {
                read.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(expand)?));
            }

            return self.client.send(&read).await;
        }

        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a comment.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the comment.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.
pub struct GetCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    id: String,
    expand: Option<GetCommentRequestExpand>,
}

impl<'a> GetCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), id: id.into(), expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
    #[must_use]
    pub fn expand(mut self, value: GetCommentRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/issue/{}/comment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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

/// Updates a comment.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  *Edit all comments*[ project permission](https://confluence.atlassian.com/x/yodKLg) to update any comment or *Edit own comments* to update comment created by the user.
///  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.
///
/// **WARNING:** Child comments inherit visibility from their parent comment. Attempting to update a child comment's visibility will result in a 400 (Bad Request) error.
pub struct UpdateCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    id: String,
    notify_users: Option<bool>,
    override_editable_flag: Option<bool>,
    expand: Option<UpdateCommentRequestExpand>,
    body: CommentInput,
}

impl<'a> UpdateCommentRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: CommentInput,
    ) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            id: id.into(),
            body,
            notify_users: None,
            override_editable_flag: None,
            expand: None,
        }
    }

    /// Whether users are notified when a comment is updated.
    #[must_use]
    pub fn notify_users(mut self, value: bool) -> Self {
        self.notify_users = Some(value);

        self
    }

    /// Whether screen security is overridden to enable uneditable fields to be edited. Available to Connect app users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) and Forge apps acting on behalf of users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    #[must_use]
    pub fn override_editable_flag(mut self, value: bool) -> Self {
        self.override_editable_flag = Some(value);

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about comments in the response. This parameter accepts `renderedBody`, which returns the comment body rendered in HTML.
    #[must_use]
    pub fn expand(mut self, value: UpdateCommentRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/issue/{}/comment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        if let Some(value) = &self.notify_users {
            config.query.push(("notifyUsers".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.override_editable_flag {
            config.query.push(("overrideEditableFlag".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Comment> {
        if let Some(CommentInputBody::Variant1(markup)) = &self.body.body {
            let mut write = crate::core::RequestConfig::new(
                crate::core::Method::PUT,
                format!("/rest/api/2/issue/{}/comment/{}", self.issue_id_or_key, self.id),
            );

            write.body = Some(crate::core::Body::Json(serde_json::json!({
                "body": markup,
                "visibility": self.body.visibility,
            })));

            self.client.send_empty(&write).await?;

            let mut read = crate::core::RequestConfig::new(
                crate::core::Method::GET,
                format!("/rest/api/3/issue/{}/comment/{}", self.issue_id_or_key, self.id),
            );

            if let Some(expand) = &self.expand {
                read.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(expand)?));
            }

            return self.client.send(&read).await;
        }

        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a comment.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue containing the comment is in.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  *Delete all comments*[ project permission](https://confluence.atlassian.com/x/yodKLg) to delete any comment or *Delete own comments* to delete comment created by the user,
///  *  If the comment has visibility restrictions, the user belongs to the group or has the role visibility is restricted to.
pub struct DeleteCommentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    id: String,
}

impl<'a> DeleteCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/issue/{}/comment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
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
