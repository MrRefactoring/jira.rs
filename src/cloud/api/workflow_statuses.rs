// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowStatuses operations.
pub struct WorkflowStatusesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowStatusesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all statuses associated with active workflows.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// [Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required: *Browse projects* [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) for the project.
    pub fn get_statuses(&self) -> GetStatusesRequest<'a> {
        GetStatusesRequest::new(self.client)
    }

    /// Returns a status. The status must be associated with an active workflow to be returned.
    ///
    /// If a name is used on more than one status, only the status found first is returned. Therefore, identifying the status by its ID may be preferable.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// [Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required: *Browse projects* [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) for the project.
    pub fn get_status(&self, id_or_name: impl Into<String>) -> GetStatusRequest<'a> {
        GetStatusRequest::new(self.client, id_or_name)
    }
}

/// Returns a list of all statuses associated with active workflows.
///
/// This operation can be accessed anonymously.
///
/// [Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required: *Browse projects* [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) for the project.
pub struct GetStatusesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetStatusesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/status".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<StatusDetails>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a status. The status must be associated with an active workflow to be returned.
///
/// If a name is used on more than one status, only the status found first is returned. Therefore, identifying the status by its ID may be preferable.
///
/// This operation can be accessed anonymously.
///
/// [Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required: *Browse projects* [project permission](https://support.atlassian.com/jira-cloud-administration/docs/manage-project-permissions/) for the project.
pub struct GetStatusRequest<'a> {
    client: &'a crate::core::Client,
    id_or_name: String,
}

impl<'a> GetStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, id_or_name: impl Into<String>) -> Self {
        Self { client, id_or_name: id_or_name.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/status/{}", crate::core::encode_path_segment(&self.id_or_name)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusDetails> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
