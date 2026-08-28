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

    /// Get details on a given status.
    pub fn get_status_type(&self, id: impl Into<String>) -> GetStatusTypeRequest<'a> {
        GetStatusTypeRequest::new(self.client, id)
    }

    /// Update an existing status type.
    pub fn update_status_type(&self, id: impl Into<String>) -> UpdateStatusTypeRequest<'a> {
        UpdateStatusTypeRequest::new(self.client, id)
    }

    /// Delete a single status type.
    pub fn delete_status_type(&self, id: impl Into<String>) -> DeleteStatusTypeRequest<'a> {
        DeleteStatusTypeRequest::new(self.client, id)
    }

    /// Find status types for a given object schema ID.
    pub fn find_status_types(&self) -> FindStatusTypesRequest<'a> {
        FindStatusTypesRequest::new(self.client)
    }

    /// Store a new status type.
    pub fn store_status_type(&self) -> StoreStatusTypeRequest<'a> {
        StoreStatusTypeRequest::new(self.client)
    }
}

/// Get details on a given status.
#[derive(Clone)]
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
            format!("/rest/assets/1.0/config/statustype/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an existing status type.
#[derive(Clone)]
pub struct UpdateStatusTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    body: Option<StatusType>,
}

impl<'a> UpdateStatusTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), body: None }
    }

    #[must_use]
    pub fn body(mut self, value: StatusType) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/assets/1.0/config/statustype/{}", crate::core::encode_path_segment(&self.id)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a single status type.
#[derive(Clone)]
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
            format!("/rest/assets/1.0/config/statustype/{}", crate::core::encode_path_segment(&self.id)),
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

/// Find status types for a given object schema ID.
#[derive(Clone)]
pub struct FindStatusTypesRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_id: Option<String>,
}

impl<'a> FindStatusTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, object_schema_id: None }
    }

    /// Include statuses for the object schema ID. If this parameter is supplied, statuses for the given object schema will be returned. Otherwise all global statuses will be returned.
    #[must_use]
    pub fn object_schema_id(mut self, value: impl Into<String>) -> Self {
        self.object_schema_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/assets/1.0/config/statustype".to_owned());

        if let Some(value) = &self.object_schema_id {
            config.query.push(("objectSchemaId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<StatusType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Store a new status type.
#[derive(Clone)]
pub struct StoreStatusTypeRequest<'a> {
    client: &'a crate::core::Client,
    status_type: Option<StatusType>,
}

impl<'a> StoreStatusTypeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, status_type: None }
    }

    #[must_use]
    pub fn status_type(mut self, value: StatusType) -> Self {
        self.status_type = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/assets/1.0/config/statustype".to_owned());

        let body = match serde_json::to_value(&self.status_type)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
