// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueWorklogs operations.
pub struct IssueWorklogsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueWorklogsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns worklogs id and delete time of worklogs that was deleted since given time. The returns set of worklogs is limited to 1000 elements. This API will not return worklogs deleted during last minute.
    pub fn get_ids_of_worklogs_deleted_since(&self) -> GetIdsOfWorklogsDeletedSinceRequest<'a> {
        GetIdsOfWorklogsDeletedSinceRequest::new(self.client)
    }

    /// Returns worklogs for given worklog ids. Only worklogs to which the calling user has permissions, will be included in the result. The returns set of worklogs is limited to 1000 elements.
    pub fn get_worklogs_for_ids(&self, worklog_ids_request: WorklogIdsRequest) -> GetWorklogsForIdsRequest<'a> {
        GetWorklogsForIdsRequest::new(self.client, worklog_ids_request)
    }

    /// Returns worklogs id and update time of worklogs that was updated since given time. The returns set of worklogs is limited to 1000 elements. This API will not return worklogs updated during last minute.
    pub fn get_ids_of_worklogs_modified_since(&self) -> GetIdsOfWorklogsModifiedSinceRequest<'a> {
        GetIdsOfWorklogsModifiedSinceRequest::new(self.client)
    }
}

/// Returns worklogs id and delete time of worklogs that was deleted since given time. The returns set of worklogs is limited to 1000 elements. This API will not return worklogs deleted during last minute.
#[derive(Clone)]
pub struct GetIdsOfWorklogsDeletedSinceRequest<'a> {
    client: &'a crate::core::Client,
    since: Option<i64>,
}

impl<'a> GetIdsOfWorklogsDeletedSinceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, since: None }
    }

    /// a date time in unix timestamp format since when deleted worklogs will be returned.
    #[must_use]
    pub fn since(mut self, value: i64) -> Self {
        self.since = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/worklog/deleted".to_owned());

        if let Some(value) = &self.since {
            config.query.push(("since".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<WorklogChange>> {
        let first = self.since;

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.since = position;
            let page = request.send().await?;
            let items = page.values.unwrap_or_default();
            Ok(crate::core::PageStep::cursor(
                items,
                &position,
                page.until.map(Some),
                page.is_last_page.unwrap_or(false),
            ))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorklogChangedSince> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns worklogs for given worklog ids. Only worklogs to which the calling user has permissions, will be included in the result. The returns set of worklogs is limited to 1000 elements.
#[derive(Clone)]
pub struct GetWorklogsForIdsRequest<'a> {
    client: &'a crate::core::Client,
    worklog_ids_request: WorklogIdsRequest,
}

impl<'a> GetWorklogsForIdsRequest<'a> {
    fn new(client: &'a crate::core::Client, worklog_ids_request: WorklogIdsRequest) -> Self {
        Self { client, worklog_ids_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/worklog/list".to_owned());

        let body = match serde_json::to_value(&self.worklog_ids_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Worklog>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns worklogs id and update time of worklogs that was updated since given time. The returns set of worklogs is limited to 1000 elements. This API will not return worklogs updated during last minute.
#[derive(Clone)]
pub struct GetIdsOfWorklogsModifiedSinceRequest<'a> {
    client: &'a crate::core::Client,
    since: Option<i64>,
}

impl<'a> GetIdsOfWorklogsModifiedSinceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, since: None }
    }

    /// a date time in unix timestamp format since when updated worklogs will be returned.
    #[must_use]
    pub fn since(mut self, value: i64) -> Self {
        self.since = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/worklog/updated".to_owned());

        if let Some(value) = &self.since {
            config.query.push(("since".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<WorklogChange>> {
        let first = self.since;

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.since = position;
            let page = request.send().await?;
            let items = page.values.unwrap_or_default();
            Ok(crate::core::PageStep::cursor(
                items,
                &position,
                page.until.map(Some),
                page.is_last_page.unwrap_or(false),
            ))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorklogChangedSince> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
