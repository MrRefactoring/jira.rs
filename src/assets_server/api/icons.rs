// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Icons operations.
pub struct IconsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IconsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get all global icons i.e. icons not associated with a particular object schema.
    pub fn find_global_icons(&self) -> FindGlobalIconsRequest<'a> {
        FindGlobalIconsRequest::new(self.client)
    }

    /// Get all icons associated with an object schema. This resource will not include global icons.
    pub fn find_icons(&self, id: impl Into<String>) -> FindIconsRequest<'a> {
        FindIconsRequest::new(self.client, id)
    }

    /// Get a single icon by ID.
    pub fn get_icon(&self, id: impl Into<String>) -> GetIconRequest<'a> {
        GetIconRequest::new(self.client, id)
    }
}

/// Get all global icons i.e. icons not associated with a particular object schema.
pub struct FindGlobalIconsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FindGlobalIconsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/assets/1.0/icon/global".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Icon>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get all icons associated with an object schema. This resource will not include global icons.
pub struct FindIconsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> FindIconsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/icon/objectschema/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Icon>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get a single icon by ID.
pub struct GetIconRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetIconRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/icon/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Icon> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
