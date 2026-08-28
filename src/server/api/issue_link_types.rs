// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueLinkTypes operations.
pub struct IssueLinkTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueLinkTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of available issue link types, if issue linking is enabled.
    pub fn get_issue_link_types(&self) -> GetIssueLinkTypesRequest<'a> {
        GetIssueLinkTypesRequest::new(self.client)
    }

    /// Create a new issue link type.
    pub fn create_issue_link_type(&self, issue_link_type_json: IssueLinkTypeJson) -> CreateIssueLinkTypeRequest<'a> {
        CreateIssueLinkTypeRequest::new(self.client, issue_link_type_json)
    }

    /// Resets the order of issue link types alphabetically.
    ///
    /// Available since Jira Data Center 10.4.
    pub fn reset_order(
        &self,
        issue_link_type_reset_order_request: IssueLinkTypeResetOrderRequest,
    ) -> ResetOrderRequest<'a> {
        ResetOrderRequest::new(self.client, issue_link_type_reset_order_request)
    }

    /// Returns for a given issue link type id all information about this issue link type.
    pub fn get_issue_link_type(&self, issue_link_type_id: impl Into<String>) -> GetIssueLinkTypeRequest<'a> {
        GetIssueLinkTypeRequest::new(self.client, issue_link_type_id)
    }

    /// Update the specified issue link type.
    pub fn update_issue_link_type(
        &self,
        issue_link_type_id: impl Into<String>,
        issue_link_type_json: IssueLinkTypeJson,
    ) -> UpdateIssueLinkTypeRequest<'a> {
        UpdateIssueLinkTypeRequest::new(self.client, issue_link_type_id, issue_link_type_json)
    }

    /// Delete the specified issue link type.
    pub fn delete_issue_link_type(&self, issue_link_type_id: impl Into<String>) -> DeleteIssueLinkTypeRequest<'a> {
        DeleteIssueLinkTypeRequest::new(self.client, issue_link_type_id)
    }

    /// Moves the issue link type to a new position within the list.
    ///
    /// Available since Jira Data Center 10.4.
    pub fn move_issue_link_type(
        &self,
        issue_link_type_id: impl Into<String>,
        issue_link_type_order_update_request: IssueLinkTypeOrderUpdateRequest,
    ) -> MoveIssueLinkTypeRequest<'a> {
        MoveIssueLinkTypeRequest::new(self.client, issue_link_type_id, issue_link_type_order_update_request)
    }
}

/// Returns a list of available issue link types, if issue linking is enabled.
#[derive(Clone)]
pub struct GetIssueLinkTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIssueLinkTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/issueLinkType".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypes> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a new issue link type.
#[derive(Clone)]
pub struct CreateIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_json: IssueLinkTypeJson,
}

impl<'a> CreateIssueLinkTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_link_type_json: IssueLinkTypeJson) -> Self {
        Self { client, issue_link_type_json }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/issueLinkType".to_owned());

        let body = match serde_json::to_value(&self.issue_link_type_json)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypeJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Resets the order of issue link types alphabetically.
///
/// Available since Jira Data Center 10.4.
#[derive(Clone)]
pub struct ResetOrderRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_reset_order_request: IssueLinkTypeResetOrderRequest,
}

impl<'a> ResetOrderRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_link_type_reset_order_request: IssueLinkTypeResetOrderRequest,
    ) -> Self {
        Self { client, issue_link_type_reset_order_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/issueLinkType/order".to_owned());

        let body = match serde_json::to_value(&self.issue_link_type_reset_order_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypes> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns for a given issue link type id all information about this issue link type.
#[derive(Clone)]
pub struct GetIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
}

impl<'a> GetIssueLinkTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_link_type_id: impl Into<String>) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issueLinkType/{}", crate::core::encode_path_segment(&self.issue_link_type_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypeJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update the specified issue link type.
#[derive(Clone)]
pub struct UpdateIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
    issue_link_type_json: IssueLinkTypeJson,
}

impl<'a> UpdateIssueLinkTypeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_link_type_id: impl Into<String>,
        issue_link_type_json: IssueLinkTypeJson,
    ) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into(), issue_link_type_json }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issueLinkType/{}", crate::core::encode_path_segment(&self.issue_link_type_id)),
        );

        let body = match serde_json::to_value(&self.issue_link_type_json)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypeJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the specified issue link type.
#[derive(Clone)]
pub struct DeleteIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
}

impl<'a> DeleteIssueLinkTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_link_type_id: impl Into<String>) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issueLinkType/{}", crate::core::encode_path_segment(&self.issue_link_type_id)),
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

/// Moves the issue link type to a new position within the list.
///
/// Available since Jira Data Center 10.4.
#[derive(Clone)]
pub struct MoveIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
    issue_link_type_order_update_request: IssueLinkTypeOrderUpdateRequest,
}

impl<'a> MoveIssueLinkTypeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_link_type_id: impl Into<String>,
        issue_link_type_order_update_request: IssueLinkTypeOrderUpdateRequest,
    ) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into(), issue_link_type_order_update_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issueLinkType/{}/order", crate::core::encode_path_segment(&self.issue_link_type_id)),
        );

        let body = match serde_json::to_value(&self.issue_link_type_order_update_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypeJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
