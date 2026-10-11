// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueTypeProperties operations.
pub struct IssueTypePropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueTypePropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the keys of all properties for the issue type identified by the id
    pub fn get_issue_type_property_keys(
        &self,
        issue_type_id: impl Into<String>,
    ) -> GetIssueTypePropertyKeysRequest<'a> {
        GetIssueTypePropertyKeysRequest::new(self.client, issue_type_id)
    }

    /// Returns the value of the property with a given key from the issue type identified by the id
    pub fn get_issue_type_property(
        &self,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetIssueTypePropertyRequest<'a> {
        GetIssueTypePropertyRequest::new(self.client, issue_type_id, property_key)
    }

    /// Sets the value of the specified issue type's property
    pub fn set_issue_type_property(
        &self,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetIssueTypePropertyRequest<'a> {
        SetIssueTypePropertyRequest::new(self.client, issue_type_id, property_key, body)
    }

    /// Removes the property from the issue type identified by the id
    pub fn delete_issue_type_property(
        &self,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteIssueTypePropertyRequest<'a> {
        DeleteIssueTypePropertyRequest::new(self.client, issue_type_id, property_key)
    }
}

/// Returns the keys of all properties for the issue type identified by the id
#[derive(Clone)]
pub struct GetIssueTypePropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
}

impl<'a> GetIssueTypePropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_id: impl Into<String>) -> Self {
        Self { client, issue_type_id: issue_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issuetype/{}/properties", crate::core::encode_path_segment(&self.issue_type_id)),
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

/// Returns the value of the property with a given key from the issue type identified by the id
#[derive(Clone)]
pub struct GetIssueTypePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
    property_key: String,
}

impl<'a> GetIssueTypePropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, issue_type_id: issue_type_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issuetype/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_type_id),
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

/// Sets the value of the specified issue type's property
#[derive(Clone)]
pub struct SetIssueTypePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetIssueTypePropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, issue_type_id: issue_type_id.into(), property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/issuetype/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_type_id),
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

/// Removes the property from the issue type identified by the id
#[derive(Clone)]
pub struct DeleteIssueTypePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
    property_key: String,
}

impl<'a> DeleteIssueTypePropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, issue_type_id: issue_type_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/issuetype/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_type_id),
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
