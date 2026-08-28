// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The PrioritySchemes operations.
pub struct PrioritySchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PrioritySchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all priority schemes. All project keys associated with the priority scheme will only be returned if additional query parameter is provided `expand=schemes.projectKeys`
    pub fn get_priority_schemes(&self) -> GetPrioritySchemesRequest<'a> {
        GetPrioritySchemesRequest::new(self.client)
    }

    /// Creates new priority scheme.
    pub fn create_priority_scheme(
        &self,
        priority_scheme_update: PrioritySchemeUpdate,
    ) -> CreatePrioritySchemeRequest<'a> {
        CreatePrioritySchemeRequest::new(self.client, priority_scheme_update)
    }

    /// Gets a full representation of a priority scheme in JSON format.
    pub fn get_priority_scheme(&self, scheme_id: i64) -> GetPrioritySchemeRequest<'a> {
        GetPrioritySchemeRequest::new(self.client, scheme_id)
    }

    /// Updates a priority scheme. Update will be rejected if issue migration would be needed as a result of scheme update. Priority scheme update with migration is possible from the UI.
    pub fn update_priority_scheme(
        &self,
        scheme_id: i64,
        priority_scheme_update: PrioritySchemeUpdate,
    ) -> UpdatePrioritySchemeRequest<'a> {
        UpdatePrioritySchemeRequest::new(self.client, scheme_id, priority_scheme_update)
    }

    /// Deletes a priority scheme. All projects using deleted scheme will use default priority scheme afterwards.
    pub fn delete_priority_scheme(&self, scheme_id: i64) -> DeletePrioritySchemeRequest<'a> {
        DeletePrioritySchemeRequest::new(self.client, scheme_id)
    }
}

/// Returns all priority schemes. All project keys associated with the priority scheme will only be returned if additional query parameter is provided `expand=schemes.projectKeys`
#[derive(Clone)]
pub struct GetPrioritySchemesRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    start_at: Option<i64>,
}

impl<'a> GetPrioritySchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, start_at: None }
    }

    /// how many results on the page should be included. Defaults to 100, maximum is 1000.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// the page offset, if not specified then defaults to 0
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/priorityschemes".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PrioritySchemeList> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates new priority scheme.
#[derive(Clone)]
pub struct CreatePrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    priority_scheme_update: PrioritySchemeUpdate,
}

impl<'a> CreatePrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, priority_scheme_update: PrioritySchemeUpdate) -> Self {
        Self { client, priority_scheme_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/priorityschemes".to_owned());

        let body = match serde_json::to_value(&self.priority_scheme_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PriorityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Gets a full representation of a priority scheme in JSON format.
#[derive(Clone)]
pub struct GetPrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: i64,
}

impl<'a> GetPrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/priorityschemes/{}", self.scheme_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PriorityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a priority scheme. Update will be rejected if issue migration would be needed as a result of scheme update. Priority scheme update with migration is possible from the UI.
#[derive(Clone)]
pub struct UpdatePrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: i64,
    priority_scheme_update: PrioritySchemeUpdate,
}

impl<'a> UpdatePrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64, priority_scheme_update: PrioritySchemeUpdate) -> Self {
        Self { client, scheme_id, priority_scheme_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/priorityschemes/{}", self.scheme_id),
        );

        let body = match serde_json::to_value(&self.priority_scheme_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PriorityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a priority scheme. All projects using deleted scheme will use default priority scheme afterwards.
#[derive(Clone)]
pub struct DeletePrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: i64,
}

impl<'a> DeletePrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: i64) -> Self {
        Self { client, scheme_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/priorityschemes/{}", self.scheme_id),
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
