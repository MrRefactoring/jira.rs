// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowSchemeProjectAssociations operations.
pub struct WorkflowSchemeProjectAssociationsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowSchemeProjectAssociationsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of the workflow schemes associated with a list of projects. Each returned workflow scheme includes a list of the requested projects associated with it. Any team-managed or non-existent projects in the request are ignored and no errors are returned.
    ///
    /// If the project is associated with the `Default Workflow Scheme` no ID is returned. This is because the way the `Default Workflow Scheme` is stored means it has no ID.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_workflow_scheme_project_associations(
        &self,
        project_id: impl IntoIterator<Item = i64>,
    ) -> GetWorkflowSchemeProjectAssociationsRequest<'a> {
        GetWorkflowSchemeProjectAssociationsRequest::new(self.client, project_id)
    }

    /// Assigns a workflow scheme to a project. This operation is performed only when there are no issues in the project.
    ///
    /// Workflow schemes can only be assigned to classic projects.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn assign_scheme_to_project(
        &self,
        workflow_scheme_project_association: WorkflowSchemeProjectAssociation,
    ) -> AssignSchemeToProjectRequest<'a> {
        AssignSchemeToProjectRequest::new(self.client, workflow_scheme_project_association)
    }
}

/// Returns a list of the workflow schemes associated with a list of projects. Each returned workflow scheme includes a list of the requested projects associated with it. Any team-managed or non-existent projects in the request are ignored and no errors are returned.
///
/// If the project is associated with the `Default Workflow Scheme` no ID is returned. This is because the way the `Default Workflow Scheme` is stored means it has no ID.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetWorkflowSchemeProjectAssociationsRequest<'a> {
    client: &'a crate::core::Client,
    project_id: Vec<i64>,
}

impl<'a> GetWorkflowSchemeProjectAssociationsRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id: impl IntoIterator<Item = i64>) -> Self {
        Self { client, project_id: project_id.into_iter().collect() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/workflowscheme/project".to_owned());

        config.query.push(("projectId".to_owned(), crate::core::QueryValue::from_serializable(&self.project_id)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ContainerOfWorkflowSchemeAssociations> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Assigns a workflow scheme to a project. This operation is performed only when there are no issues in the project.
///
/// Workflow schemes can only be assigned to classic projects.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct AssignSchemeToProjectRequest<'a> {
    client: &'a crate::core::Client,
    workflow_scheme_project_association: WorkflowSchemeProjectAssociation,
}

impl<'a> AssignSchemeToProjectRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        workflow_scheme_project_association: WorkflowSchemeProjectAssociation,
    ) -> Self {
        Self { client, workflow_scheme_project_association }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/workflowscheme/project".to_owned());

        let body = match serde_json::to_value(&self.workflow_scheme_project_association)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

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
