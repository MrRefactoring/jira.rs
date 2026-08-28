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

    /// Load a single icon by id
    pub fn get_icon(&self, id: impl Into<String>) -> GetIconRequest<'a> {
        GetIconRequest::new(self.client, id)
    }

    /// Load a single icon PNG by id
    pub fn get_icon_image(&self, id: impl Into<String>) -> GetIconImageRequest<'a> {
        GetIconImageRequest::new(self.client, id)
    }

    /// Return all global icons i.e. icons not associated with a particular object schema
    pub fn find_global_icons(&self) -> FindGlobalIconsRequest<'a> {
        FindGlobalIconsRequest::new(self.client)
    }
}

/// Load a single icon by id
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
            format!("/icon/{}", crate::core::encode_path_segment(&self.id)),
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

/// Load a single icon PNG by id
pub struct GetIconImageRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    size: Option<i64>,
}

impl<'a> GetIconImageRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), size: None }
    }

    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/icon/{}/icon.png", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.size {
            config.query.push(("size".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.headers.push(("Accept".to_owned(), "image/png".to_owned()));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<bytes::Bytes> {
        self.client.send_bytes(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Return all global icons i.e. icons not associated with a particular object schema
pub struct FindGlobalIconsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FindGlobalIconsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/icon/global".to_owned());

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
