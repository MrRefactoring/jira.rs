// @generated. Do not edit: change the generator or the specification.

/// The MyPreferences operations.
pub struct MyPreferencesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> MyPreferencesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns preference of the currently logged in user. Preference key must be provided as input parameter (key). The value is returned exactly as it is. If key parameter is not provided or wrong - status code 404. If value is found  - status code 200.
    pub fn get_preference(&self) -> GetPreferenceRequest<'a> {
        GetPreferenceRequest::new(self.client)
    }

    /// Sets preference of the currently logged in user. Preference key must be provided as input parameters (key). Value must be provided as post body. If key or value parameter is not provided - status code 404. If preference is set - status code 204.
    pub fn set_preference(&self) -> SetPreferenceRequest<'a> {
        SetPreferenceRequest::new(self.client)
    }

    /// Removes preference of the currently logged in user. Preference key must be provided as input parameters (key). If key parameter is not provided or wrong - status code 404. If preference is unset - status code 204.
    pub fn remove_preference(&self) -> RemovePreferenceRequest<'a> {
        RemovePreferenceRequest::new(self.client)
    }
}

/// Returns preference of the currently logged in user. Preference key must be provided as input parameter (key). The value is returned exactly as it is. If key parameter is not provided or wrong - status code 404. If value is found  - status code 200.
#[derive(Clone)]
pub struct GetPreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
}

impl<'a> GetPreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, key: None }
    }

    /// Key of the preference to be returned.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/mypreferences".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<String> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets preference of the currently logged in user. Preference key must be provided as input parameters (key). Value must be provided as post body. If key or value parameter is not provided - status code 404. If preference is set - status code 204.
#[derive(Clone)]
pub struct SetPreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
    body: Option<String>,
}

impl<'a> SetPreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, key: None, body: None }
    }

    /// Key of the preference to be set.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/mypreferences".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Removes preference of the currently logged in user. Preference key must be provided as input parameters (key). If key parameter is not provided or wrong - status code 404. If preference is unset - status code 204.
#[derive(Clone)]
pub struct RemovePreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
}

impl<'a> RemovePreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, key: None }
    }

    /// Key of the preference to be removed.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/2/mypreferences".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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
