// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// Optional flags: renderedBody (provides body rendered in HTML)
    pub enum GetCommentsRequestExpand {
        RenderedBody => "renderedBody",
    }
}

crate::open_enum! {
    /// Optional flags: renderedBody (provides body rendered in HTML)
    pub enum AddCommentRequestExpand {
        RenderedBody => "renderedBody",
    }
}

crate::open_enum! {
    /// Optional flags: renderedBody (provides body rendered in HTML)
    pub enum GetCommentRequestExpand {
        RenderedBody => "renderedBody",
    }
}

crate::open_enum! {
    /// Optional flags: renderedBody (provides body rendered in HTML)
    pub enum UpdateCommentRequestExpand {
        RenderedBody => "renderedBody",
    }
}

/// The IssueComments operations.
pub struct IssueCommentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueCommentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all comments for an issue. Results can be ordered by the 'created' field which means the date a comment was added.
    pub fn get_comments(&self, issue_id_or_key: impl Into<String>) -> GetCommentsRequest<'a> {
        GetCommentsRequest::new(self.client, issue_id_or_key)
    }

    /// Adds a new comment to an issue.
    pub fn add_comment(&self, issue_id_or_key: impl Into<String>, comment_input: CommentJson) -> AddCommentRequest<'a> {
        AddCommentRequest::new(self.client, issue_id_or_key, comment_input)
    }

    /// Returns a single comment.
    pub fn get_comment(&self, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> GetCommentRequest<'a> {
        GetCommentRequest::new(self.client, issue_id_or_key, id)
    }

    /// Updates an existing comment using its JSON representation.
    pub fn update_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: CommentJson,
    ) -> UpdateCommentRequest<'a> {
        UpdateCommentRequest::new(self.client, issue_id_or_key, id, body)
    }

    /// Deletes an existing comment.
    pub fn delete_comment(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
    ) -> DeleteCommentRequest<'a> {
        DeleteCommentRequest::new(self.client, issue_id_or_key, id)
    }
}

/// Returns all comments for an issue. Results can be ordered by the 'created' field which means the date a comment was added.
#[derive(Clone)]
pub struct GetCommentsRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetCommentsRequestExpand>,
    max_results: Option<i64>,
    issue_id_or_key: String,
    order_by: Option<String>,
    start_at: Option<i64>,
}

impl<'a> GetCommentsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            expand: None,
            max_results: None,
            order_by: None,
            start_at: None,
        }
    }

    /// Optional flags: renderedBody (provides body rendered in HTML)
    #[must_use]
    pub fn expand(mut self, value: impl Into<GetCommentsRequestExpand>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// How many results on the page should be included. Defaults to 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Ordering of the results
    #[must_use]
    pub fn order_by(mut self, value: impl Into<String>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The page offset, if not specified then defaults to 0
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/comment", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<CommentJson>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.comments.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CommentsWithPaginationJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a new comment to an issue.
#[derive(Clone)]
pub struct AddCommentRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<AddCommentRequestExpand>,
    issue_id_or_key: String,
    comment_input: CommentJson,
}

impl<'a> AddCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, comment_input: CommentJson) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), comment_input, expand: None }
    }

    /// Optional flags: renderedBody (provides body rendered in HTML)
    #[must_use]
    pub fn expand(mut self, value: impl Into<AddCommentRequestExpand>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/comment", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.comment_input)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CommentJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a single comment.
#[derive(Clone)]
pub struct GetCommentRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetCommentRequestExpand>,
    issue_id_or_key: String,
    id: String,
}

impl<'a> GetCommentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), id: id.into(), expand: None }
    }

    /// Optional flags: renderedBody (provides body rendered in HTML)
    #[must_use]
    pub fn expand(mut self, value: impl Into<GetCommentRequestExpand>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issue/{}/comment/{}",
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
    pub async fn send(self) -> crate::core::Result<CommentJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates an existing comment using its JSON representation.
#[derive(Clone)]
pub struct UpdateCommentRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<UpdateCommentRequestExpand>,
    issue_id_or_key: String,
    id: String,
    body: CommentJson,
}

impl<'a> UpdateCommentRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: CommentJson,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), id: id.into(), body, expand: None }
    }

    /// Optional flags: renderedBody (provides body rendered in HTML)
    #[must_use]
    pub fn expand(mut self, value: impl Into<UpdateCommentRequestExpand>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/issue/{}/comment/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CommentJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an existing comment.
#[derive(Clone)]
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
                "/rest/api/2/issue/{}/comment/{}",
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
