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

    /// Retrieve archived objects.
    pub fn get_archived_objects(&self) -> GetArchivedObjectsRequest<'a> {
        GetArchivedObjectsRequest::new(self.client)
    }

    /// Archive the referenced object.
    pub fn archive_object(&self, object_identifier: impl Into<String>) -> ArchiveObjectRequest<'a> {
        ArchiveObjectRequest::new(self.client, object_identifier)
    }

    /// Bulk archive objects of an object type by filter.
    pub fn archive_objects_by_filter(&self) -> ArchiveObjectsByFilterRequest<'a> {
        ArchiveObjectsByFilterRequest::new(self.client)
    }

    /// Bulk archive objects across object schemas by object keys asynchronously.
    pub fn archive_objects_by_keys(&self) -> ArchiveObjectsByKeysRequest<'a> {
        ArchiveObjectsByKeysRequest::new(self.client)
    }

    /// Restore the referenced object.
    pub fn restore_object(&self, object_identifier: impl Into<String>) -> RestoreObjectRequest<'a> {
        RestoreObjectRequest::new(self.client, object_identifier)
    }

    /// Bulk restore objects by archive search filter.
    pub fn restore_objects_by_filter(&self) -> RestoreObjectsByFilterRequest<'a> {
        RestoreObjectsByFilterRequest::new(self.client)
    }

    /// Bulk restore objects by object ids.
    pub fn restore_objects_by_ids(&self) -> RestoreObjectsByIdsRequest<'a> {
        RestoreObjectsByIdsRequest::new(self.client)
    }

    /// Bulk restore objects across object schemas by object keys asynchronously.
    pub fn restore_objects_by_keys(&self) -> RestoreObjectsByKeysRequest<'a> {
        RestoreObjectsByKeysRequest::new(self.client)
    }

    /// Set the import source for all objects matching the given IQL criteria.
    pub fn bulk_set_object_import_source(&self) -> BulkSetObjectImportSourceRequest<'a> {
        BulkSetObjectImportSourceRequest::new(self.client)
    }

    /// Get the current import source ID for an object.
    pub fn get_object_import_source(&self, id: impl Into<String>) -> GetObjectImportSourceRequest<'a> {
        GetObjectImportSourceRequest::new(self.client, id)
    }

    /// Clear the import source association for an object.
    pub fn clear_object_import_source(&self, id: impl Into<String>) -> ClearObjectImportSourceRequest<'a> {
        ClearObjectImportSourceRequest::new(self.client, id)
    }

    /// Create a new object in Assets.
    pub fn create_object(&self) -> CreateObjectRequest<'a> {
        CreateObjectRequest::new(self.client)
    }

    /// Get an object.
    pub fn load_object(&self, id: impl Into<String>) -> LoadObjectRequest<'a> {
        LoadObjectRequest::new(self.client, id)
    }

    /// Update an existing object in Assets.
    pub fn update_object(&self, id: impl Into<String>) -> UpdateObjectRequest<'a> {
        UpdateObjectRequest::new(self.client, id)
    }

    /// Delete the referenced object.
    pub fn delete_object(&self, id: impl Into<String>) -> DeleteObjectRequest<'a> {
        DeleteObjectRequest::new(self.client, id)
    }

    /// Retrieve a list of objects based on an AQL query.
    pub fn find_object(&self) -> FindObjectRequest<'a> {
        FindObjectRequest::new(self.client)
    }

    /// Get all attributes for the given object.
    pub fn find_object_attributes(&self, id: impl Into<String>) -> FindObjectAttributesRequest<'a> {
        FindObjectAttributesRequest::new(self.client, id)
    }

    /// Retrieve the history entries for this object.
    pub fn find_object_history(&self, id: impl Into<String>) -> FindObjectHistoryRequest<'a> {
        FindObjectHistoryRequest::new(self.client, id)
    }

    /// Find all inbound references for an object.
    pub fn find_object_reference_info(&self, id: impl Into<String>) -> FindObjectReferenceInfoRequest<'a> {
        FindObjectReferenceInfoRequest::new(self.client, id)
    }
}

/// Retrieve archived objects.
pub struct GetArchivedObjectsRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_id: Option<String>,
    archived_from_date: Option<String>,
    offset: Option<String>,
    object_type_ids: Option<String>,
    limit: Option<String>,
    archived_to_date: Option<String>,
    archived_by: Option<String>,
}

impl<'a> GetArchivedObjectsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            object_schema_id: None,
            archived_from_date: None,
            offset: None,
            object_type_ids: None,
            limit: None,
            archived_to_date: None,
            archived_by: None,
        }
    }

    /// The id of the object schema to search for.
    #[must_use]
    pub fn object_schema_id(mut self, value: impl Into<String>) -> Self {
        self.object_schema_id = Some(value.into());

        self
    }

    /// Timestamp in ISO Offset Date Time format e.g. 2021-12-03T10:15:30+01:00. Only objects archived at or after this time will be displayed. Optional.
    #[must_use]
    pub fn archived_from_date(mut self, value: impl Into<String>) -> Self {
        self.archived_from_date = Some(value.into());

        self
    }

    /// The offset of the first object to return. Optional.
    #[must_use]
    pub fn offset(mut self, value: impl Into<String>) -> Self {
        self.offset = Some(value.into());

        self
    }

    /// A list of object type ids to search for. Optional. If not set, all object types within the schema will be searched for.
    #[must_use]
    pub fn object_type_ids(mut self, value: impl Into<String>) -> Self {
        self.object_type_ids = Some(value.into());

        self
    }

    /// The maximum number of objects to return. Optional.
    #[must_use]
    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());

        self
    }

    /// Timestamp in ISO  Offset Date Time format e.g. 2021-12-03T10:15:30+01:00. Only objects archived before this time will be displayed. Must be after archivedFromDate, if both are set. Optional.
    #[must_use]
    pub fn archived_to_date(mut self, value: impl Into<String>) -> Self {
        self.archived_to_date = Some(value.into());

        self
    }

    /// A list of users that archived the objects. Optional. If not set, all users will be searched for.
    #[must_use]
    pub fn archived_by(mut self, value: impl Into<String>) -> Self {
        self.archived_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/assets/1.0/object/archived".to_owned());

        if let Some(value) = &self.object_schema_id {
            config.query.push(("objectSchemaId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.archived_from_date {
            config.query.push(("archivedFromDate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.offset {
            config.query.push(("offset".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.object_type_ids {
            config.query.push(("objectTypeIds".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.archived_to_date {
            config.query.push(("archivedToDate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.archived_by {
            config.query.push(("archivedBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ArchivedObjectsPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Archive the referenced object.
pub struct ArchiveObjectRequest<'a> {
    client: &'a crate::core::Client,
    object_identifier: String,
}

impl<'a> ArchiveObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, object_identifier: impl Into<String>) -> Self {
        Self { client, object_identifier: object_identifier.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/assets/1.0/object/archive/{}", crate::core::encode_path_segment(&self.object_identifier)),
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

/// Bulk archive objects of an object type by filter.
pub struct ArchiveObjectsByFilterRequest<'a> {
    client: &'a crate::core::Client,
    type_id: Option<String>,
    object_filters: Option<ObjectFilters>,
}

impl<'a> ArchiveObjectsByFilterRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, type_id: None, object_filters: None }
    }

    /// The object type id to archive.
    #[must_use]
    pub fn type_id(mut self, value: impl Into<String>) -> Self {
        self.type_id = Some(value.into());

        self
    }

    #[must_use]
    pub fn object_filters(mut self, value: ObjectFilters) -> Self {
        self.object_filters = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/archive/by-filter".to_owned(),
        );

        if let Some(value) = &self.type_id {
            config.query.push(("typeId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.object_filters)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Bulk archive objects across object schemas by object keys asynchronously.
pub struct ArchiveObjectsByKeysRequest<'a> {
    client: &'a crate::core::Client,
    body: Option<Vec<String>>,
}

impl<'a> ArchiveObjectsByKeysRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, body: None }
    }

    /// The keys of the objects to archive.
    #[must_use]
    pub fn body(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/archive/by-keys".to_owned(),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Restore the referenced object.
pub struct RestoreObjectRequest<'a> {
    client: &'a crate::core::Client,
    object_identifier: String,
}

impl<'a> RestoreObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, object_identifier: impl Into<String>) -> Self {
        Self { client, object_identifier: object_identifier.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/assets/1.0/object/restore/{}", crate::core::encode_path_segment(&self.object_identifier)),
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

/// Bulk restore objects by archive search filter.
pub struct RestoreObjectsByFilterRequest<'a> {
    client: &'a crate::core::Client,
    object_schema_id: Option<String>,
    archived_from_date: Option<String>,
    object_type_ids: Option<String>,
    archived_to_date: Option<String>,
    archived_by: Option<String>,
}

impl<'a> RestoreObjectsByFilterRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            object_schema_id: None,
            archived_from_date: None,
            object_type_ids: None,
            archived_to_date: None,
            archived_by: None,
        }
    }

    /// The id of the object schema to search for.
    #[must_use]
    pub fn object_schema_id(mut self, value: impl Into<String>) -> Self {
        self.object_schema_id = Some(value.into());

        self
    }

    /// Timestamp in ISO Offset Date Time format e.g. 2021-12-03T10:15:30+01:00. Only objects archived at or after this time will be displayed. Optional.
    #[must_use]
    pub fn archived_from_date(mut self, value: impl Into<String>) -> Self {
        self.archived_from_date = Some(value.into());

        self
    }

    /// A list of object type ids to search for. Optional. If not set, all object types within the schema will be searched for.
    #[must_use]
    pub fn object_type_ids(mut self, value: impl Into<String>) -> Self {
        self.object_type_ids = Some(value.into());

        self
    }

    /// Timestamp in ISO  Offset Date Time format e.g. 2021-12-03T10:15:30+01:00. Only objects archived before this time will be displayed. Must be after archivedFromDate, if both are set. Optional.
    #[must_use]
    pub fn archived_to_date(mut self, value: impl Into<String>) -> Self {
        self.archived_to_date = Some(value.into());

        self
    }

    /// A list of users that archived the objects. Optional. If not set, all users will be searched for.
    #[must_use]
    pub fn archived_by(mut self, value: impl Into<String>) -> Self {
        self.archived_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/restore/by-filter".to_owned(),
        );

        if let Some(value) = &self.object_schema_id {
            config.query.push(("objectSchemaId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.archived_from_date {
            config.query.push(("archivedFromDate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.object_type_ids {
            config.query.push(("objectTypeIds".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.archived_to_date {
            config.query.push(("archivedToDate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.archived_by {
            config.query.push(("archivedBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Bulk restore objects by object ids.
pub struct RestoreObjectsByIdsRequest<'a> {
    client: &'a crate::core::Client,
    body: Option<Vec<i64>>,
}

impl<'a> RestoreObjectsByIdsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, body: None }
    }

    #[must_use]
    pub fn body(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.body = Some(value.into_iter().collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/restore/by-ids".to_owned(),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Bulk restore objects across object schemas by object keys asynchronously.
pub struct RestoreObjectsByKeysRequest<'a> {
    client: &'a crate::core::Client,
    body: Option<Vec<String>>,
}

impl<'a> RestoreObjectsByKeysRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, body: None }
    }

    #[must_use]
    pub fn body(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/restore/by-keys".to_owned(),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Set the import source for all objects matching the given IQL criteria.
pub struct BulkSetObjectImportSourceRequest<'a> {
    client: &'a crate::core::Client,
    body: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl<'a> BulkSetObjectImportSourceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, body: None }
    }

    #[must_use]
    pub fn body(mut self, value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/bulk/importSource".to_owned(),
        );

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

/// Get the current import source ID for an object.
pub struct GetObjectImportSourceRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetObjectImportSourceRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/object/{}/importSource", crate::core::encode_path_segment(&self.id)),
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

/// Clear the import source association for an object.
pub struct ClearObjectImportSourceRequest<'a> {
    client: &'a crate::core::Client,
    previous_name: Option<String>,
    id: String,
}

impl<'a> ClearObjectImportSourceRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), previous_name: None }
    }

    #[must_use]
    pub fn previous_name(mut self, value: impl Into<String>) -> Self {
        self.previous_name = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/assets/1.0/object/{}/importSource", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.previous_name {
            config.query.push(("previousName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Create a new object in Assets.
pub struct CreateObjectRequest<'a> {
    client: &'a crate::core::Client,
    asset_object_in: Option<AssetObjectIn>,
}

impl<'a> CreateObjectRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, asset_object_in: None }
    }

    #[must_use]
    pub fn asset_object_in(mut self, value: AssetObjectIn) -> Self {
        self.asset_object_in = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/assets/1.0/object/create".to_owned());

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

/// Get an object.
pub struct LoadObjectRequest<'a> {
    client: &'a crate::core::Client,
    xoauth_requestor_id: Option<String>,
    include_attributes: Option<String>,
    id: String,
    include_extended_info: Option<String>,
}

impl<'a> LoadObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), xoauth_requestor_id: None, include_attributes: None, include_extended_info: None }
    }

    #[must_use]
    pub fn xoauth_requestor_id(mut self, value: impl Into<String>) -> Self {
        self.xoauth_requestor_id = Some(value.into());

        self
    }

    /// Should the attributes be included in the response.
    #[must_use]
    pub fn include_attributes(mut self, value: impl Into<String>) -> Self {
        self.include_attributes = Some(value.into());

        self
    }

    /// Should the extended info be included in the response.
    #[must_use]
    pub fn include_extended_info(mut self, value: impl Into<String>) -> Self {
        self.include_extended_info = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/object/{}", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.xoauth_requestor_id {
            config.query.push(("xoauth_requestor_id".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_attributes {
            config.query.push(("includeAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_extended_info {
            config.query.push(("includeExtendedInfo".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Update an existing object in Assets.
pub struct UpdateObjectRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    asset_object_in: Option<AssetObjectIn>,
}

impl<'a> UpdateObjectRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), asset_object_in: None }
    }

    #[must_use]
    pub fn asset_object_in(mut self, value: AssetObjectIn) -> Self {
        self.asset_object_in = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/assets/1.0/object/{}", crate::core::encode_path_segment(&self.id)),
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

/// Delete the referenced object.
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
            format!("/rest/assets/1.0/object/{}", crate::core::encode_path_segment(&self.id)),
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

/// Retrieve a list of objects based on an AQL query.
pub struct FindObjectRequest<'a> {
    client: &'a crate::core::Client,
    object_iql_filter_param: Option<ObjectIQLFilterParam>,
}

impl<'a> FindObjectRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, object_iql_filter_param: None }
    }

    #[must_use]
    pub fn object_iql_filter_param(mut self, value: ObjectIQLFilterParam) -> Self {
        self.object_iql_filter_param = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/object/navlist/aql".to_owned(),
        );

        let body = match serde_json::to_value(&self.object_iql_filter_param)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectListResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get all attributes for the given object.
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
            format!("/rest/assets/1.0/object/{}/attributes", crate::core::encode_path_segment(&self.id)),
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

/// Retrieve the history entries for this object.
pub struct FindObjectHistoryRequest<'a> {
    client: &'a crate::core::Client,
    asc: Option<bool>,
    abbreviate: Option<String>,
    order_asc: Option<String>,
    id: String,
}

impl<'a> FindObjectHistoryRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), asc: None, abbreviate: None, order_asc: None }
    }

    #[must_use]
    pub fn asc(mut self, value: bool) -> Self {
        self.asc = Some(value);

        self
    }

    /// Should the values returned in the history entry be abbreviated.
    #[must_use]
    pub fn abbreviate(mut self, value: impl Into<String>) -> Self {
        self.abbreviate = Some(value.into());

        self
    }

    /// Should the history be retrieved in ascending order. Uses the Jira setting for sort order as its default value.
    #[must_use]
    pub fn order_asc(mut self, value: impl Into<String>) -> Self {
        self.order_asc = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/object/{}/history", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.asc {
            config.query.push(("asc".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.abbreviate {
            config.query.push(("abbreviate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.order_asc {
            config.query.push(("orderAsc".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Find all inbound references for an object.
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
            format!("/rest/assets/1.0/object/{}/referenceinfo", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ReferenceTypeObjectInfo>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
