// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ObjectTypes operations.
pub struct ObjectTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ObjectTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Change the position of an object type in the object type hierarchy tree.
    pub fn change_order_object_type(&self, affected_id: impl Into<String>) -> ChangeOrderObjectTypeRequest<'a> {
        ChangeOrderObjectTypeRequest::new(self.client, affected_id)
    }

    /// Create a new object type.
    pub fn create_object_type(&self) -> CreateObjectTypeRequest<'a> {
        CreateObjectTypeRequest::new(self.client)
    }

    /// Get a single object type.
    pub fn load_object_type(&self, id: impl Into<String>) -> LoadObjectTypeRequest<'a> {
        LoadObjectTypeRequest::new(self.client, id)
    }

    /// Update an existing object type.
    pub fn update_object_type(&self, id: impl Into<String>) -> UpdateObjectTypeRequest<'a> {
        UpdateObjectTypeRequest::new(self.client, id)
    }

    /// Delete an object type.
    pub fn delete_object_type(&self, id: impl Into<String>) -> DeleteObjectTypeRequest<'a> {
        DeleteObjectTypeRequest::new(self.client, id)
    }

    /// Find all object type attributes for this object type.
    pub fn find_object_type_attributes(&self, id: impl Into<String>) -> FindObjectTypeAttributesRequest<'a> {
        FindObjectTypeAttributesRequest::new(self.client, id)
    }
}

/// Change the position of an object type in the object type hierarchy tree.
pub struct ChangeOrderObjectTypeRequest<'a> {
    client: &'a crate::core::Client,
    affected_id: String,
    js_tree_position: Option<JSTreePosition>,
}

impl<'a> ChangeOrderObjectTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, affected_id: impl Into<String>) -> Self {
        Self { client, affected_id: affected_id.into(), js_tree_position: None }
    }

    #[must_use]
    pub fn js_tree_position(mut self, value: JSTreePosition) -> Self {
        self.js_tree_position = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/assets/1.0/objecttype/{}/position", crate::core::encode_path_segment(&self.affected_id)),
        );

        let body = match serde_json::to_value(&self.js_tree_position)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a new object type.
pub struct CreateObjectTypeRequest<'a> {
    client: &'a crate::core::Client,
    object_type_in: Option<ObjectTypeIn>,
}

impl<'a> CreateObjectTypeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, object_type_in: None }
    }

    #[must_use]
    pub fn object_type_in(mut self, value: ObjectTypeIn) -> Self {
        self.object_type_in = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/assets/1.0/objecttype/create".to_owned());

        let body = match serde_json::to_value(&self.object_type_in)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get a single object type.
pub struct LoadObjectTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> LoadObjectTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/objecttype/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an existing object type.
pub struct UpdateObjectTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    body: Option<ObjectTypeIn>,
}

impl<'a> UpdateObjectTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), body: None }
    }

    #[must_use]
    pub fn body(mut self, value: ObjectTypeIn) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/assets/1.0/objecttype/{}", crate::core::encode_path_segment(&self.id)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete an object type.
pub struct DeleteObjectTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteObjectTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/assets/1.0/objecttype/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ObjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Find all object type attributes for this object type.
pub struct FindObjectTypeAttributesRequest<'a> {
    client: &'a crate::core::Client,
    order_by_required: Option<String>,
    order_by_name: Option<String>,
    include_children: Option<String>,
    query: Option<String>,
    exclude_parent_attributes: Option<String>,
    include_value_exist: Option<String>,
    id: String,
    only_value_editable: Option<String>,
}

impl<'a> FindObjectTypeAttributesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self {
            client,
            id: id.into(),
            order_by_required: None,
            order_by_name: None,
            include_children: None,
            query: None,
            exclude_parent_attributes: None,
            include_value_exist: None,
            only_value_editable: None,
        }
    }

    /// Should the response be ordered by the number of required attributes.
    #[must_use]
    pub fn order_by_required(mut self, value: impl Into<String>) -> Self {
        self.order_by_required = Some(value.into());

        self
    }

    /// Should the response be ordered by name.
    #[must_use]
    pub fn order_by_name(mut self, value: impl Into<String>) -> Self {
        self.order_by_name = Some(value.into());

        self
    }

    /// Should the response include child attributes.
    #[must_use]
    pub fn include_children(mut self, value: impl Into<String>) -> Self {
        self.include_children = Some(value.into());

        self
    }

    /// Filter attributes that start with the query.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// Should the response exclude parent attributes.
    #[must_use]
    pub fn exclude_parent_attributes(mut self, value: impl Into<String>) -> Self {
        self.exclude_parent_attributes = Some(value.into());

        self
    }

    /// Should the response only include attributes where attribute values exists.
    #[must_use]
    pub fn include_value_exist(mut self, value: impl Into<String>) -> Self {
        self.include_value_exist = Some(value.into());

        self
    }

    /// Should the response only include attributes where only the value is editable.
    #[must_use]
    pub fn only_value_editable(mut self, value: impl Into<String>) -> Self {
        self.only_value_editable = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/objecttype/{}/attributes", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.order_by_required {
            config.query.push(("orderByRequired".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.order_by_name {
            config.query.push(("orderByName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_children {
            config.query.push(("includeChildren".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.exclude_parent_attributes {
            config.query.push(("excludeParentAttributes".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_value_exist {
            config.query.push(("includeValueExist".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.only_value_editable {
            config.query.push(("onlyValueEditable".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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
