// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The AppDataPolicies operations.
pub struct AppDataPoliciesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AppDataPoliciesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns data policy for the workspace.
    pub fn get_policy(&self) -> GetPolicyRequest<'a> {
        GetPolicyRequest::new(self.client)
    }

    /// Returns data policies for the projects specified in the request.
    pub fn get_policies(&self) -> GetPoliciesRequest<'a> {
        GetPoliciesRequest::new(self.client)
    }
}

/// Returns data policy for the workspace.
#[derive(Clone)]
pub struct GetPolicyRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetPolicyRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/data-policy".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkspaceDataPolicy> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns data policies for the projects specified in the request.
#[derive(Clone)]
pub struct GetPoliciesRequest<'a> {
    client: &'a crate::core::Client,
    ids: Option<Vec<String>>,
}

impl<'a> GetPoliciesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, ids: None }
    }

    /// A list of project identifiers. This parameter accepts a comma-separated list.
    #[must_use]
    pub fn ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/data-policy/project".to_owned());

        if let Some(value) = &self.ids {
            config.query.push(("ids".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectDataPolicies> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
