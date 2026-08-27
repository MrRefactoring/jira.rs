// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ObjectSchemas operations.
pub struct ObjectSchemasService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ObjectSchemasService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Create a new object schema.
    pub fn create_schema(&self) -> CreateSchemaRequest<'a> {
        CreateSchemaRequest::new(self.client)
    }

    /// Get a single object schema.
    pub fn load_schema(&self, id: impl Into<String>) -> LoadSchemaRequest<'a> {
        LoadSchemaRequest::new(self.client, id)
    }

    /// Update an object schema.
    pub fn update_schema(&self, id: impl Into<String>) -> UpdateSchemaRequest<'a> {
        UpdateSchemaRequest::new(self.client, id)
    }

    /// Deletes a single object schema.
    pub fn delete_schema(&self, id: impl Into<String>) -> DeleteSchemaRequest<'a> {
        DeleteSchemaRequest::new(self.client, id)
    }

    /// Searches for an object schema by name.
    pub fn find_schemas(&self) -> FindSchemasRequest<'a> {
        FindSchemasRequest::new(self.client)
    }

    /// Get a flat list of all object types belonging to a certain object schema.
    pub fn find_object_type_flat_list(&self, id: impl Into<String>) -> FindObjectTypeFlatListRequest<'a> {
        FindObjectTypeFlatListRequest::new(self.client, id)
    }
}

/// Create a new object schema.
pub struct CreateSchemaRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_in: Option<ObjectSchemaIn>,
}

impl<'a> CreateSchemaRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, object_schema_in: None }
    }

    #[must_use]
    pub fn object_schema_in(mut self, value: ObjectSchemaIn) -> Self {
        self.object_schema_in = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/objectschema/create".to_owned(),
        );

        let body = match serde_json::to_value(&self.object_schema_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectSchema> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get a single object schema.
pub struct LoadSchemaRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> LoadSchemaRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/objectschema/{}", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectSchema> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an object schema.
pub struct UpdateSchemaRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    body: Option<ObjectSchema>,
}

impl<'a> UpdateSchemaRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), body: None }
    }

    #[must_use]
    pub fn body(mut self, value: ObjectSchema) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/assets/1.0/objectschema/{}", self.id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectSchema> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a single object schema.
pub struct DeleteSchemaRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteSchemaRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/assets/1.0/objectschema/{}", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectSchema> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Searches for an object schema by name.
pub struct FindSchemasRequest<'a> {
    client: &'a crate::core::Client,
    xoauth_requestor_id: Option<String>,
    query: Option<String>,
}

impl<'a> FindSchemasRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, xoauth_requestor_id: None, query: None }
    }

    #[must_use]
    pub fn xoauth_requestor_id(mut self, value: impl Into<String>) -> Self {
        self.xoauth_requestor_id = Some(value.into());

        self
    }

    /// Query to filter on available object schemas
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/assets/1.0/objectschema/list".to_owned());

        if let Some(value) = &self.xoauth_requestor_id {
            config.query.push(("xoauth_requestor_id".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectSchemaList> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get a flat list of all object types belonging to a certain object schema.
pub struct FindObjectTypeFlatListRequest<'a> {
    client: &'a crate::core::Client,
    role: Option<String>,
    query: Option<String>,
    exclude: Option<String>,
    id: String,
}

impl<'a> FindObjectTypeFlatListRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), role: None, query: None, exclude: None }
    }

    /// The Object Type role to filter the list of object types. Valid values: `OBJECT_TYPE_USER`, `OBJECT_TYPE_DEVELOPER`, `OBJECT_TYPE_MANAGER`
    #[must_use]
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());

        self
    }

    /// Query to filter on available object types
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// Exclude object types with this name
    #[must_use]
    pub fn exclude(mut self, value: impl Into<String>) -> Self {
        self.exclude = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/objectschema/{}/objecttypes/flat", self.id),
        );

        if let Some(value) = &self.role {
            config.query.push(("role".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.exclude {
            config.query.push(("exclude".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ObjectType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
