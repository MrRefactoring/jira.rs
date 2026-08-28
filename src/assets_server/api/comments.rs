// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Comments operations.
pub struct CommentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CommentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Add a comment to an object.
    pub fn create_comment(&self) -> CreateCommentRequest<'a> {
        CreateCommentRequest::new(self.client)
    }

    /// Get the comments for an object by object ID.
    pub fn get_comments(&self, object_id: impl Into<String>) -> GetCommentsRequest<'a> {
        GetCommentsRequest::new(self.client, object_id)
    }
}

/// Add a comment to an object.
pub struct CreateCommentRequest<'a> {
    client: &'a crate::core::Client,
    comment: Option<Comment>,
}

impl<'a> CreateCommentRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, comment: None }
    }

    #[must_use]
    pub fn comment(mut self, value: Comment) -> Self {
        self.comment = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/assets/1.0/comment/create".to_owned());

        let body = match serde_json::to_value(&self.comment)? {
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

/// Get the comments for an object by object ID.
pub struct GetCommentsRequest<'a> {
    client: &'a crate::core::Client,
    asc: Option<String>,
    object_id: String,
}

impl<'a> GetCommentsRequest<'a> {
    fn new(client: &'a crate::core::Client, object_id: impl Into<String>) -> Self {
        Self { client, object_id: object_id.into(), asc: None }
    }

    /// Whether to sort ascending
    #[must_use]
    pub fn asc(mut self, value: impl Into<String>) -> Self {
        self.asc = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/comment/object/{}", crate::core::encode_path_segment(&self.object_id)),
        );

        if let Some(value) = &self.asc {
            config.query.push(("asc".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Comment>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
