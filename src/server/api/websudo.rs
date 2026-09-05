// @generated. Do not edit: change the generator or the specification.

/// The Websudo operations.
pub struct WebsudoService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WebsudoService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// This method invalidates the any current WebSudo session.
    pub fn release(&self) -> ReleaseRequest<'a> {
        ReleaseRequest::new(self.client)
    }
}

/// This method invalidates the any current WebSudo session.
#[derive(Clone)]
pub struct ReleaseRequest<'a> {
    client: &'a crate::core::Client,
    body: Option<String>,
}

impl<'a> ReleaseRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, body: None }
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/auth/1/websudo".to_owned());

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
