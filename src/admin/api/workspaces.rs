// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Workspaces operations.
pub struct WorkspacesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkspacesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// A workspace refers to a specific instance of an Atlassian product that is accessed through a unique URL. Whenever a user initiates or adds a new product instance, it results in the creation of a distinct workspace.
    ///
    /// This API will:
    /// - Return a paginated list of workspaces in a given org
    /// - Return more details about an organization's products (including product URL).
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:workspaces:admin`
    pub fn query_workspaces(&self, org_id: impl Into<String>) -> QueryWorkspacesRequest<'a> {
        QueryWorkspacesRequest::new(self.client, org_id)
    }
}

/// A workspace refers to a specific instance of an Atlassian product that is accessed through a unique URL. Whenever a user initiates or adds a new product instance, it results in the creation of a distinct workspace.
///
/// This API will:
/// - Return a paginated list of workspaces in a given org
/// - Return more details about an organization's products (including product URL).
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:workspaces:admin`
pub struct QueryWorkspacesRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    search_workspaces_request_v2: Option<SearchWorkspacesRequestV2>,
}

impl<'a> QueryWorkspacesRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), search_workspaces_request_v2: None }
    }

    #[must_use]
    pub fn search_workspaces_request_v2(mut self, value: SearchWorkspacesRequestV2) -> Self {
        self.search_workspaces_request_v2 = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/admin/v2/orgs/{}/workspaces", self.org_id),
        );

        let body = match serde_json::to_value(&self.search_workspaces_request_v2)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageDataResponseV2> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
