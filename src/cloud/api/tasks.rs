// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Tasks operations.
pub struct TasksService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> TasksService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the status of a [long-running asynchronous task](#async).
    ///
    /// When a task has finished, this operation returns the JSON blob applicable to the task. See the documentation of the operation that created the task for details. Task details are not permanently retained. As of September 2019, details are retained for 14 days although this period may change without notice.
    ///
    /// **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.
    ///
    ///  *  `read:jira-work`
    ///
    /// **[Permissions](#permissions) required:** either of:
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    ///  *  Creator of the task.
    pub fn get_task(&self, task_id: impl Into<String>) -> GetTaskRequest<'a> {
        GetTaskRequest::new(self.client, task_id)
    }
}

/// Returns the status of a [long-running asynchronous task](#async).
///
/// When a task has finished, this operation returns the JSON blob applicable to the task. See the documentation of the operation that created the task for details. Task details are not permanently retained. As of September 2019, details are retained for 14 days although this period may change without notice.
///
/// **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.
///
///  *  `read:jira-work`
///
/// **[Permissions](#permissions) required:** either of:
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
///  *  Creator of the task.
pub struct GetTaskRequest<'a> {
    client: &'a crate::core::Client,
    task_id: String,
}

impl<'a> GetTaskRequest<'a> {
    fn new(client: &'a crate::core::Client, task_id: impl Into<String>) -> Self {
        Self { client, task_id: task_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/3/task/{}", self.task_id));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TaskProgressObject> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
