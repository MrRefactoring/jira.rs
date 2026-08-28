// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Objects operations.
pub struct ObjectsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ObjectsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Load one object
    pub fn load_object(&self, id: impl Into<String>) -> LoadObjectRequest<'a> {
        LoadObjectRequest::new(self.client, id)
    }

    /// Update an existing object in Assets
    pub fn update_object(&self, id: impl Into<String>, asset_object_in: AssetObjectIn) -> UpdateObjectRequest<'a> {
        UpdateObjectRequest::new(self.client, id, asset_object_in)
    }

    /// Delete the referenced object
    pub fn delete_object(&self, id: impl Into<String>) -> DeleteObjectRequest<'a> {
        DeleteObjectRequest::new(self.client, id)
    }

    /// List all attributes for the given object
    pub fn find_object_attributes(&self, id: impl Into<String>) -> FindObjectAttributesRequest<'a> {
        FindObjectAttributesRequest::new(self.client, id)
    }

    /// Retrieve the history entries for this object
    pub fn find_object_history(&self, id: impl Into<String>) -> FindObjectHistoryRequest<'a> {
        FindObjectHistoryRequest::new(self.client, id)
    }

    /// Find all references for an object
    pub fn find_object_reference_info(&self, id: impl Into<String>) -> FindObjectReferenceInfoRequest<'a> {
        FindObjectReferenceInfoRequest::new(self.client, id)
    }

    /// Create a new object in Assets
    pub fn create_object(&self, asset_object_in: AssetObjectIn) -> CreateObjectRequest<'a> {
        CreateObjectRequest::new(self.client, asset_object_in)
    }

    /// Fetch Objects by AQL
    pub fn find_objects_by_aql(&self, object_aql_params: ObjectAQLParams) -> FindObjectsByAqlRequest<'a> {
        FindObjectsByAqlRequest::new(self.client, object_aql_params)
    }

    /// This API provides the total count of objects that match a specified AQL query. Please note that this operation may incur performance latency.
    pub fn count_objects_by_aql(
        &self,
        object_aql_total_count_params: ObjectAQLTotalCountParams,
    ) -> CountObjectsByAqlRequest<'a> {
        CountObjectsByAqlRequest::new(self.client, object_aql_total_count_params)
    }
}

/// Load one object
#[derive(Clone)]
pub struct LoadObjectRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> LoadObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/object/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AssetObject> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an existing object in Assets
#[derive(Clone)]
pub struct UpdateObjectRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    asset_object_in: AssetObjectIn,
}

impl<'a> UpdateObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, asset_object_in: AssetObjectIn) -> Self {
        Self { client, id: id.into(), asset_object_in }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/object/{}", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.asset_object_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AssetObject> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the referenced object
#[derive(Clone)]
pub struct DeleteObjectRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/object/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// List all attributes for the given object
#[derive(Clone)]
pub struct FindObjectAttributesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> FindObjectAttributesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/object/{}/attributes", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ObjectAttribute>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Retrieve the history entries for this object
#[derive(Clone)]
pub struct FindObjectHistoryRequest<'a> {
    client: &'a crate::core::Client,
    asc: Option<bool>,
    id: String,
}

impl<'a> FindObjectHistoryRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), asc: None }
    }

    /// Should the history be retrieved in ascending order
    #[must_use]
    pub fn asc(mut self, value: bool) -> Self {
        self.asc = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/object/{}/history", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.asc {
            config.query.push(("asc".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ObjectHistory>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Find all references for an object
#[derive(Clone)]
pub struct FindObjectReferenceInfoRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> FindObjectReferenceInfoRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/object/{}/referenceinfo", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ObjectReferenceTypeInfo>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a new object in Assets
#[derive(Clone)]
pub struct CreateObjectRequest<'a> {
    client: &'a crate::core::Client,
    asset_object_in: AssetObjectIn,
}

impl<'a> CreateObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, asset_object_in: AssetObjectIn) -> Self {
        Self { client, asset_object_in }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/object/create".to_owned());

        let body = match serde_json::to_value(&self.asset_object_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AssetObject> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Fetch Objects by AQL
#[derive(Clone)]
pub struct FindObjectsByAqlRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    include_attributes: Option<bool>,
    object_aql_params: ObjectAQLParams,
}

impl<'a> FindObjectsByAqlRequest<'a> {
    fn new(client: &'a crate::core::Client, object_aql_params: ObjectAQLParams) -> Self {
        Self { client, object_aql_params, start_at: None, max_results: None, include_attributes: None }
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

    /// Should the objects attributes be included in the response. If this parameter is false only the information on the object will be returned and the object attributes will not be present
    #[must_use]
    pub fn include_attributes(mut self, value: bool) -> Self {
        self.include_attributes = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/object/aql".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.include_attributes {
            config.query.push(("includeAttributes".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.object_aql_params)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectListInclTypeAttributesEntryResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// This API provides the total count of objects that match a specified AQL query. Please note that this operation may incur performance latency.
#[derive(Clone)]
pub struct CountObjectsByAqlRequest<'a> {
    client: &'a crate::core::Client,
    object_aql_total_count_params: ObjectAQLTotalCountParams,
}

impl<'a> CountObjectsByAqlRequest<'a> {
    fn new(client: &'a crate::core::Client, object_aql_total_count_params: ObjectAQLTotalCountParams) -> Self {
        Self { client, object_aql_total_count_params }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/object/aql/totalcount".to_owned());

        let body = match serde_json::to_value(&self.object_aql_total_count_params)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectAQLTotalCountResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
