// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectProperties operations.
pub struct ProjectPropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectPropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the keys of all properties for the project identified by the key or by the id.
    pub fn get_project_property_keys(&self, project_id_or_key: impl Into<String>) -> GetProjectPropertyKeysRequest<'a> {
        GetProjectPropertyKeysRequest::new(self.client, project_id_or_key)
    }

    /// Returns the value of the property with a given key from the project identified by the key or by the id.
    pub fn get_project_property(
        &self,
        property_key: impl Into<String>,
        project_id_or_key: impl Into<String>,
    ) -> GetProjectPropertyRequest<'a> {
        GetProjectPropertyRequest::new(self.client, property_key, project_id_or_key)
    }

    /// Sets the value of the specified project's property. You can use this resource to store a custom data against the project identified by the key or by the id. The user who stores the data is required to have permissions to administer the project.
    pub fn set_project_property(
        &self,
        property_key: impl Into<String>,
        project_id_or_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetProjectPropertyRequest<'a> {
        SetProjectPropertyRequest::new(self.client, property_key, project_id_or_key, body)
    }

    /// Removes the property from the project identified by the key or by the id.
    pub fn delete_project_property(
        &self,
        property_key: impl Into<String>,
        project_id_or_key: impl Into<String>,
    ) -> DeleteProjectPropertyRequest<'a> {
        DeleteProjectPropertyRequest::new(self.client, property_key, project_id_or_key)
    }
}

/// Returns the keys of all properties for the project identified by the key or by the id.
#[derive(Clone)]
pub struct GetProjectPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> GetProjectPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/properties", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityPropertiesKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the project identified by the key or by the id.
#[derive(Clone)]
pub struct GetProjectPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    project_id_or_key: String,
}

impl<'a> GetProjectPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        project_id_or_key: impl Into<String>,
    ) -> Self {
        Self { client, property_key: property_key.into(), project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/project/{}/properties/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
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

/// Sets the value of the specified project's property. You can use this resource to store a custom data against the project identified by the key or by the id. The user who stores the data is required to have permissions to administer the project.
#[derive(Clone)]
pub struct SetProjectPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    project_id_or_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetProjectPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        project_id_or_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, property_key: property_key.into(), project_id_or_key: project_id_or_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/project/{}/properties/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
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

/// Removes the property from the project identified by the key or by the id.
#[derive(Clone)]
pub struct DeleteProjectPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    project_id_or_key: String,
}

impl<'a> DeleteProjectPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        project_id_or_key: impl Into<String>,
    ) -> Self {
        Self { client, property_key: property_key.into(), project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/project/{}/properties/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
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
