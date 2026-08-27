// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Workflows operations.
pub struct WorkflowsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all workflows. The “lastModifiedDate” is returned in Jira Complete Date/Time Format (dd/MMM/yy h:mm by default), but can also be returned as a relative date.
    pub fn get_all_workflows(&self) -> GetAllWorkflowsRequest<'a> {
        GetAllWorkflowsRequest::new(self.client)
    }
}

/// Returns all workflows. The “lastModifiedDate” is returned in Jira Complete Date/Time Format (dd/MMM/yy h:mm by default), but can also be returned as a relative date.
pub struct GetAllWorkflowsRequest<'a> {
    client: &'a crate::core::Client,
    workflow_name: Option<String>,
}

impl<'a> GetAllWorkflowsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, workflow_name: None }
    }

    /// an optional String containing workflow name. If not passed then all workflows are returned
    #[must_use]
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/workflow".to_owned());

        if let Some(value) = &self.workflow_name {
            config.query.push(("workflowName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Workflow>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
