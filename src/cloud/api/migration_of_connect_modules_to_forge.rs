// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The MigrationOfConnectModulesToForge operations.
pub struct MigrationOfConnectModulesToForgeService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> MigrationOfConnectModulesToForgeService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the details of a Connect issue field's migration to Forge.
    ///
    /// When migrating a Connect app to Forge, [Issue Field](https://developer.atlassian.com/cloud/jira/software/modules/issue-field/) modules
    /// must be converted to [Custom field](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/). When the
    /// Forge version of the app is installed, Forge creates a
    /// [background task](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-tasks/#api-group-tasks) to track the
    /// migration of field data across. This endpoint returns the status and other details of that background task.
    ///
    /// For more details, see
    /// [Jira modules > Jira Custom Fields](https://developer.atlassian.com/platform/adopting-forge-from-connect/migrate-jira-custom-fields/).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Connect and Forge apps can make this request.
    pub fn fetch_migration_task(
        &self,
        connect_key: impl Into<String>,
        jira_issue_fields_key: impl Into<String>,
    ) -> FetchMigrationTaskRequest<'a> {
        FetchMigrationTaskRequest::new(self.client, connect_key, jira_issue_fields_key)
    }

    /// Submits a request to trigger migration of connect issue field to its Forge custom field counterpart.
    ///
    /// When migrating a Connect app to Forge, [Issue Field](https://developer.atlassian.com/cloud/jira/software/modules/issue-field/) modules
    /// must be converted to [Custom field](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/) modules.
    /// This endpoint triggers the background migration of field data. Use the GET endpoint to retrieve
    /// the status and progress of the task.
    ///
    /// For more details, see
    /// [Jira modules > Jira Custom Fields](https://developer.atlassian.com/platform/adopting-forge-from-connect/migrate-jira-custom-fields/).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Connect and Forge apps can make this request.
    pub fn submit_task(
        &self,
        connect_key: impl Into<String>,
        jira_issue_fields_key: impl Into<String>,
    ) -> SubmitTaskRequest<'a> {
        SubmitTaskRequest::new(self.client, connect_key, jira_issue_fields_key)
    }
}

/// Returns the details of a Connect issue field's migration to Forge.
///
/// When migrating a Connect app to Forge, [Issue Field](https://developer.atlassian.com/cloud/jira/software/modules/issue-field/) modules
/// must be converted to [Custom field](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/). When the
/// Forge version of the app is installed, Forge creates a
/// [background task](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-tasks/#api-group-tasks) to track the
/// migration of field data across. This endpoint returns the status and other details of that background task.
///
/// For more details, see
/// [Jira modules > Jira Custom Fields](https://developer.atlassian.com/platform/adopting-forge-from-connect/migrate-jira-custom-fields/).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Connect and Forge apps can make this request.
pub struct FetchMigrationTaskRequest<'a> {
    client: &'a crate::core::Client,
    connect_key: String,
    jira_issue_fields_key: String,
}

impl<'a> FetchMigrationTaskRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        connect_key: impl Into<String>,
        jira_issue_fields_key: impl Into<String>,
    ) -> Self {
        Self { client, connect_key: connect_key.into(), jira_issue_fields_key: jira_issue_fields_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/atlassian-connect/1/migration/{}/{}/task",
                crate::core::encode_path_segment(&self.connect_key),
                crate::core::encode_path_segment(&self.jira_issue_fields_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TaskProgress> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Submits a request to trigger migration of connect issue field to its Forge custom field counterpart.
///
/// When migrating a Connect app to Forge, [Issue Field](https://developer.atlassian.com/cloud/jira/software/modules/issue-field/) modules
/// must be converted to [Custom field](https://developer.atlassian.com/platform/forge/manifest-reference/modules/jira-custom-field/) modules.
/// This endpoint triggers the background migration of field data. Use the GET endpoint to retrieve
/// the status and progress of the task.
///
/// For more details, see
/// [Jira modules > Jira Custom Fields](https://developer.atlassian.com/platform/adopting-forge-from-connect/migrate-jira-custom-fields/).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Connect and Forge apps can make this request.
pub struct SubmitTaskRequest<'a> {
    client: &'a crate::core::Client,
    connect_key: String,
    jira_issue_fields_key: String,
    retrigger_completed_migration: Option<bool>,
}

impl<'a> SubmitTaskRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        connect_key: impl Into<String>,
        jira_issue_fields_key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            connect_key: connect_key.into(),
            jira_issue_fields_key: jira_issue_fields_key.into(),
            retrigger_completed_migration: None,
        }
    }

    /// Whether to retrigger the migration if it has already completed.
    #[must_use]
    pub fn retrigger_completed_migration(mut self, value: bool) -> Self {
        self.retrigger_completed_migration = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/atlassian-connect/1/migration/{}/{}/task",
                crate::core::encode_path_segment(&self.connect_key),
                crate::core::encode_path_segment(&self.jira_issue_fields_key)
            ),
        );

        if let Some(value) = &self.retrigger_completed_migration {
            config
                .query
                .push(("retriggerCompletedMigration".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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
