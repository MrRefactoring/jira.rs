// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Policies operations.
pub struct PoliciesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PoliciesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns information about org policies
    pub fn get_policies(&self, org_id: impl Into<String>) -> GetPoliciesRequest<'a> {
        GetPoliciesRequest::new(self.client, org_id)
    }

    /// Create a policy for an org
    pub fn create_policy(&self, org_id: impl Into<String>) -> CreatePolicyRequest<'a> {
        CreatePolicyRequest::new(self.client, org_id)
    }

    /// Returns information about a single policy by ID
    pub fn get_policy_by_id(
        &self,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
    ) -> GetPolicyByIdRequest<'a> {
        GetPolicyByIdRequest::new(self.client, org_id, policy_id)
    }

    /// Update a policy for an org
    pub fn update_policy(&self, org_id: impl Into<String>, policy_id: impl Into<String>) -> UpdatePolicyRequest<'a> {
        UpdatePolicyRequest::new(self.client, org_id, policy_id)
    }

    /// Delete a policy for an org
    pub fn delete_policy(&self, org_id: impl Into<String>, policy_id: impl Into<String>) -> DeletePolicyRequest<'a> {
        DeletePolicyRequest::new(self.client, org_id, policy_id)
    }

    /// Adds a resource to an existing Policy
    pub fn add_resource_to_policy(
        &self,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
    ) -> AddResourceToPolicyRequest<'a> {
        AddResourceToPolicyRequest::new(self.client, org_id, policy_id)
    }

    /// Update an existing Policy Resource
    pub fn update_policy_resource(
        &self,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> UpdatePolicyResourceRequest<'a> {
        UpdatePolicyResourceRequest::new(self.client, org_id, policy_id, resource_id)
    }

    /// Delete an existing Policy Resource
    pub fn delete_policy_resource(
        &self,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> DeletePolicyResourceRequest<'a> {
        DeletePolicyResourceRequest::new(self.client, org_id, policy_id, resource_id)
    }

    /// Validate a policy based on specific requirements. For example, Trigger CDEN validation by pushing a task into the SQS dns-validation queue
    pub fn validate_policy(
        &self,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
    ) -> ValidatePolicyRequest<'a> {
        ValidatePolicyRequest::new(self.client, org_id, policy_id)
    }
}

/// Returns information about org policies
pub struct GetPoliciesRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    cursor: Option<String>,
    r#type: Option<String>,
}

impl<'a> GetPoliciesRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), cursor: None, r#type: None }
    }

    /// Sets the starting point for the page of results to return.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// Sets the type for the page of policies to return.
    #[must_use]
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/admin/v1/orgs/{}/policies", crate::core::encode_path_segment(&self.org_id)),
        );

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.r#type {
            config.query.push(("type".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PolicyPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a policy for an org
pub struct CreatePolicyRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_create_input: Option<PolicyCreateInput>,
}

impl<'a> CreatePolicyRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), policy_create_input: None }
    }

    #[must_use]
    pub fn policy_create_input(mut self, value: PolicyCreateInput) -> Self {
        self.policy_create_input = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/admin/v1/orgs/{}/policies", crate::core::encode_path_segment(&self.org_id)),
        );

        let body = match serde_json::to_value(&self.policy_create_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Policy> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns information about a single policy by ID
pub struct GetPolicyByIdRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
}

impl<'a> GetPolicyByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, policy_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), policy_id: policy_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v1/orgs/{}/policies/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Policy> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update a policy for an org
pub struct UpdatePolicyRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
    policy_update_input: Option<PolicyUpdateInput>,
}

impl<'a> UpdatePolicyRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, policy_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), policy_id: policy_id.into(), policy_update_input: None }
    }

    #[must_use]
    pub fn policy_update_input(mut self, value: PolicyUpdateInput) -> Self {
        self.policy_update_input = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/admin/v1/orgs/{}/policies/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id)
            ),
        );

        let body = match serde_json::to_value(&self.policy_update_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Policy> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a policy for an org
pub struct DeletePolicyRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
}

impl<'a> DeletePolicyRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, policy_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), policy_id: policy_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/admin/v1/orgs/{}/policies/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id)
            ),
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

/// Adds a resource to an existing Policy
pub struct AddResourceToPolicyRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
    resource_input: Option<ResourceInput>,
}

impl<'a> AddResourceToPolicyRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, policy_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), policy_id: policy_id.into(), resource_input: None }
    }

    #[must_use]
    pub fn resource_input(mut self, value: ResourceInput) -> Self {
        self.resource_input = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/admin/v1/orgs/{}/policies/{}/resources",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id)
            ),
        );

        let body = match serde_json::to_value(&self.resource_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Policy> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an existing Policy Resource
pub struct UpdatePolicyResourceRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
    resource_id: String,
    resource_update_input: Option<ResourceUpdateInput>,
}

impl<'a> UpdatePolicyResourceRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            policy_id: policy_id.into(),
            resource_id: resource_id.into(),
            resource_update_input: None,
        }
    }

    #[must_use]
    pub fn resource_update_input(mut self, value: ResourceUpdateInput) -> Self {
        self.resource_update_input = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/admin/v1/orgs/{}/policies/{}/resources/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id),
                crate::core::encode_path_segment(&self.resource_id)
            ),
        );

        let body = match serde_json::to_value(&self.resource_update_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Policy> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete an existing Policy Resource
pub struct DeletePolicyResourceRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
    resource_id: String,
}

impl<'a> DeletePolicyResourceRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        policy_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        Self { client, org_id: org_id.into(), policy_id: policy_id.into(), resource_id: resource_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/admin/v1/orgs/{}/policies/{}/resources/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id),
                crate::core::encode_path_segment(&self.resource_id)
            ),
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

/// Validate a policy based on specific requirements. For example, Trigger CDEN validation by pushing a task into the SQS dns-validation queue
pub struct ValidatePolicyRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    policy_id: String,
}

impl<'a> ValidatePolicyRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, policy_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), policy_id: policy_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v1/orgs/{}/policies/{}/validate",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.policy_id)
            ),
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
