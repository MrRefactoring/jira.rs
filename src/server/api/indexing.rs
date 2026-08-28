// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Indexing operations.
pub struct IndexingService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IndexingService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Lists available index snapshots absolute paths with timestamps
    pub fn list_index_snapshot(&self) -> ListIndexSnapshotRequest<'a> {
        ListIndexSnapshotRequest::new(self.client)
    }

    /// Starts taking an index snapshot if no other snapshot creation process is in progress
    pub fn create_index_snapshot(&self) -> CreateIndexSnapshotRequest<'a> {
        CreateIndexSnapshotRequest::new(self.client)
    }

    /// Checks if index snapshot creation is currently running
    pub fn is_index_snapshot_running(&self) -> IsIndexSnapshotRunningRequest<'a> {
        IsIndexSnapshotRunningRequest::new(self.client)
    }

    /// Returns a summary of the index condition of the current node.
    /// The returned data consists of:
    /// - `nodeId` - Node identifier.
    /// - `reportTime` - Time of this report creation.
    /// - `issueIndex` - Summary of the issue index status.
    /// - `replicationQueues` - Map of index replication queues, where keys represent nodes from which replication operations came from.
    /// - `externalPlatformIndexReplay` - Map of external platform's index replay queues, where keys represents which node logged to index replay queue (journal)
    ///
    /// `issueIndex` can contain:
    ///     - `indexReadable` - If `false` the endpoint failed to read data from the issue index (check Jira logs for detailed stack trace), otherwise `true`.
    ///     - `countInDatabase` - Count of issues found in the database.
    ///     - `countInIndex` - Count of issues found while querying the index.
    ///     - `lastUpdatedInDatabase` - Time of the last update of the issue found in the database.
    ///     - `lastUpdatedInIndex` - Time of the last update of the issue found while querying the index.
    /// `replicationQueues`'s map values can contain:
    ///     - `lastConsumedOperation` - Last executed index replication operation by the current node from the sending node's queue.
    ///     - `lastConsumedOperation.id` - Identifier of the operation.
    ///     - `lastConsumedOperation.replicationTime` - Time when the operation was sent to other nodes.
    ///     - `lastOperationInQueue` - Last index replication operation in the sending node's queue.
    ///     - `lastOperationInQueue.id` - Identifier of the operation.
    ///     - `lastOperationInQueue.replicationTime` - Time when the operation was sent to other nodes.
    ///     - `queueSize` - Number of operations in the queue from the sending node to the current node.
    /// `externalPlatformIndexReplay`'s map values can contain:
    ///     - `lastConsumedOperation` - Last executed external platform's index replay operation.
    ///     - `lastConsumedOperation.id` - Identifier of the operation.
    ///     - `lastConsumedOperation.journalWriteTime` - Time when the operation was written to the journal.
    ///     - `lastOperationInQueue` - Last external platform's index replay operation in the replay queue.
    ///     - `lastOperationInQueue.id` - Identifier of the operation.
    ///     - `lastOperationInQueue.journalWriteTime` - Time when the operation was written to the journal.
    ///     - `queueSize` - Number of operations in the queue awaiting synchronization with the external platform's index.
    pub fn get_index_summary(&self) -> GetIndexSummaryRequest<'a> {
        GetIndexSummaryRequest::new(self.client)
    }

    /// Returns information on the system reindexes. If a reindex is currently taking place then information about this reindex is returned. If there is no active index task, then returns information about the latest reindex task run, otherwise returns a 404 indicating that no reindex has taken place.
    pub fn get_reindex_info(&self) -> GetReindexInfoRequest<'a> {
        GetReindexInfoRequest::new(self.client)
    }

    /// Kicks off a reindex. Need Admin permissions to perform this reindex.
    pub fn reindex(&self) -> ReindexRequest2<'a> {
        ReindexRequest2::new(self.client)
    }

    /// Reindexes one or more individual issues. Indexing is performed synchronously - the call returns when indexing of the issues has completed or a failure occurs.
    pub fn reindex_issues(&self) -> ReindexIssuesRequest<'a> {
        ReindexIssuesRequest::new(self.client)
    }

    /// Returns information on the system reindexes. If a reindex is currently taking place then information about this reindex is returned. If there is no active index task, then returns information about the latest reindex task run, otherwise returns a 404 indicating that no reindex has taken place.
    pub fn get_reindex_progress(&self) -> GetReindexProgressRequest<'a> {
        GetReindexProgressRequest::new(self.client)
    }

    /// Executes any pending reindex requests. Execution is asynchronous - progress of the returned tasks can be monitored through other REST calls.
    pub fn process_requests(&self) -> ProcessRequestsRequest<'a> {
        ProcessRequestsRequest::new(self.client)
    }

    /// Retrieves the progress of multiple reindex requests. Only reindex requests that actually exist will be returned in the results.
    pub fn get_progress_bulk(&self) -> GetProgressBulkRequest<'a> {
        GetProgressBulkRequest::new(self.client)
    }

    /// Retrieves the progress of a single reindex request.
    pub fn get_reindex_request_progress(&self, request_id: i64) -> GetReindexRequestProgressRequest<'a> {
        GetReindexRequestProgressRequest::new(self.client, request_id)
    }
}

/// Lists available index snapshots absolute paths with timestamps
#[derive(Clone)]
pub struct ListIndexSnapshotRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ListIndexSnapshotRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/index-snapshot".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<IndexSnapshot>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Starts taking an index snapshot if no other snapshot creation process is in progress
#[derive(Clone)]
pub struct CreateIndexSnapshotRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CreateIndexSnapshotRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/index-snapshot".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IndexSnapshotPromise> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Checks if index snapshot creation is currently running
#[derive(Clone)]
pub struct IsIndexSnapshotRunningRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IsIndexSnapshotRunningRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/index-snapshot/isRunning".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IndexSnapshotStatus> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a summary of the index condition of the current node.
/// The returned data consists of:
/// - `nodeId` - Node identifier.
/// - `reportTime` - Time of this report creation.
/// - `issueIndex` - Summary of the issue index status.
/// - `replicationQueues` - Map of index replication queues, where keys represent nodes from which replication operations came from.
/// - `externalPlatformIndexReplay` - Map of external platform's index replay queues, where keys represents which node logged to index replay queue (journal)
///
/// `issueIndex` can contain:
///     - `indexReadable` - If `false` the endpoint failed to read data from the issue index (check Jira logs for detailed stack trace), otherwise `true`.
///     - `countInDatabase` - Count of issues found in the database.
///     - `countInIndex` - Count of issues found while querying the index.
///     - `lastUpdatedInDatabase` - Time of the last update of the issue found in the database.
///     - `lastUpdatedInIndex` - Time of the last update of the issue found while querying the index.
/// `replicationQueues`'s map values can contain:
///     - `lastConsumedOperation` - Last executed index replication operation by the current node from the sending node's queue.
///     - `lastConsumedOperation.id` - Identifier of the operation.
///     - `lastConsumedOperation.replicationTime` - Time when the operation was sent to other nodes.
///     - `lastOperationInQueue` - Last index replication operation in the sending node's queue.
///     - `lastOperationInQueue.id` - Identifier of the operation.
///     - `lastOperationInQueue.replicationTime` - Time when the operation was sent to other nodes.
///     - `queueSize` - Number of operations in the queue from the sending node to the current node.
/// `externalPlatformIndexReplay`'s map values can contain:
///     - `lastConsumedOperation` - Last executed external platform's index replay operation.
///     - `lastConsumedOperation.id` - Identifier of the operation.
///     - `lastConsumedOperation.journalWriteTime` - Time when the operation was written to the journal.
///     - `lastOperationInQueue` - Last external platform's index replay operation in the replay queue.
///     - `lastOperationInQueue.id` - Identifier of the operation.
///     - `lastOperationInQueue.journalWriteTime` - Time when the operation was written to the journal.
///     - `queueSize` - Number of operations in the queue awaiting synchronization with the external platform's index.
#[derive(Clone)]
pub struct GetIndexSummaryRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIndexSummaryRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/index/summary".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IndexSummary> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns information on the system reindexes. If a reindex is currently taking place then information about this reindex is returned. If there is no active index task, then returns information about the latest reindex task run, otherwise returns a 404 indicating that no reindex has taken place.
#[derive(Clone)]
pub struct GetReindexInfoRequest<'a> {
    client: &'a crate::core::Client,
    task_id: Option<i64>,
}

impl<'a> GetReindexInfoRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, task_id: None }
    }

    /// The id of an indexing task you wish to obtain details on. If omitted, then defaults to the standard behaviour and returns information on the active reindex task, or the last task to run if no reindex is taking place.
    #[must_use]
    pub fn task_id(mut self, value: i64) -> Self {
        self.task_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/reindex".to_owned());

        if let Some(value) = &self.task_id {
            config.query.push(("taskId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Reindex> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Kicks off a reindex. Need Admin permissions to perform this reindex.
#[derive(Clone)]
pub struct ReindexRequest2<'a> {
    client: &'a crate::core::Client,
    index_change_history: Option<bool>,
    r#type: Option<String>,
    index_worklogs: Option<bool>,
    index_comments: Option<bool>,
}

impl<'a> ReindexRequest2<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, index_change_history: None, r#type: None, index_worklogs: None, index_comments: None }
    }

    /// Indicates that changeHistory should also be reindexed. Not relevant for foreground reindex, where changeHistory is always reindexed.
    #[must_use]
    pub fn index_change_history(mut self, value: bool) -> Self {
        self.index_change_history = Some(value);

        self
    }

    /// Case insensitive String indicating type of reindex. If omitted, then defaults to BACKGROUND_PREFERRED. Not relevant for Search Platform that only supports BACKGROUND reindexing e.g. OpenSearch.
    #[must_use]
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());

        self
    }

    /// Indicates that worklogs should also be reindexed. Not relevant for foreground reindex, where worklogs are always reindexed.
    #[must_use]
    pub fn index_worklogs(mut self, value: bool) -> Self {
        self.index_worklogs = Some(value);

        self
    }

    /// Indicates that comments should also be reindexed. Not relevant for foreground reindex, where comments are always reindexed.
    #[must_use]
    pub fn index_comments(mut self, value: bool) -> Self {
        self.index_comments = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/reindex".to_owned());

        if let Some(value) = &self.index_change_history {
            config.query.push(("indexChangeHistory".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.r#type {
            config.query.push(("type".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.index_worklogs {
            config.query.push(("indexWorklogs".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.index_comments {
            config.query.push(("indexComments".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Reindex> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Reindexes one or more individual issues. Indexing is performed synchronously - the call returns when indexing of the issues has completed or a failure occurs.
#[derive(Clone)]
pub struct ReindexIssuesRequest<'a> {
    client: &'a crate::core::Client,
    issue_id: Option<Vec<String>>,
    index_change_history: Option<bool>,
    index_worklogs: Option<bool>,
    index_comments: Option<bool>,
}

impl<'a> ReindexIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, issue_id: None, index_change_history: None, index_worklogs: None, index_comments: None }
    }

    /// The IDs or keys of one or more issues to reindex.
    #[must_use]
    pub fn issue_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.issue_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Indicates that changeHistory should also be reindexed.
    #[must_use]
    pub fn index_change_history(mut self, value: bool) -> Self {
        self.index_change_history = Some(value);

        self
    }

    /// Indicates that worklogs should also be reindexed.
    #[must_use]
    pub fn index_worklogs(mut self, value: bool) -> Self {
        self.index_worklogs = Some(value);

        self
    }

    /// Indicates that comments should also be reindexed.
    #[must_use]
    pub fn index_comments(mut self, value: bool) -> Self {
        self.index_comments = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/reindex/issue".to_owned());

        if let Some(value) = &self.issue_id {
            config.query.push(("issueId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.index_change_history {
            config.query.push(("indexChangeHistory".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.index_worklogs {
            config.query.push(("indexWorklogs".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.index_comments {
            config.query.push(("indexComments".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Reindex> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns information on the system reindexes. If a reindex is currently taking place then information about this reindex is returned. If there is no active index task, then returns information about the latest reindex task run, otherwise returns a 404 indicating that no reindex has taken place.
#[derive(Clone)]
pub struct GetReindexProgressRequest<'a> {
    client: &'a crate::core::Client,
    task_id: Option<i64>,
}

impl<'a> GetReindexProgressRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, task_id: None }
    }

    /// The id of an indexing task you wish to obtain details on. If omitted, then defaults to the standard behaviour and returns information on the active reindex task, or the last task to run if no reindex is taking place.
    #[must_use]
    pub fn task_id(mut self, value: i64) -> Self {
        self.task_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/reindex/progress".to_owned());

        if let Some(value) = &self.task_id {
            config.query.push(("taskId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Reindex> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Executes any pending reindex requests. Execution is asynchronous - progress of the returned tasks can be monitored through other REST calls.
#[derive(Clone)]
pub struct ProcessRequestsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProcessRequestsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/reindex/request".to_owned());

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

/// Retrieves the progress of multiple reindex requests. Only reindex requests that actually exist will be returned in the results.
#[derive(Clone)]
pub struct GetProgressBulkRequest<'a> {
    client: &'a crate::core::Client,
    request_id: Option<Vec<i64>>,
}

impl<'a> GetProgressBulkRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, request_id: None }
    }

    /// The reindex request IDs.
    #[must_use]
    pub fn request_id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.request_id = Some(value.into_iter().collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/reindex/request/bulk".to_owned());

        if let Some(value) = &self.request_id {
            config.query.push(("requestId".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ReindexRequest>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Retrieves the progress of a single reindex request.
#[derive(Clone)]
pub struct GetReindexRequestProgressRequest<'a> {
    client: &'a crate::core::Client,
    request_id: i64,
}

impl<'a> GetReindexRequestProgressRequest<'a> {
    fn new(client: &'a crate::core::Client, request_id: i64) -> Self {
        Self { client, request_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/reindex/request/{}", self.request_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ReindexRequest> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
