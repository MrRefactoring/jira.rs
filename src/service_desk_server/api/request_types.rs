// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The RequestTypes operations.
pub struct RequestTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> RequestTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the fields for a request type, for a given request type Id and service project Id. These are the fields that are required to create a customer request of that particular request type.
    ///
    /// In addition, the following information about the current user's permissions for the request type is returned:
    /// * `canRaiseOnBehalfOf` field - Returns true, if the user has permission to raise requests on behalf of customers. Otherwise, returns false.
    /// * `canAddRequestParticipants` field - Returns true, if the user can add request participants. Otherwise, returns false.
    pub fn get_request_type_fields(
        &self,
        service_desk_id: impl Into<String>,
        request_type_id: impl Into<String>,
    ) -> GetRequestTypeFieldsRequest<'a> {
        GetRequestTypeFieldsRequest::new(self.client, service_desk_id, request_type_id)
    }

    /// Returns all request type groups from a service project, for a given service project Id. The groups will be in the same order as the order in which they appear on the customer portal
    pub fn get_request_type_groups(&self, service_desk_id: impl Into<String>) -> GetRequestTypeGroupsRequest<'a> {
        GetRequestTypeGroupsRequest::new(self.client, service_desk_id)
    }

    /// Returns all request types from a service project, for a given service project Id.
    pub fn get_request_types(&self, service_desk_id: impl Into<String>) -> GetRequestTypesRequest<'a> {
        GetRequestTypesRequest::new(self.client, service_desk_id)
    }

    /// Creates a new request type for a given service project. Certain fields cannot be specified on creation. These fields are given default values instead, as shown below.
    ///
    /// Request Type icon - Question mark icon
    ///
    /// Request Type groups - Empty, i.e. this request type will be hidden by default and not visible on the customer portal
    ///
    /// Request Type status mapping - Empty, i.e. no custom status mapping
    ///
    /// Request Type field mapping - Show the required fields as specified by the issue type
    ///
    /// These fields can be updated after creation by a project administrator using the Agent view.
    ///
    ///
    ///
    ///  **Permissions:**
    ///
    /// The calling user must be a project administrator for the service project project.
    pub fn create_request_type(&self, service_desk_id: impl Into<String>) -> CreateRequestTypeRequest<'a> {
        CreateRequestTypeRequest::new(self.client, service_desk_id)
    }

    /// Updates a request type for a given service project. Note Issue Type cannot be changed.
    ///
    ///
    ///
    ///  **Permissions:**
    ///
    /// The calling user must be a project administrator for the service project project.
    pub fn update_request_type(&self, service_desk_id: impl Into<String>) -> UpdateRequestTypeRequest<'a> {
        UpdateRequestTypeRequest::new(self.client, service_desk_id)
    }

    /// Returns a request type for a given request type Id.
    pub fn get_request_type_by_id(
        &self,
        service_desk_id: impl Into<String>,
        request_type_id: impl Into<String>,
    ) -> GetRequestTypeByIdRequest<'a> {
        GetRequestTypeByIdRequest::new(self.client, service_desk_id, request_type_id)
    }

    /// Deletes a request type for a given service project.
    ///
    ///
    ///
    ///  **Permissions:**
    ///
    /// The calling user must be a project administrator for the service project project.
    pub fn delete_request_type(
        &self,
        service_desk_id: impl Into<String>,
        request_type_id: impl Into<String>,
    ) -> DeleteRequestTypeRequest<'a> {
        DeleteRequestTypeRequest::new(self.client, service_desk_id, request_type_id)
    }
}

/// Returns the fields for a request type, for a given request type Id and service project Id. These are the fields that are required to create a customer request of that particular request type.
///
/// In addition, the following information about the current user's permissions for the request type is returned:
/// * `canRaiseOnBehalfOf` field - Returns true, if the user has permission to raise requests on behalf of customers. Otherwise, returns false.
/// * `canAddRequestParticipants` field - Returns true, if the user can add request participants. Otherwise, returns false.
#[derive(Clone)]
pub struct GetRequestTypeFieldsRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    request_type_id: String,
}

impl<'a> GetRequestTypeFieldsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        service_desk_id: impl Into<String>,
        request_type_id: impl Into<String>,
    ) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), request_type_id: request_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttype/{}/field",
                crate::core::encode_path_segment(&self.service_desk_id),
                crate::core::encode_path_segment(&self.request_type_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CustomerRequestCreateMeta> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all request type groups from a service project, for a given service project Id. The groups will be in the same order as the order in which they appear on the customer portal
#[derive(Clone)]
pub struct GetRequestTypeGroupsRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    start: Option<f64>,
    limit: Option<f64>,
}

impl<'a> GetRequestTypeGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0.
    #[must_use]
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttypegroup",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<RequestTypeGroup>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all request types from a service project, for a given service project Id.
#[derive(Clone)]
pub struct GetRequestTypesRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    group_id: Option<String>,
    restriction_status: Option<String>,
    start: Option<f64>,
    limit: Option<f64>,
}

impl<'a> GetRequestTypesRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self {
            client,
            service_desk_id: service_desk_id.into(),
            group_id: None,
            restriction_status: None,
            start: None,
            limit: None,
        }
    }

    /// Filter results where the group ID of the request type matches `groupId`
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// Filter request type by restriction status. It can be OPEN, RESTRICTED or both separated by a comma.
    #[must_use]
    pub fn restriction_status(mut self, value: impl Into<String>) -> Self {
        self.restriction_status = Some(value.into());

        self
    }

    /// The starting index of the returned objects. Base index: 0.
    #[must_use]
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttype",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        if let Some(value) = &self.group_id {
            config.query.push(("groupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.restriction_status {
            config.query.push(("restrictionStatus".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<RequestType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a new request type for a given service project. Certain fields cannot be specified on creation. These fields are given default values instead, as shown below.
///
/// Request Type icon - Question mark icon
///
/// Request Type groups - Empty, i.e. this request type will be hidden by default and not visible on the customer portal
///
/// Request Type status mapping - Empty, i.e. no custom status mapping
///
/// Request Type field mapping - Show the required fields as specified by the issue type
///
/// These fields can be updated after creation by a project administrator using the Agent view.
///
///
///
///  **Permissions:**
///
/// The calling user must be a project administrator for the service project project.
#[derive(Clone)]
pub struct CreateRequestTypeRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    request_type_create: Option<RequestTypeCreate>,
}

impl<'a> CreateRequestTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), request_type_create: None }
    }

    #[must_use]
    pub fn request_type_create(mut self, value: RequestTypeCreate) -> Self {
        self.request_type_create = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttype",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        let body = match serde_json::to_value(&self.request_type_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RequestType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a request type for a given service project. Note Issue Type cannot be changed.
///
///
///
///  **Permissions:**
///
/// The calling user must be a project administrator for the service project project.
#[derive(Clone)]
pub struct UpdateRequestTypeRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    request_type_update: Option<RequestTypeUpdate>,
}

impl<'a> UpdateRequestTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), request_type_update: None }
    }

    #[must_use]
    pub fn request_type_update(mut self, value: RequestTypeUpdate) -> Self {
        self.request_type_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttype",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        let body = match serde_json::to_value(&self.request_type_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RequestType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a request type for a given request type Id.
#[derive(Clone)]
pub struct GetRequestTypeByIdRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    request_type_id: String,
    restriction_status: Option<String>,
}

impl<'a> GetRequestTypeByIdRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        service_desk_id: impl Into<String>,
        request_type_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            service_desk_id: service_desk_id.into(),
            request_type_id: request_type_id.into(),
            restriction_status: None,
        }
    }

    /// Filter request type by restriction status. It can be OPEN, RESTRICTED or both separated by a comma.
    #[must_use]
    pub fn restriction_status(mut self, value: impl Into<String>) -> Self {
        self.restriction_status = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttype/{}",
                crate::core::encode_path_segment(&self.service_desk_id),
                crate::core::encode_path_segment(&self.request_type_id)
            ),
        );

        if let Some(value) = &self.restriction_status {
            config.query.push(("restrictionStatus".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RequestType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a request type for a given service project.
///
///
///
///  **Permissions:**
///
/// The calling user must be a project administrator for the service project project.
#[derive(Clone)]
pub struct DeleteRequestTypeRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    request_type_id: String,
}

impl<'a> DeleteRequestTypeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        service_desk_id: impl Into<String>,
        request_type_id: impl Into<String>,
    ) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), request_type_id: request_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/servicedeskapi/servicedesk/{}/requesttype/{}",
                crate::core::encode_path_segment(&self.service_desk_id),
                crate::core::encode_path_segment(&self.request_type_id)
            ),
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
