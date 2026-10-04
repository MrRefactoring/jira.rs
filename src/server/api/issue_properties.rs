// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueProperties operations.
pub struct IssuePropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssuePropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the keys of all properties for the issue identified by the key or by the id.
    pub fn get_issue_property_keys(&self, issue_id_or_key: impl Into<String>) -> GetIssuePropertyKeysRequest<'a> {
        GetIssuePropertyKeysRequest::new(self.client, issue_id_or_key)
    }

    /// Returns the value of the property with a given key from the issue identified by the key or by the id.
    pub fn get_issue_property(
        &self,
        issue_id_or_key: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetIssuePropertyRequest<'a> {
        GetIssuePropertyRequest::new(self.client, issue_id_or_key, property_key)
    }

    /// Sets the value of the specified issue's property.
    pub fn set_issue_property(
        &self,
        issue_id_or_key: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetIssuePropertyRequest<'a> {
        SetIssuePropertyRequest::new(self.client, issue_id_or_key, property_key, body)
    }

    /// Removes the property from the issue identified by the key or by the id.
    pub fn delete_issue_property(
        &self,
        issue_id_or_key: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteIssuePropertyRequest<'a> {
        DeleteIssuePropertyRequest::new(self.client, issue_id_or_key, property_key)
    }
}

/// Returns the keys of all properties for the issue identified by the key or by the id.
#[derive(Clone)]
pub struct GetIssuePropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetIssuePropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/properties", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the issue identified by the key or by the id.
#[derive(Clone)]
pub struct GetIssuePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    property_key: String,
}

impl<'a> GetIssuePropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issue/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of the specified issue's property.
#[derive(Clone)]
pub struct SetIssuePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetIssuePropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/issue/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Removes the property from the issue identified by the key or by the id.
#[derive(Clone)]
pub struct DeleteIssuePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    property_key: String,
}

impl<'a> DeleteIssuePropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/issue/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.property_key)
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
