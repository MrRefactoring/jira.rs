// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowSchemes operations.
pub struct WorkflowSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Create a new workflow scheme. The body contains a representation of the new scheme. Values not passed are assumed to be set to their defaults.
    pub fn create_workflow_scheme(&self, workflow_scheme: WorkflowScheme) -> CreateWorkflowSchemeRequest<'a> {
        CreateWorkflowSchemeRequest::new(self.client, workflow_scheme)
    }

    /// Returns the requested workflow scheme to the caller.
    pub fn get_workflow_scheme(&self, id: i64) -> GetWorkflowSchemeRequest<'a> {
        GetWorkflowSchemeRequest::new(self.client, id)
    }

    /// Update the passed workflow scheme. The body of the request is a representation of the workflow scheme. Values not passed are assumed to indicate no change for that field.
    /// The passed representation can have its updateDraftIfNeeded flag set to true to indicate that the draft
    /// should be created and/or updated when the actual scheme cannot be edited (e.g. when the scheme is being used by
    /// a project). Values not appearing the body will not be touched.
    pub fn update_workflow_scheme(&self, id: i64, body: WorkflowScheme) -> UpdateWorkflowSchemeRequest<'a> {
        UpdateWorkflowSchemeRequest::new(self.client, id, body)
    }

    /// Delete the passed workflow scheme.
    pub fn delete_workflow_scheme(&self, id: i64) -> DeleteWorkflowSchemeRequest<'a> {
        DeleteWorkflowSchemeRequest::new(self.client, id)
    }

    /// Return the default workflow from the passed workflow scheme.
    pub fn get_default_workflow(&self, id: i64) -> GetDefaultWorkflowRequest<'a> {
        GetDefaultWorkflowRequest::new(self.client, id)
    }

    /// Set the default workflow for the passed workflow scheme. The passed representation can have its
    /// updateDraftIfNeeded flag set to true to indicate that the draft should be created/updated when the actual scheme
    /// cannot be edited.
    pub fn update_default_workflow(&self, id: i64, default_workflow: DefaultModel) -> UpdateDefaultWorkflowRequest<'a> {
        UpdateDefaultWorkflowRequest::new(self.client, id, default_workflow)
    }

    /// Remove the default workflow from the passed workflow scheme.
    pub fn delete_default_workflow(&self, id: i64) -> DeleteDefaultWorkflowRequest<'a> {
        DeleteDefaultWorkflowRequest::new(self.client, id)
    }

    /// Returns the issue type mapping for the passed workflow scheme.
    pub fn get_workflow_scheme_issue_type(
        &self,
        id: i64,
        issue_type: impl Into<String>,
    ) -> GetWorkflowSchemeIssueTypeRequest<'a> {
        GetWorkflowSchemeIssueTypeRequest::new(self.client, id, issue_type)
    }

    /// Set the issue type mapping for the passed scheme. The passed representation can have its updateDraftIfNeeded flag set to true to indicate that
    /// the draft should be created/updated when the actual scheme cannot be edited.
    pub fn set_workflow_scheme_issue_type(
        &self,
        id: i64,
        issue_type: impl Into<String>,
        body: IssueTypeMapping,
    ) -> SetWorkflowSchemeIssueTypeRequest<'a> {
        SetWorkflowSchemeIssueTypeRequest::new(self.client, id, issue_type, body)
    }

    /// Remove the specified issue type mapping from the scheme.
    pub fn delete_workflow_scheme_issue_type(
        &self,
        id: i64,
        issue_type: impl Into<String>,
    ) -> DeleteWorkflowSchemeIssueTypeRequest<'a> {
        DeleteWorkflowSchemeIssueTypeRequest::new(self.client, id, issue_type)
    }

    /// Returns the workflow mappings or requested mapping to the caller for the passed scheme.
    pub fn get_workflow(&self, id: i64) -> GetWorkflowRequest<'a> {
        GetWorkflowRequest::new(self.client, id)
    }

    /// Update the scheme to include the passed mapping. The body is a representation of the workflow mapping. Values not passed are assumed to indicate no change for that field.
    /// The passed representation can have its updateDraftIfNeeded flag set to true to indicate that the draft
    /// should be created/updated when the actual scheme cannot be edited.
    pub fn update_workflow_mapping(
        &self,
        id: i64,
        workflow_mapping: WorkflowMapping,
    ) -> UpdateWorkflowMappingRequest<'a> {
        UpdateWorkflowMappingRequest::new(self.client, id, workflow_mapping)
    }

    /// Delete the passed workflow from the workflow scheme.
    pub fn delete_workflow_mapping(&self, id: i64) -> DeleteWorkflowMappingRequest<'a> {
        DeleteWorkflowMappingRequest::new(self.client, id)
    }
}

/// Create a new workflow scheme. The body contains a representation of the new scheme. Values not passed are assumed to be set to their defaults.
#[derive(Clone)]
pub struct CreateWorkflowSchemeRequest<'a> {
    client: &'a crate::core::Client,
    workflow_scheme: WorkflowScheme,
}

impl<'a> CreateWorkflowSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, workflow_scheme: WorkflowScheme) -> Self {
        Self { client, workflow_scheme }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/workflowscheme".to_owned());

        let body = match serde_json::to_value(&self.workflow_scheme)? {
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

/// Returns the requested workflow scheme to the caller.
#[derive(Clone)]
pub struct GetWorkflowSchemeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    return_draft_if_exists: Option<bool>,
}

impl<'a> GetWorkflowSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, return_draft_if_exists: None }
    }

    /// When true indicates that a scheme's draft, if it exists, should be queried instead of the scheme itself.
    #[must_use]
    pub fn return_draft_if_exists(mut self, value: bool) -> Self {
        self.return_draft_if_exists = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/workflowscheme/{}", self.id),
        );

        if let Some(value) = &self.return_draft_if_exists {
            config.query.push(("returnDraftIfExists".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Update the passed workflow scheme. The body of the request is a representation of the workflow scheme. Values not passed are assumed to indicate no change for that field.
/// The passed representation can have its updateDraftIfNeeded flag set to true to indicate that the draft
/// should be created and/or updated when the actual scheme cannot be edited (e.g. when the scheme is being used by
/// a project). Values not appearing the body will not be touched.
#[derive(Clone)]
pub struct UpdateWorkflowSchemeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    body: WorkflowScheme,
}

impl<'a> UpdateWorkflowSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, body: WorkflowScheme) -> Self {
        Self { client, id, body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/workflowscheme/{}", self.id),
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

/// Delete the passed workflow scheme.
#[derive(Clone)]
pub struct DeleteWorkflowSchemeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> DeleteWorkflowSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/workflowscheme/{}", self.id),
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

/// Return the default workflow from the passed workflow scheme.
#[derive(Clone)]
pub struct GetDefaultWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    return_draft_if_exists: Option<bool>,
}

impl<'a> GetDefaultWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, return_draft_if_exists: None }
    }

    /// When true indicates that a scheme's draft, if it exists, should be queried instead of the scheme itself.
    #[must_use]
    pub fn return_draft_if_exists(mut self, value: bool) -> Self {
        self.return_draft_if_exists = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/workflowscheme/{}/default", self.id),
        );

        if let Some(value) = &self.return_draft_if_exists {
            config.query.push(("returnDraftIfExists".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Set the default workflow for the passed workflow scheme. The passed representation can have its
/// updateDraftIfNeeded flag set to true to indicate that the draft should be created/updated when the actual scheme
/// cannot be edited.
#[derive(Clone)]
pub struct UpdateDefaultWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    default_workflow: DefaultModel,
}

impl<'a> UpdateDefaultWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, default_workflow: DefaultModel) -> Self {
        Self { client, id, default_workflow }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/workflowscheme/{}/default", self.id),
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

/// Remove the default workflow from the passed workflow scheme.
#[derive(Clone)]
pub struct DeleteDefaultWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    update_draft_if_needed: Option<bool>,
    id: i64,
}

impl<'a> DeleteDefaultWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, update_draft_if_needed: None }
    }

    /// When true will create and return a draft when the workflow scheme cannot be edited (e.g. when it is being used by a project).
    #[must_use]
    pub fn update_draft_if_needed(mut self, value: bool) -> Self {
        self.update_draft_if_needed = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/workflowscheme/{}/default", self.id),
        );

        if let Some(value) = &self.update_draft_if_needed {
            config.query.push(("updateDraftIfNeeded".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Returns the issue type mapping for the passed workflow scheme.
#[derive(Clone)]
pub struct GetWorkflowSchemeIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    issue_type: String,
    return_draft_if_exists: Option<bool>,
}

impl<'a> GetWorkflowSchemeIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, issue_type: impl Into<String>) -> Self {
        Self { client, id, issue_type: issue_type.into(), return_draft_if_exists: None }
    }

    /// When true indicates that a scheme's draft, if it exists, should be queried instead of the scheme itself.
    #[must_use]
    pub fn return_draft_if_exists(mut self, value: bool) -> Self {
        self.return_draft_if_exists = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/workflowscheme/{}/issuetype/{}",
                self.id,
                crate::core::encode_path_segment(&self.issue_type)
            ),
        );

        if let Some(value) = &self.return_draft_if_exists {
            config.query.push(("returnDraftIfExists".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeMapping> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Set the issue type mapping for the passed scheme. The passed representation can have its updateDraftIfNeeded flag set to true to indicate that
/// the draft should be created/updated when the actual scheme cannot be edited.
#[derive(Clone)]
pub struct SetWorkflowSchemeIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    issue_type: String,
    body: IssueTypeMapping,
}

impl<'a> SetWorkflowSchemeIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, issue_type: impl Into<String>, body: IssueTypeMapping) -> Self {
        Self { client, id, issue_type: issue_type.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/workflowscheme/{}/issuetype/{}",
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

/// Remove the specified issue type mapping from the scheme.
#[derive(Clone)]
pub struct DeleteWorkflowSchemeIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    update_draft_if_needed: Option<bool>,
    issue_type: String,
}

impl<'a> DeleteWorkflowSchemeIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, issue_type: impl Into<String>) -> Self {
        Self { client, id, issue_type: issue_type.into(), update_draft_if_needed: None }
    }

    /// When true will create and return a draft when the workflow scheme cannot be edited (e.g. when it is being used by a project).
    #[must_use]
    pub fn update_draft_if_needed(mut self, value: bool) -> Self {
        self.update_draft_if_needed = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/workflowscheme/{}/issuetype/{}",
                self.id,
                crate::core::encode_path_segment(&self.issue_type)
            ),
        );

        if let Some(value) = &self.update_draft_if_needed {
            config.query.push(("updateDraftIfNeeded".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Returns the workflow mappings or requested mapping to the caller for the passed scheme.
#[derive(Clone)]
pub struct GetWorkflowRequest<'a> {
    client: &'a crate::core::Client,
    workflow_name: Option<String>,
    id: i64,
    return_draft_if_exists: Option<bool>,
}

impl<'a> GetWorkflowRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, workflow_name: None, return_draft_if_exists: None }
    }

    /// The workflow mapping to return. Null can be passed to return all mappings. Must be a valid workflow name.
    #[must_use]
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());

        self
    }

    /// When true indicates that a scheme's draft, if it exists, should be queried instead of the scheme itself.
    #[must_use]
    pub fn return_draft_if_exists(mut self, value: bool) -> Self {
        self.return_draft_if_exists = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/workflowscheme/{}/workflow", self.id),
        );

        if let Some(value) = &self.workflow_name {
            config.query.push(("workflowName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.return_draft_if_exists {
            config.query.push(("returnDraftIfExists".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetWorkflow> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update the scheme to include the passed mapping. The body is a representation of the workflow mapping. Values not passed are assumed to indicate no change for that field.
/// The passed representation can have its updateDraftIfNeeded flag set to true to indicate that the draft
/// should be created/updated when the actual scheme cannot be edited.
#[derive(Clone)]
pub struct UpdateWorkflowMappingRequest<'a> {
    client: &'a crate::core::Client,
    workflow_name: Option<String>,
    id: i64,
    workflow_mapping: WorkflowMapping,
}

impl<'a> UpdateWorkflowMappingRequest<'a> {
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
            format!("/rest/api/2/workflowscheme/{}/workflow", self.id),
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

/// Delete the passed workflow from the workflow scheme.
#[derive(Clone)]
pub struct DeleteWorkflowMappingRequest<'a> {
    client: &'a crate::core::Client,
    update_draft_if_needed: Option<bool>,
    workflow_name: Option<String>,
    id: i64,
}

impl<'a> DeleteWorkflowMappingRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, update_draft_if_needed: None, workflow_name: None }
    }

    /// Flag to indicate if a draft should be created if necessary to delete the workflow from the scheme.
    #[must_use]
    pub fn update_draft_if_needed(mut self, value: bool) -> Self {
        self.update_draft_if_needed = Some(value);

        self
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
            format!("/rest/api/2/workflowscheme/{}/workflow", self.id),
        );

        if let Some(value) = &self.update_draft_if_needed {
            config.query.push(("updateDraftIfNeeded".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

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
