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

    /// Resource to find object schemas in Assets
    pub fn find_schemas(&self) -> FindSchemasRequest<'a> {
        FindSchemasRequest::new(self.client)
    }

    /// Create a new object schema
    pub fn create_schema(&self, object_schema_in: ObjectSchemaIn) -> CreateSchemaRequest<'a> {
        CreateSchemaRequest::new(self.client, object_schema_in)
    }

    /// Find a schema by id
    pub fn load_schema(&self, id: impl Into<String>) -> LoadSchemaRequest<'a> {
        LoadSchemaRequest::new(self.client, id)
    }

    /// Update an object schema
    pub fn update_schema(
        &self,
        id: impl Into<String>,
        object_schema_update: ObjectSchemaUpdate,
    ) -> UpdateSchemaRequest<'a> {
        UpdateSchemaRequest::new(self.client, id, object_schema_update)
    }

    /// Delete a schema
    pub fn delete_schema(&self, id: impl Into<String>) -> DeleteSchemaRequest<'a> {
        DeleteSchemaRequest::new(self.client, id)
    }

    /// Find all object type attributes for this object schema
    pub fn find_schema_attributes(&self, id: impl Into<String>) -> FindSchemaAttributesRequest<'a> {
        FindSchemaAttributesRequest::new(self.client, id)
    }

    /// Find all object types for this object schema
    pub fn find_schema_object_types(&self, id: impl Into<String>) -> FindSchemaObjectTypesRequest<'a> {
        FindSchemaObjectTypesRequest::new(self.client, id)
    }

    /// Find all object types for this object schema
    pub fn find_schema_object_types_flat(&self, id: impl Into<String>) -> FindSchemaObjectTypesFlatRequest<'a> {
        FindSchemaObjectTypesFlatRequest::new(self.client, id)
    }
}

/// Resource to find object schemas in Assets
#[derive(Clone)]
pub struct FindSchemasRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    include_counts: Option<bool>,
}

impl<'a> FindSchemasRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None, include_counts: None }
    }

    /// The starting index for the next page of results
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of objects to return in this page of results. Actual number of results may be less, for example, if the last page of results is returned.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Should the object and object type count for schema be included in the response. If this parameter is false, object and object type count will return 0.
    #[must_use]
    pub fn include_counts(mut self, value: bool) -> Self {
        self.include_counts = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/objectschema/list".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.include_counts {
            config.query.push(("includeCounts".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Create a new object schema
#[derive(Clone)]
pub struct CreateSchemaRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_in: ObjectSchemaIn,
}

impl<'a> CreateSchemaRequest<'a> {
    fn new(client: &'a crate::core::Client, object_schema_in: ObjectSchemaIn) -> Self {
        Self { client, object_schema_in }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/objectschema/create".to_owned());

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

/// Find a schema by id
#[derive(Clone)]
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
            format!("/objectschema/{}", crate::core::encode_path_segment(&self.id)),
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

/// Update an object schema
#[derive(Clone)]
pub struct UpdateSchemaRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    object_schema_update: ObjectSchemaUpdate,
}

impl<'a> UpdateSchemaRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, object_schema_update: ObjectSchemaUpdate) -> Self {
        Self { client, id: id.into(), object_schema_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/objectschema/{}", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.object_schema_update)? {
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

/// Delete a schema
#[derive(Clone)]
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
            format!("/objectschema/{}", crate::core::encode_path_segment(&self.id)),
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

/// Find all object type attributes for this object schema
#[derive(Clone)]
pub struct FindSchemaAttributesRequest<'a> {
    client: &'a crate::core::Client,
    only_value_editable: Option<bool>,
    extended: Option<bool>,
    query: Option<String>,
    id: String,
}

impl<'a> FindSchemaAttributesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), only_value_editable: None, extended: None, query: None }
    }

    /// Return only values that are associated with values that can be edited
    #[must_use]
    pub fn only_value_editable(mut self, value: bool) -> Self {
        self.only_value_editable = Some(value);

        self
    }

    /// Include the object type with each object type attribute
    #[must_use]
    pub fn extended(mut self, value: bool) -> Self {
        self.extended = Some(value);

        self
    }

    /// A query that will be used to filter object type attributes by their name
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/objectschema/{}/attributes", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.only_value_editable {
            config.query.push(("onlyValueEditable".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.extended {
            config.query.push(("extended".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ObjectTypeAttribute>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Find all object types for this object schema
#[derive(Clone)]
pub struct FindSchemaObjectTypesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    exclude_abstract: Option<bool>,
}

impl<'a> FindSchemaObjectTypesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), exclude_abstract: None }
    }

    /// If true, filters out Abstract Object Types from the results
    #[must_use]
    pub fn exclude_abstract(mut self, value: bool) -> Self {
        self.exclude_abstract = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/objectschema/{}/objecttypes", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.exclude_abstract {
            config.query.push(("excludeAbstract".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Find all object types for this object schema
#[derive(Clone)]
pub struct FindSchemaObjectTypesFlatRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    query: Option<bool>,
    exclude: Option<String>,
    include_object_counts: Option<bool>,
}

impl<'a> FindSchemaObjectTypesFlatRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), query: None, exclude: None, include_object_counts: None }
    }

    /// Object Type Names to search for
    #[must_use]
    pub fn query(mut self, value: bool) -> Self {
        self.query = Some(value);

        self
    }

    /// Exclude objects with this name
    #[must_use]
    pub fn exclude(mut self, value: impl Into<String>) -> Self {
        self.exclude = Some(value.into());

        self
    }

    /// If true, the objectCount attribute is populated for each object type
    #[must_use]
    pub fn include_object_counts(mut self, value: bool) -> Self {
        self.include_object_counts = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/objectschema/{}/objecttypes/flat", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.exclude {
            config.query.push(("exclude".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_object_counts {
            config.query.push(("includeObjectCounts".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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
