// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ObjectTypeAttributes operations.
pub struct ObjectTypeAttributesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ObjectTypeAttributesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Create a new attribute on the given object type
    pub fn create_object_type_attribute(
        &self,
        object_type_id: impl Into<String>,
        object_type_attribute_create: ObjectTypeAttributeCreate,
    ) -> CreateObjectTypeAttributeRequest<'a> {
        CreateObjectTypeAttributeRequest::new(self.client, object_type_id, object_type_attribute_create)
    }

    /// Update an existing object type attribute
    pub fn update_object_type_attribute(
        &self,
        id: impl Into<String>,
        object_type_id: impl Into<String>,
        object_type_attribute_update: ObjectTypeAttributeUpdate,
    ) -> UpdateObjectTypeAttributeRequest<'a> {
        UpdateObjectTypeAttributeRequest::new(self.client, id, object_type_id, object_type_attribute_update)
    }

    /// Delete an existing object type attribute
    pub fn delete_object_type_attribute(&self, id: impl Into<String>) -> DeleteObjectTypeAttributeRequest<'a> {
        DeleteObjectTypeAttributeRequest::new(self.client, id)
    }
}

/// Create a new attribute on the given object type
pub struct CreateObjectTypeAttributeRequest<'a> {
    client: &'a crate::core::Client,
    object_type_id: String,
    object_type_attribute_create: ObjectTypeAttributeCreate,
}

impl<'a> CreateObjectTypeAttributeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        object_type_id: impl Into<String>,
        object_type_attribute_create: ObjectTypeAttributeCreate,
    ) -> Self {
        Self { client, object_type_id: object_type_id.into(), object_type_attribute_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/objecttypeattribute/{}", crate::core::encode_path_segment(&self.object_type_id)),
        );

        let body = match serde_json::to_value(&self.object_type_attribute_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectTypeAttribute> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an existing object type attribute
pub struct UpdateObjectTypeAttributeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    object_type_id: String,
    object_type_attribute_update: ObjectTypeAttributeUpdate,
}

impl<'a> UpdateObjectTypeAttributeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        id: impl Into<String>,
        object_type_id: impl Into<String>,
        object_type_attribute_update: ObjectTypeAttributeUpdate,
    ) -> Self {
        Self { client, id: id.into(), object_type_id: object_type_id.into(), object_type_attribute_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/objecttypeattribute/{}/{}",
                crate::core::encode_path_segment(&self.object_type_id),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        let body = match serde_json::to_value(&self.object_type_attribute_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectTypeAttribute> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete an existing object type attribute
pub struct DeleteObjectTypeAttributeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteObjectTypeAttributeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/objecttypeattribute/{}", crate::core::encode_path_segment(&self.id)),
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
