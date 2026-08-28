// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The StatusTypes operations.
pub struct StatusTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> StatusTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Find all status
    pub fn find_status_types(&self) -> FindStatusTypesRequest<'a> {
        FindStatusTypesRequest::new(self.client)
    }

    /// Create a new status
    pub fn create_status_type(&self, status_in: StatusIn) -> CreateStatusTypeRequest<'a> {
        CreateStatusTypeRequest::new(self.client, status_in)
    }

    /// Find a status by id
    pub fn get_status_type(&self, id: impl Into<String>) -> GetStatusTypeRequest<'a> {
        GetStatusTypeRequest::new(self.client, id)
    }

    /// Update an existing status
    pub fn update_status_type(&self, id: impl Into<String>, status_in: StatusIn) -> UpdateStatusTypeRequest<'a> {
        UpdateStatusTypeRequest::new(self.client, id, status_in)
    }

    /// Delete an existing status
    pub fn delete_status_type(&self, id: impl Into<String>) -> DeleteStatusTypeRequest<'a> {
        DeleteStatusTypeRequest::new(self.client, id)
    }
}

/// Find all status
pub struct FindStatusTypesRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_id: Option<String>,
}

impl<'a> FindStatusTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, object_schema_id: None }
    }

    /// Include statuses for the object schema id. If supplied statuses for the object schema will be returned otherwise all global will be returned
    #[must_use]
    pub fn object_schema_id(mut self, value: impl Into<String>) -> Self {
        self.object_schema_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/config/statustype".to_owned());

        if let Some(value) = &self.object_schema_id {
            config.query.push(("objectSchemaId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Status>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a new status
pub struct CreateStatusTypeRequest<'a> {
    client: &'a crate::core::Client,
    status_in: StatusIn,
}

impl<'a> CreateStatusTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, status_in: StatusIn) -> Self {
        Self { client, status_in }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/config/statustype".to_owned());

        let body = match serde_json::to_value(&self.status_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Status> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Find a status by id
pub struct GetStatusTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetStatusTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/config/statustype/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Status> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an existing status
pub struct UpdateStatusTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    status_in: StatusIn,
}

impl<'a> UpdateStatusTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, status_in: StatusIn) -> Self {
        Self { client, id: id.into(), status_in }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/config/statustype/{}", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.status_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Status> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete an existing status
pub struct DeleteStatusTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteStatusTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/config/statustype/{}", crate::core::encode_path_segment(&self.id)),
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
