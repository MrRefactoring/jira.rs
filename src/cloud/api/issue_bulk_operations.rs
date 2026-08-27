// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueBulkOperations operations.
pub struct IssueBulkOperationsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueBulkOperationsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Use this API to submit a bulk delete request. You can delete up to 1,000 issues in a single operation.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Delete [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Delete-issues/) in all projects that contain the selected issues.
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn submit_bulk_delete(&self, issue_bulk_delete_payload: IssueBulkDeletePayload) -> SubmitBulkDeleteRequest<'a> {
        SubmitBulkDeleteRequest::new(self.client, issue_bulk_delete_payload)
    }

    /// Use this API to get a list of fields visible to the user to perform bulk edit operations. You can pass single or multiple issues in the query to get eligible editable fields. This API uses pagination to return responses, delivering 50 fields at a time.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  Depending on the field, any field-specific permissions required to edit it.
    pub fn get_bulk_editable_fields(&self, issue_ids_or_keys: impl Into<String>) -> GetBulkEditableFieldsRequest<'a> {
        GetBulkEditableFieldsRequest::new(self.client, issue_ids_or_keys)
    }

    /// Use this API to submit a bulk edit request and simultaneously edit multiple issues. There are limits applied to the number of issues and fields that can be edited. A single request can accommodate a maximum of 1000 issues (including subtasks) and 200 fields.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  Edit [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn submit_bulk_edit(&self, issue_bulk_edit_payload: IssueBulkEditPayload) -> SubmitBulkEditRequest<'a> {
        SubmitBulkEditRequest::new(self.client, issue_bulk_edit_payload)
    }

    /// Use this API to submit a bulk issue move request. You can move multiple issues from multiple projects in a single request, but they must all be moved to a single project, issue type, and parent. You can't move more than 1000 issues (including subtasks) at once.
    ///
    /// #### Scenarios: ####
    ///
    /// This is an early version of the API and it doesn't have full feature parity with the Bulk Move UI experience.
    ///
    ///  *  Moving issue of type A to issue of type B in the same project or a different project: `SUPPORTED`
    ///  *  Moving multiple issues of type A in one or more projects to multiple issues of type B in one of the source projects or a different project: `SUPPORTED`
    ///  *  Moving issues of multiple issue types in one or more projects to issues of a single issue type in one of the source project or a different project: **`SUPPORTED`**
    ///     E.g. Moving issues of story and task issue types in project 1 and project 2 to issues of task issue type in project 3
    ///  *  Moving a standard parent issue of type A with its multiple subtask issue types in one project to standard issue of type B and multiple subtask issue types in the same project or a different project: `SUPPORTED`
    ///  *  Moving standard issues with their subtasks to a parent issue in the same project or a different project without losing their relation: `SUPPORTED`
    ///  *  Moving an epic issue with its child issues to a different project without losing their relation: `SUPPORTED`
    ///     This usecase is **supported using multiple requests**. Move the epic in one request and then move the children in a separate request with target parent set to the epic issue id
    ///
    ///     (Alternatively, move them individually and stitch the relationship back with the Bulk Edit API)
    ///
    /// #### Limits applied to bulk issue moves: ####
    ///
    /// When using the bulk move, keep in mind that there are limits on the number of issues and fields you can include.
    ///
    ///  *  You can move up to 1,000 issues in a single operation, including any subtasks.
    ///  *  The total combined number of fields across all issues must not exceed 1,500,000. For example, if each issue includes 15,000 fields, then the maximum number of issues that can be moved is 100.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Move [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in source projects.
    ///  *  Create [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects.
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects, if moving subtasks only.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn submit_bulk_move(&self, issue_bulk_move_payload: IssueBulkMovePayload) -> SubmitBulkMoveRequest<'a> {
        SubmitBulkMoveRequest::new(self.client, issue_bulk_move_payload)
    }

    /// Use this API to retrieve a list of transitions available for the specified issues that can be used or bulk transition operations. You can submit either single or multiple issues in the query to obtain the available transitions.
    ///
    /// The response will provide the available transitions for issues, organized by their respective workflows. **Only the transitions that are common among the issues within that workflow and do not involve any additional field updates will be included.** For bulk transitions that require additional field updates, please utilise the Jira Cloud UI.
    ///
    /// You can request available transitions for up to 1,000 issues in a single operation. This API uses pagination to return responses, delivering 50 workflows at a time.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Transition [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Transition-issues/) in all projects that contain the selected issues.
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn get_available_transitions(
        &self,
        issue_ids_or_keys: impl Into<String>,
    ) -> GetAvailableTransitionsRequest<'a> {
        GetAvailableTransitionsRequest::new(self.client, issue_ids_or_keys)
    }

    /// Use this API to submit a bulk issue status transition request. You can transition multiple issues, alongside with their valid transition Ids. You can transition up to 1,000 issues in a single operation.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Transition [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Transition-issues/) in all projects that contain the selected issues.
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn submit_bulk_transition(
        &self,
        issue_bulk_transition_payload: IssueBulkTransitionPayload,
    ) -> SubmitBulkTransitionRequest<'a> {
        SubmitBulkTransitionRequest::new(self.client, issue_bulk_transition_payload)
    }

    /// Use this API to submit a bulk unwatch request. You can unwatch up to 1,000 issues in a single operation.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn submit_bulk_unwatch(
        &self,
        issue_bulk_watch_or_unwatch_payload: IssueBulkWatchOrUnwatchPayload,
    ) -> SubmitBulkUnwatchRequest<'a> {
        SubmitBulkUnwatchRequest::new(self.client, issue_bulk_watch_or_unwatch_payload)
    }

    /// Use this API to submit a bulk watch request. You can watch up to 1,000 issues in a single operation.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn submit_bulk_watch(
        &self,
        issue_bulk_watch_or_unwatch_payload: IssueBulkWatchOrUnwatchPayload,
    ) -> SubmitBulkWatchRequest<'a> {
        SubmitBulkWatchRequest::new(self.client, issue_bulk_watch_or_unwatch_payload)
    }

    /// Use this to get the progress state for the specified bulk operation `taskId`.
    ///
    /// **[Permissions](#permissions) required:**
    ///
    ///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
    ///
    /// If the task is running, this resource will return:
    ///
    ///    {"taskId":"10779","status":"RUNNING","progressPercent":65,"submittedBy":{"accountId":"5b10a2844c20165700ede21g"},"created":1690180055963,"started":1690180056206,"updated":169018005829}
    ///
    /// If the task has completed, then this resource will return:
    ///
    ///    {"processedAccessibleIssues":\[10001,10002\],"created":1709189449954,"progressPercent":100,"started":1709189450154,"status":"COMPLETE","submittedBy":{"accountId":"5b10a2844c20165700ede21g"},"invalidOrInaccessibleIssueCount":0,"taskId":"10000","totalIssueCount":2,"updated":1709189450354}
    ///
    /// **Note:** You can view task progress for up to 14 days from creation.
    pub fn get_bulk_operation_progress(&self, task_id: impl Into<String>) -> GetBulkOperationProgressRequest<'a> {
        GetBulkOperationProgressRequest::new(self.client, task_id)
    }
}

/// Use this API to submit a bulk delete request. You can delete up to 1,000 issues in a single operation.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Delete [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Delete-issues/) in all projects that contain the selected issues.
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct SubmitBulkDeleteRequest<'a> {
    client: &'a crate::core::Client,
    issue_bulk_delete_payload: IssueBulkDeletePayload,
}

impl<'a> SubmitBulkDeleteRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_bulk_delete_payload: IssueBulkDeletePayload) -> Self {
        Self { client, issue_bulk_delete_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/bulk/issues/delete".to_owned());

        let body = match serde_json::to_value(&self.issue_bulk_delete_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmittedBulkOperation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to get a list of fields visible to the user to perform bulk edit operations. You can pass single or multiple issues in the query to get eligible editable fields. This API uses pagination to return responses, delivering 50 fields at a time.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  Depending on the field, any field-specific permissions required to edit it.
pub struct GetBulkEditableFieldsRequest<'a> {
    client: &'a crate::core::Client,
    issue_ids_or_keys: String,
    search_text: Option<String>,
    ending_before: Option<String>,
    starting_after: Option<String>,
}

impl<'a> GetBulkEditableFieldsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_ids_or_keys: impl Into<String>) -> Self {
        Self {
            client,
            issue_ids_or_keys: issue_ids_or_keys.into(),
            search_text: None,
            ending_before: None,
            starting_after: None,
        }
    }

    /// (Optional)The text to search for in the editable fields.
    #[must_use]
    pub fn search_text(mut self, value: impl Into<String>) -> Self {
        self.search_text = Some(value.into());

        self
    }

    /// (Optional)The end cursor for use in pagination.
    #[must_use]
    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());

        self
    }

    /// (Optional)The start cursor for use in pagination.
    #[must_use]
    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/bulk/issues/fields".to_owned());

        config
            .query
            .push(("issueIdsOrKeys".to_owned(), crate::core::QueryValue::Scalar(self.issue_ids_or_keys.clone())));

        if let Some(value) = &self.search_text {
            config.query.push(("searchText".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.ending_before {
            config.query.push(("endingBefore".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.starting_after {
            config.query.push(("startingAfter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkEditGetFields> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to submit a bulk edit request and simultaneously edit multiple issues. There are limits applied to the number of issues and fields that can be edited. A single request can accommodate a maximum of 1000 issues (including subtasks) and 200 fields.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  Edit [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct SubmitBulkEditRequest<'a> {
    client: &'a crate::core::Client,
    issue_bulk_edit_payload: IssueBulkEditPayload,
}

impl<'a> SubmitBulkEditRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_bulk_edit_payload: IssueBulkEditPayload) -> Self {
        Self { client, issue_bulk_edit_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/bulk/issues/fields".to_owned());

        let body = match serde_json::to_value(&self.issue_bulk_edit_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmittedBulkOperation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to submit a bulk issue move request. You can move multiple issues from multiple projects in a single request, but they must all be moved to a single project, issue type, and parent. You can't move more than 1000 issues (including subtasks) at once.
///
/// #### Scenarios: ####
///
/// This is an early version of the API and it doesn't have full feature parity with the Bulk Move UI experience.
///
///  *  Moving issue of type A to issue of type B in the same project or a different project: `SUPPORTED`
///  *  Moving multiple issues of type A in one or more projects to multiple issues of type B in one of the source projects or a different project: `SUPPORTED`
///  *  Moving issues of multiple issue types in one or more projects to issues of a single issue type in one of the source project or a different project: **`SUPPORTED`**
///     E.g. Moving issues of story and task issue types in project 1 and project 2 to issues of task issue type in project 3
///  *  Moving a standard parent issue of type A with its multiple subtask issue types in one project to standard issue of type B and multiple subtask issue types in the same project or a different project: `SUPPORTED`
///  *  Moving standard issues with their subtasks to a parent issue in the same project or a different project without losing their relation: `SUPPORTED`
///  *  Moving an epic issue with its child issues to a different project without losing their relation: `SUPPORTED`
///     This usecase is **supported using multiple requests**. Move the epic in one request and then move the children in a separate request with target parent set to the epic issue id
///
///     (Alternatively, move them individually and stitch the relationship back with the Bulk Edit API)
///
/// #### Limits applied to bulk issue moves: ####
///
/// When using the bulk move, keep in mind that there are limits on the number of issues and fields you can include.
///
///  *  You can move up to 1,000 issues in a single operation, including any subtasks.
///  *  The total combined number of fields across all issues must not exceed 1,500,000. For example, if each issue includes 15,000 fields, then the maximum number of issues that can be moved is 100.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Move [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in source projects.
///  *  Create [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects.
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in destination projects, if moving subtasks only.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct SubmitBulkMoveRequest<'a> {
    client: &'a crate::core::Client,
    issue_bulk_move_payload: IssueBulkMovePayload,
}

impl<'a> SubmitBulkMoveRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_bulk_move_payload: IssueBulkMovePayload) -> Self {
        Self { client, issue_bulk_move_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/bulk/issues/move".to_owned());

        let body = match serde_json::to_value(&self.issue_bulk_move_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmittedBulkOperation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to retrieve a list of transitions available for the specified issues that can be used or bulk transition operations. You can submit either single or multiple issues in the query to obtain the available transitions.
///
/// The response will provide the available transitions for issues, organized by their respective workflows. **Only the transitions that are common among the issues within that workflow and do not involve any additional field updates will be included.** For bulk transitions that require additional field updates, please utilise the Jira Cloud UI.
///
/// You can request available transitions for up to 1,000 issues in a single operation. This API uses pagination to return responses, delivering 50 workflows at a time.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Transition [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Transition-issues/) in all projects that contain the selected issues.
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct GetAvailableTransitionsRequest<'a> {
    client: &'a crate::core::Client,
    issue_ids_or_keys: String,
    ending_before: Option<String>,
    starting_after: Option<String>,
}

impl<'a> GetAvailableTransitionsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_ids_or_keys: impl Into<String>) -> Self {
        Self { client, issue_ids_or_keys: issue_ids_or_keys.into(), ending_before: None, starting_after: None }
    }

    /// (Optional)The end cursor for use in pagination.
    #[must_use]
    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());

        self
    }

    /// (Optional)The start cursor for use in pagination.
    #[must_use]
    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/bulk/issues/transition".to_owned());

        config
            .query
            .push(("issueIdsOrKeys".to_owned(), crate::core::QueryValue::Scalar(self.issue_ids_or_keys.clone())));

        if let Some(value) = &self.ending_before {
            config.query.push(("endingBefore".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.starting_after {
            config.query.push(("startingAfter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkTransitionGetAvailableTransitions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to submit a bulk issue status transition request. You can transition multiple issues, alongside with their valid transition Ids. You can transition up to 1,000 issues in a single operation.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Transition [issues permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Transition-issues/) in all projects that contain the selected issues.
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct SubmitBulkTransitionRequest<'a> {
    client: &'a crate::core::Client,
    issue_bulk_transition_payload: IssueBulkTransitionPayload,
}

impl<'a> SubmitBulkTransitionRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_bulk_transition_payload: IssueBulkTransitionPayload) -> Self {
        Self { client, issue_bulk_transition_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/bulk/issues/transition".to_owned());

        let body = match serde_json::to_value(&self.issue_bulk_transition_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmittedBulkOperation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to submit a bulk unwatch request. You can unwatch up to 1,000 issues in a single operation.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct SubmitBulkUnwatchRequest<'a> {
    client: &'a crate::core::Client,
    issue_bulk_watch_or_unwatch_payload: IssueBulkWatchOrUnwatchPayload,
}

impl<'a> SubmitBulkUnwatchRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_bulk_watch_or_unwatch_payload: IssueBulkWatchOrUnwatchPayload,
    ) -> Self {
        Self { client, issue_bulk_watch_or_unwatch_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/bulk/issues/unwatch".to_owned());

        let body = match serde_json::to_value(&self.issue_bulk_watch_or_unwatch_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmittedBulkOperation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this API to submit a bulk watch request. You can watch up to 1,000 issues in a single operation.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///  *  Browse [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) in all projects that contain the selected issues.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
pub struct SubmitBulkWatchRequest<'a> {
    client: &'a crate::core::Client,
    issue_bulk_watch_or_unwatch_payload: IssueBulkWatchOrUnwatchPayload,
}

impl<'a> SubmitBulkWatchRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_bulk_watch_or_unwatch_payload: IssueBulkWatchOrUnwatchPayload,
    ) -> Self {
        Self { client, issue_bulk_watch_or_unwatch_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/bulk/issues/watch".to_owned());

        let body = match serde_json::to_value(&self.issue_bulk_watch_or_unwatch_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SubmittedBulkOperation> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Use this to get the progress state for the specified bulk operation `taskId`.
///
/// **[Permissions](#permissions) required:**
///
///  *  Global bulk change [permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-global-permissions/).
///
/// If the task is running, this resource will return:
///
///    {"taskId":"10779","status":"RUNNING","progressPercent":65,"submittedBy":{"accountId":"5b10a2844c20165700ede21g"},"created":1690180055963,"started":1690180056206,"updated":169018005829}
///
/// If the task has completed, then this resource will return:
///
///    {"processedAccessibleIssues":\[10001,10002\],"created":1709189449954,"progressPercent":100,"started":1709189450154,"status":"COMPLETE","submittedBy":{"accountId":"5b10a2844c20165700ede21g"},"invalidOrInaccessibleIssueCount":0,"taskId":"10000","totalIssueCount":2,"updated":1709189450354}
///
/// **Note:** You can view task progress for up to 14 days from creation.
pub struct GetBulkOperationProgressRequest<'a> {
    client: &'a crate::core::Client,
    task_id: String,
}

impl<'a> GetBulkOperationProgressRequest<'a> {
    fn new(client: &'a crate::core::Client, task_id: impl Into<String>) -> Self {
        Self { client, task_id: task_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/bulk/queue/{}", self.task_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkOperationProgress> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
