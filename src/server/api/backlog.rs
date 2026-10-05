// @generated. Do not edit: change the generator or the specification.

/// The Backlog operations.
pub struct BacklogService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> BacklogService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Move issues to the backlog. This operation is equivalent to remove future and active sprints from a given set of issues. At most 50 issues may be moved at once.
    pub fn move_issues_to_backlog(
        &self,
        issues: impl IntoIterator<Item = impl Into<String>>,
    ) -> MoveIssuesToBacklogRequest<'a> {
        MoveIssuesToBacklogRequest::new(self.client, issues)
    }
}

/// Move issues to the backlog. This operation is equivalent to remove future and active sprints from a given set of issues. At most 50 issues may be moved at once.
#[derive(Clone)]
pub struct MoveIssuesToBacklogRequest<'a> {
    client: &'a crate::core::Client,
    issues: Vec<String>,
}

impl<'a> MoveIssuesToBacklogRequest<'a> {
    fn new(client: &'a crate::core::Client, issues: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { client, issues: issues.into_iter().map(Into::into).collect() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/agile/1.0/backlog/issue".to_owned());

        let mut body = serde_json::Map::new();

        body.insert("issues".to_owned(), serde_json::to_value(&self.issues)?);

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
