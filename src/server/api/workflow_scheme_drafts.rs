// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowSchemeDrafts operations.
pub struct WorkflowSchemeDraftsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowSchemeDraftsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Create a draft for the passed scheme. The draft will be a copy of the state of the parent.
    pub fn create_workflow_scheme_draft_from_parent(&self, id: i64) -> CreateWorkflowSchemeDraftFromParentRequest<'a> {
        CreateWorkflowSchemeDraftFromParentRequest::new(self.client, id)
    }

    /// Returns the requested draft workflow scheme to the caller.
    pub fn get_workflow_scheme_draft(&self, id: i64) -> GetWorkflowSchemeDraftRequest<'a> {
        GetWorkflowSchemeDraftRequest::new(self.client, id)
    }

    /// Update a draft workflow scheme. The draft will created if necessary. The body of the request is a representation of the workflow scheme. Values not passed are assumed to indicate no change for that field.
    pub fn update_workflow_scheme_draft(&self, id: i64, body: WorkflowScheme) -> UpdateWorkflowSchemeDraftRequest<'a> {
        UpdateWorkflowSchemeDraftRequest::new(self.client, id, body)
    }

    /// Delete the passed draft workflow scheme.
    pub fn delete_workflow_scheme_draft(&self, id: i64) -> DeleteWorkflowSchemeDraftRequest<'a> {
        DeleteWorkflowSchemeDraftRequest::new(self.client, id)
    }

    /// Return the default workflow from the passed draft workflow scheme to the caller.
    pub fn get_draft_default_workflow(&self, id: i64) -> GetDraftDefaultWorkflowRequest<'a> {
        GetDraftDefaultWorkflowRequest::new(self.client, id)
    }

    /// Set the default workflow for the passed draft workflow scheme.
    pub fn update_draft_default_workflow(
        &self,
        id: i64,
        default_workflow: DefaultModel,
    ) -> UpdateDraftDefaultWorkflowRequest<'a> {
        UpdateDraftDefaultWorkflowRequest::new(self.client, id, default_workflow)
    }

    /// Remove the default workflow from the passed draft workflow scheme.
    pub fn delete_draft_default_workflow(&self, id: i64) -> DeleteDraftDefaultWorkflowRequest<'a> {
        DeleteDraftDefaultWorkflowRequest::new(self.client, id)
    }

    /// Returns the issue type mapping for the passed draft workflow scheme.
    pub fn get_workflow_scheme_draft_issue_type(
        &self,
        id: i64,
        issue_type: impl Into<String>,
    ) -> GetWorkflowSchemeDraftIssueTypeRequest<'a> {
        GetWorkflowSchemeDraftIssueTypeRequest::new(self.client, id, issue_type)
    }

    /// Set the issue type mapping for the passed draft scheme. The passed representation can have its updateDraftIfNeeded flag set to true to indicate that
    /// the draft should be created/updated when the actual scheme cannot be edited.
    pub fn set_workflow_scheme_draft_issue_type(
        &self,
        id: i64,
        issue_type: impl Into<String>,
        body: IssueTypeWorkflowMapping,
    ) -> SetWorkflowSchemeDraftIssueTypeRequest<'a> {
        SetWorkflowSchemeDraftIssueTypeRequest::new(self.client, id, issue_type, body)
    }

    /// Remove the specified issue type mapping from the draft scheme.
    pub fn delete_workflow_scheme_draft_issue_type(
        &self,
        id: i64,
        issue_type: impl Into<String>,
    ) -> DeleteWorkflowSchemeDraftIssueTypeRequest<'a> {
        DeleteWorkflowSchemeDraftIssueTypeRequest::new(self.client, id, issue_type)
    }

    /// Returns the draft workflow mappings or requested mapping to the caller.
    pub fn get_draft_workflow(&self, id: i64) -> GetDraftWorkflowRequest<'a> {
        GetDraftWorkflowRequest::new(self.client, id)
    }

    /// Update the draft scheme to include the passed mapping. The body is a representation of the workflow mapping. Values not passed are assumed to indicate no change for that field.
    pub fn update_draft_workflow_mapping(
        &self,
        id: i64,
        workflow_mapping: WorkflowMapping,
    ) -> UpdateDraftWorkflowMappingRequest<'a> {
        UpdateDraftWorkflowMappingRequest::new(self.client, id, workflow_mapping)
    }

    /// Delete the passed workflow from the draft workflow scheme.
    pub fn delete_draft_workflow_mapping(&self, id: i64) -> DeleteDraftWorkflowMappingRequest<'a> {
        DeleteDraftWorkflowMappingRequest::new(self.client, id)
    }
}

/// Create a draft for the passed scheme. The draft will be a copy of the state of the parent.
#[derive(Clone)]
pub struct CreateWorkflowSchemeDraftFromParentRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> CreateWorkflowSchemeDraftFromParentRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/workflowscheme/{}/createdraft", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the requested draft workflow scheme to the caller.
#[derive(Clone)]
pub struct GetWorkflowSchemeDraftRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetWorkflowSchemeDraftRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/workflowscheme/{}/draft", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update a draft workflow scheme. The draft will created if necessary. The body of the request is a representation of the workflow scheme. Values not passed are assumed to indicate no change for that field.
#[derive(Clone)]
pub struct UpdateWorkflowSchemeDraftRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    body: WorkflowScheme,
}

impl<'a> UpdateWorkflowSchemeDraftRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, body: WorkflowScheme) -> Self {
        Self { client, id, body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/workflowscheme/{}/draft", self.id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the passed draft workflow scheme.
#[derive(Clone)]
pub struct DeleteWorkflowSchemeDraftRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> DeleteWorkflowSchemeDraftRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/workflowscheme/{}/draft", self.id),
        );

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

/// Return the default workflow from the passed draft workflow scheme to the caller.
#[derive(Clone)]
pub struct GetDraftDefaultWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetDraftDefaultWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/workflowscheme/{}/draft/default", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Set the default workflow for the passed draft workflow scheme.
#[derive(Clone)]
pub struct UpdateDraftDefaultWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    default_workflow: DefaultModel,
}

impl<'a> UpdateDraftDefaultWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, default_workflow: DefaultModel) -> Self {
        Self { client, id, default_workflow }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/workflowscheme/{}/draft/default", self.id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.default_workflow)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Remove the default workflow from the passed draft workflow scheme.
#[derive(Clone)]
pub struct DeleteDraftDefaultWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> DeleteDraftDefaultWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/workflowscheme/{}/draft/default", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the issue type mapping for the passed draft workflow scheme.
#[derive(Clone)]
pub struct GetWorkflowSchemeDraftIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    issue_type: String,
}

impl<'a> GetWorkflowSchemeDraftIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, issue_type: impl Into<String>) -> Self {
        Self { client, id, issue_type: issue_type.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/workflowscheme/{}/draft/issuetype/{}",
                self.id,
                crate::core::encode_path_segment(&self.issue_type)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeWorkflowMapping> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Set the issue type mapping for the passed draft scheme. The passed representation can have its updateDraftIfNeeded flag set to true to indicate that
/// the draft should be created/updated when the actual scheme cannot be edited.
#[derive(Clone)]
pub struct SetWorkflowSchemeDraftIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    issue_type: String,
    body: IssueTypeWorkflowMapping,
}

impl<'a> SetWorkflowSchemeDraftIssueTypeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        id: i64,
        issue_type: impl Into<String>,
        body: IssueTypeWorkflowMapping,
    ) -> Self {
        Self { client, id, issue_type: issue_type.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/workflowscheme/{}/draft/issuetype/{}",
                self.id,
                crate::core::encode_path_segment(&self.issue_type)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Remove the specified issue type mapping from the draft scheme.
#[derive(Clone)]
pub struct DeleteWorkflowSchemeDraftIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    issue_type: String,
}

impl<'a> DeleteWorkflowSchemeDraftIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, issue_type: impl Into<String>) -> Self {
        Self { client, id, issue_type: issue_type.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/workflowscheme/{}/draft/issuetype/{}",
                self.id,
                crate::core::encode_path_segment(&self.issue_type)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the draft workflow mappings or requested mapping to the caller.
#[derive(Clone)]
pub struct GetDraftWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    workflow_name: Option<String>,
    id: i64,
}

impl<'a> GetDraftWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, workflow_name: None }
    }

    /// The workflow mapping to return. Null can be passed to return all mappings. Must be a valid workflow name.
    #[must_use]
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/workflowscheme/{}/draft/workflow", self.id),
        );

        if let Some(value) = &self.workflow_name {
            config.query.push(("workflowName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update the draft scheme to include the passed mapping. The body is a representation of the workflow mapping. Values not passed are assumed to indicate no change for that field.
#[derive(Clone)]
pub struct UpdateDraftWorkflowMappingRequest<'a> {
    client: &'a crate::core::Client,
    workflow_name: Option<String>,
    id: i64,
    workflow_mapping: WorkflowMapping,
}

impl<'a> UpdateDraftWorkflowMappingRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, workflow_mapping: WorkflowMapping) -> Self {
        Self { client, id, workflow_mapping, workflow_name: None }
    }

    /// The name of the workflow mapping to update.
    #[must_use]
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/workflowscheme/{}/draft/workflow", self.id),
        );

        if let Some(value) = &self.workflow_name {
            config.query.push(("workflowName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.workflow_mapping)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the passed workflow from the draft workflow scheme.
#[derive(Clone)]
pub struct DeleteDraftWorkflowMappingRequest<'a> {
    client: &'a crate::core::Client,
    workflow_name: Option<String>,
    id: i64,
}

impl<'a> DeleteDraftWorkflowMappingRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, workflow_name: None }
    }

    /// The name of the workflow to delete.
    #[must_use]
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/workflowscheme/{}/draft/workflow", self.id),
        );

        if let Some(value) = &self.workflow_name {
            config.query.push(("workflowName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
