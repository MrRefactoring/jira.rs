// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ReferenceTypes operations.
pub struct ReferenceTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ReferenceTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get reference type
    pub fn find_reference_types(&self) -> FindReferenceTypesRequest<'a> {
        FindReferenceTypesRequest::new(self.client)
    }

    /// Update a reference type
    pub fn create_reference_type(&self, reference_type_in: ReferenceTypeIn) -> CreateReferenceTypeRequest<'a> {
        CreateReferenceTypeRequest::new(self.client, reference_type_in)
    }
}

/// Get reference type
#[derive(Clone)]
pub struct FindReferenceTypesRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_id: Option<String>,
    include_all: Option<bool>,
}

impl<'a> FindReferenceTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, object_schema_id: None, include_all: None }
    }

    /// Include reference types for the object schema id. If supplied reference types for the object schema will be returned otherwise all global will be returned
    #[must_use]
    pub fn object_schema_id(mut self, value: impl Into<String>) -> Self {
        self.object_schema_id = Some(value.into());

        self
    }

    /// Include all reference types. Defaults to false
    #[must_use]
    pub fn include_all(mut self, value: bool) -> Self {
        self.include_all = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/config/referencetype".to_owned());

        if let Some(value) = &self.object_schema_id {
            config.query.push(("objectSchemaId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_all {
            config.query.push(("includeAll".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ReferenceType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update a reference type
#[derive(Clone)]
pub struct CreateReferenceTypeRequest<'a> {
    client: &'a crate::core::Client,
    reference_type_in: ReferenceTypeIn,
}

impl<'a> CreateReferenceTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, reference_type_in: ReferenceTypeIn) -> Self {
        Self { client, reference_type_in }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/config/referencetype".to_owned());

        let body = match serde_json::to_value(&self.reference_type_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ReferenceType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
