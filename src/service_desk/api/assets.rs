// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Assets operations.
pub struct AssetsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AssetsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of Assets workspace IDs. Include a workspace ID in the path to access the [Assets REST APIs](https://developer.atlassian.com/cloud/assets/rest).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
    pub fn get_assets_workspaces(&self) -> GetAssetsWorkspacesRequest<'a> {
        GetAssetsWorkspacesRequest::new(self.client)
    }

    /// This endpoint is deprecated, please use /assets/workspace/.
    ///
    /// Use [`get_assets_workspaces`](Self::get_assets_workspaces) instead.
    #[deprecated(
        note = "This endpoint is deprecated, please use /assets/workspace/. In this crate that is `get_assets_workspaces`."
    )]
    pub fn get_insight_workspaces(&self) -> GetInsightWorkspacesRequest<'a> {
        GetInsightWorkspacesRequest::new(self.client)
    }
}

/// Returns a list of Assets workspace IDs. Include a workspace ID in the path to access the [Assets REST APIs](https://developer.atlassian.com/cloud/assets/rest).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Any
#[derive(Clone)]
pub struct GetAssetsWorkspacesRequest<'a> {
    client: &'a crate::core::Client,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetAssetsWorkspacesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start: None, limit: None }
    }

    /// The starting index of the returned workspace IDs. Base index: 0 See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of workspace IDs to return per page. Default: 50 See the [Pagination](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#pagination) section for more details.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/servicedeskapi/assets/workspace".to_owned(),
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
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<AssetsWorkspace>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(
                items,
                page.start.unwrap_or(position),
                page.is_last_page.unwrap_or(false),
                None,
                None,
            ))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<AssetsWorkspace>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This endpoint is deprecated, please use /assets/workspace/.
#[derive(Clone)]
pub struct GetInsightWorkspacesRequest<'a> {
    client: &'a crate::core::Client,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetInsightWorkspacesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start: None, limit: None }
    }

    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/servicedeskapi/insight/workspace".to_owned(),
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
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<InsightWorkspace>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(
                items,
                page.start.unwrap_or(position),
                page.is_last_page.unwrap_or(false),
                None,
                None,
            ))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<InsightWorkspace>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
