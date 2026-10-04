// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// Allows you to provide specific instructions to update the remaining time estimate of the issue. Valid values are: new, leave, manual, auto
    pub enum AddWorklogRequestAdjustEstimate {
        New => "new",
        Leave => "leave",
        Manual => "manual",
        Auto => "auto",
    }
}

crate::open_enum! {
    /// allows you to provide specific instructions to update the remaining time estimate of the issue. Valid values are: new, leave, auto
    pub enum UpdateWorklogRequestAdjustEstimate {
        New => "new",
        Leave => "leave",
        Auto => "auto",
    }
}

crate::open_enum! {
    /// Allows you to provide specific instructions to update the remaining time estimate of the issue. Valid values are: new, leave, manual, auto
    pub enum DeleteWorklogRequestAdjustEstimate {
        New => "new",
        Leave => "leave",
        Manual => "manual",
        Auto => "auto",
    }
}

/// The IssueWorklogs operations.
pub struct IssueWorklogsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueWorklogsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all work logs for an issue. Work logs won't be returned if the Log work field is hidden for the project.
    pub fn get_issue_worklog(&self, issue_id_or_key: impl Into<String>) -> GetIssueWorklogRequest<'a> {
        GetIssueWorklogRequest::new(self.client, issue_id_or_key)
    }

    /// Adds a new worklog entry to an issue.
    pub fn add_worklog(&self, issue_id_or_key: impl Into<String>, worklog_input: Worklog) -> AddWorklogRequest<'a> {
        AddWorklogRequest::new(self.client, issue_id_or_key, worklog_input)
    }

    /// Returns a specific worklog. The work log won't be returned if the Log work field is hidden for the project.
    pub fn get_worklog(&self, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> GetWorklogRequest<'a> {
        GetWorklogRequest::new(self.client, issue_id_or_key, id)
    }

    /// Updates an existing worklog entry. Note that:
    /// - Fields possible for editing are: comment, visibility, started, timeSpent and timeSpentSeconds.
    /// - Either timeSpent or timeSpentSeconds can be set.
    /// - Fields which are not set will not be updated.
    /// - For a request to be valid, it has to have at least one field change.
    pub fn update_worklog(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: Worklog,
    ) -> UpdateWorklogRequest<'a> {
        UpdateWorklogRequest::new(self.client, issue_id_or_key, id, body)
    }

    /// Deletes an existing worklog entry.
    pub fn delete_worklog(
        &self,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
    ) -> DeleteWorklogRequest<'a> {
        DeleteWorklogRequest::new(self.client, issue_id_or_key, id)
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

/// Returns all work logs for an issue. Work logs won't be returned if the Log work field is hidden for the project.
#[derive(Clone)]
pub struct GetIssueWorklogRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetIssueWorklogRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/worklog", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Worklog>> {
        let first = ();

        crate::core::stream_pages(self, first, |request, _position| async move {
            let page = request.send().await?;
            let items = page.worklogs.unwrap_or_default();
            Ok(crate::core::PageStep::new(items, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageOfWorklogs> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a new worklog entry to an issue.
#[derive(Clone)]
pub struct AddWorklogRequest<'a> {
    client: &'a crate::core::Client,
    new_estimate: Option<String>,
    adjust_estimate: Option<AddWorklogRequestAdjustEstimate>,
    reduce_by: Option<String>,
    issue_id_or_key: String,
    worklog_input: Worklog,
}

impl<'a> AddWorklogRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, worklog_input: Worklog) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            worklog_input,
            new_estimate: None,
            adjust_estimate: None,
            reduce_by: None,
        }
    }

    /// Required when 'new' is selected for adjustEstimate. e.g. "2d"
    #[must_use]
    pub fn new_estimate(mut self, value: impl Into<String>) -> Self {
        self.new_estimate = Some(value.into());

        self
    }

    /// Allows you to provide specific instructions to update the remaining time estimate of the issue. Valid values are: new, leave, manual, auto
    #[must_use]
    pub fn adjust_estimate(mut self, value: impl Into<AddWorklogRequestAdjustEstimate>) -> Self {
        self.adjust_estimate = Some(value.into());

        self
    }

    /// Required when 'manual' is selected for adjustEstimate. e.g. "2d"
    #[must_use]
    pub fn reduce_by(mut self, value: impl Into<String>) -> Self {
        self.reduce_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/worklog", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.new_estimate {
            config.query.push(("newEstimate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.adjust_estimate {
            config.query.push(("adjustEstimate".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.reduce_by {
            config.query.push(("reduceBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.worklog_input)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Worklog> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a specific worklog. The work log won't be returned if the Log work field is hidden for the project.
#[derive(Clone)]
pub struct GetWorklogRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    id: String,
}

impl<'a> GetWorklogRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issue/{}/worklog/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Worklog> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates an existing worklog entry. Note that:
/// - Fields possible for editing are: comment, visibility, started, timeSpent and timeSpentSeconds.
/// - Either timeSpent or timeSpentSeconds can be set.
/// - Fields which are not set will not be updated.
/// - For a request to be valid, it has to have at least one field change.
#[derive(Clone)]
pub struct UpdateWorklogRequest<'a> {
    client: &'a crate::core::Client,
    new_estimate: Option<String>,
    adjust_estimate: Option<UpdateWorklogRequestAdjustEstimate>,
    issue_id_or_key: String,
    id: String,
    body: Worklog,
}

impl<'a> UpdateWorklogRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        id: impl Into<String>,
        body: Worklog,
    ) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            id: id.into(),
            body,
            new_estimate: None,
            adjust_estimate: None,
        }
    }

    /// required when 'new' is selected for adjustEstimate
    #[must_use]
    pub fn new_estimate(mut self, value: impl Into<String>) -> Self {
        self.new_estimate = Some(value.into());

        self
    }

    /// allows you to provide specific instructions to update the remaining time estimate of the issue. Valid values are: new, leave, auto
    #[must_use]
    pub fn adjust_estimate(mut self, value: impl Into<UpdateWorklogRequestAdjustEstimate>) -> Self {
        self.adjust_estimate = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/issue/{}/worklog/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        if let Some(value) = &self.new_estimate {
            config.query.push(("newEstimate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.adjust_estimate {
            config.query.push(("adjustEstimate".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Worklog> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an existing worklog entry.
#[derive(Clone)]
pub struct DeleteWorklogRequest<'a> {
    client: &'a crate::core::Client,
    new_estimate: Option<String>,
    adjust_estimate: Option<DeleteWorklogRequestAdjustEstimate>,
    issue_id_or_key: String,
    id: String,
    increase_by: Option<String>,
}

impl<'a> DeleteWorklogRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            id: id.into(),
            new_estimate: None,
            adjust_estimate: None,
            increase_by: None,
        }
    }

    /// Required when 'new' is selected for adjustEstimate. e.g. "2d"
    #[must_use]
    pub fn new_estimate(mut self, value: impl Into<String>) -> Self {
        self.new_estimate = Some(value.into());

        self
    }

    /// Allows you to provide specific instructions to update the remaining time estimate of the issue. Valid values are: new, leave, manual, auto
    #[must_use]
    pub fn adjust_estimate(mut self, value: impl Into<DeleteWorklogRequestAdjustEstimate>) -> Self {
        self.adjust_estimate = Some(value.into());

        self
    }

    /// Required when 'manual' is selected for adjustEstimate. e.g. "2d"
    #[must_use]
    pub fn increase_by(mut self, value: impl Into<String>) -> Self {
        self.increase_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/issue/{}/worklog/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        if let Some(value) = &self.new_estimate {
            config.query.push(("newEstimate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.adjust_estimate {
            config.query.push(("adjustEstimate".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.increase_by {
            config.query.push(("increaseBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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
    pub async fn send(self) -> crate::core::Result<ChangedWorklogs> {
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
    pub async fn send(self) -> crate::core::Result<ChangedWorklogs> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
