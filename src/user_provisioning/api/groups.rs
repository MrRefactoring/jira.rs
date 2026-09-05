// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Groups operations.
pub struct GroupsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GroupsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Gets the details of a group based on the id.
    pub fn get_group(&self, directory_id: impl Into<String>, id: impl Into<String>) -> GetGroupRequest<'a> {
        GetGroupRequest::new(self.client, directory_id, id)
    }

    /// Updates the details of a group with its unique ID.
    pub fn replace_group(&self, directory_id: impl Into<String>, id: impl Into<String>) -> ReplaceGroupRequest<'a> {
        ReplaceGroupRequest::new(self.client, directory_id, id)
    }

    /// Deletes a group to remove the group from the organization's directory.
    ///
    ///  **Note**: An attempt to delete a non-existent group will fail with a 404 (Resource Not found) error.
    ///
    ///  **Note**: Deleting a synced group from your identity provider will delete the group from your organization's directory and associated sites.
    ///  1. If this group is used for allocating product license (granting role in a product), then members of this group may lose access to corresponding product after group deletion.
    ///  2. If this group is used to grant permissions in product, then members of this group may lose their permissions in the corresponding product.
    pub fn delete_group(&self, directory_id: impl Into<String>, id: impl Into<String>) -> DeleteGroupRequest<'a> {
        DeleteGroupRequest::new(self.client, directory_id, id)
    }

    /// Updates a group's information in the directory and manages group membership.
    ///
    /// **Note:** Renaming groups after they've synced to your Atlassian organization isn't supported in this
    /// release of User Provisioning API. To rename a group, create a new group with the desired
    /// name, update membership, and then delete the old group.
    ///
    /// #### Example
    ///
    /// Some HTTP headers omitted and JSON payloads formatted for readability.
    ///
    /// ```text
    /// # Request
    /// PATCH /scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Groups/50202593-bc47-45df-8fa0-3f63343aa3c1 HTTP/1.1
    /// Accept: application/scim+json
    /// Accept-Charset: utf-8
    /// Content-Type: application/scim+json; charset=utf-8
    /// Authorization: Bearer 0j6lDgrjU7HmGagocgLe
    /// Host: api.atlassian.com
    ///
    /// {
    ///    "schemas":[
    ///       "urn:ietf:params:scim:api:messages:2.0:PatchOp"
    ///    ],
    ///    "Operations":[
    ///       {
    ///          "op":"add",
    ///          "path":"members",
    ///          "value":[
    ///             {
    ///                "value":"c6993c94-dbda-40f1-b6f0-18c855522ade",
    ///                "display":"dave.meyer@demotime.authteam.com"
    ///             },
    ///             {
    ///                "value":"f0ae48f7-1466-445e-85ea-e83ef754aefd",
    ///                "display":"lingbo.lu@demotime.authteam.com"
    ///             },
    ///             {
    ///                "value":"432d6f10-2e28-454e-be99-0f8c732a046f",
    ///                "display":"joanna@demotime.authteam.com"
    ///             }
    ///          ]
    ///       }
    ///    ]
    /// }
    ///
    /// # Response
    /// HTTP/1.1 200
    /// Content-Type: application/scim+json
    ///
    /// {
    ///    "schemas":[
    ///       "urn:ietf:params:scim:schemas:core:2.0:Group"
    ///    ],
    ///    "id":"50202593-bc47-45df-8fa0-3f63343aa3c1",
    ///    "displayName":"demotime-confluence-users",
    ///    "members":[
    ///       {
    ///          "type":"User",
    ///          "value":"f0ae48f7-1466-445e-85ea-e83ef754aefd",
    ///          "display":"lingbo.lu@demotime.authteam.com",
    ///          "$ref":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Users/f0ae48f7-1466-445e-85ea-e83ef754aefd"
    ///       },
    ///       {
    ///          "type":"User",
    ///          "value":"c6993c94-dbda-40f1-b6f0-18c855522ade",
    ///          "display":"dave.meyer@demotime.authteam.com",
    ///          "$ref":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Users/c6993c94-dbda-40f1-b6f0-18c855522ade"
    ///       },
    ///       {
    ///          "type":"User",
    ///          "value":"432d6f10-2e28-454e-be99-0f8c732a046f",
    ///          "display":"joanna@demotime.authteam.com",
    ///          "$ref":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Users/432d6f10-2e28-454e-be99-0f8c732a046f"
    ///       }
    ///    ],
    ///    "meta":{
    ///       "resourceType":"Group",
    ///       "location":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Groups/50202593-bc47-45df-8fa0-3f63343aa3c1",
    ///       "lastModified":"2018-09-26T17:49:09.420654Z",
    ///       "created":"2018-09-26T17:41:35.49073Z"
    ///    }
    /// }
    /// ```text
    pub fn patch_group(
        &self,
        directory_id: impl Into<String>,
        id: impl Into<String>,
        request_payload_to_patch: RequestPayloadToPatch,
    ) -> PatchGroupRequest<'a> {
        PatchGroupRequest::new(self.client, directory_id, id, request_payload_to_patch)
    }

    /// Get groups from the directory. Filter the groups by name supported with a single exact match (`eq`) against the `displayName` attribute.
    ///
    /// **Note**: While this API enables pagination, sorting functionality is not supported.
    pub fn get_groups(&self, directory_id: impl Into<String>) -> GetGroupsRequest<'a> {
        GetGroupsRequest::new(self.client, directory_id)
    }

    /// Creates a read-only group in the organization's directory. You can only edit groups from your identity provider.
    ///
    /// **Note:** An attempt to create a group with an existing name will fail with a 409 (Conflict) error.
    pub fn create_group(&self, directory_id: impl Into<String>) -> CreateGroupRequest<'a> {
        CreateGroupRequest::new(self.client, directory_id)
    }
}

/// Gets the details of a group based on the id.
#[derive(Clone)]
pub struct GetGroupRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    id: String,
}

impl<'a> GetGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, directory_id: directory_id.into(), id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/scim/directory/{}/Groups/{}",
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimGroup> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the details of a group with its unique ID.
#[derive(Clone)]
pub struct ReplaceGroupRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    id: String,
    body: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl<'a> ReplaceGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, directory_id: directory_id.into(), id: id.into(), body: None }
    }

    #[must_use]
    pub fn body(mut self, value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/scim/directory/{}/Groups/{}",
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimGroup> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a group to remove the group from the organization's directory.
///
///  **Note**: An attempt to delete a non-existent group will fail with a 404 (Resource Not found) error.
///
///  **Note**: Deleting a synced group from your identity provider will delete the group from your organization's directory and associated sites.
///  1. If this group is used for allocating product license (granting role in a product), then members of this group may lose access to corresponding product after group deletion.
///  2. If this group is used to grant permissions in product, then members of this group may lose their permissions in the corresponding product.
#[derive(Clone)]
pub struct DeleteGroupRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    id: String,
}

impl<'a> DeleteGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, directory_id: directory_id.into(), id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/scim/directory/{}/Groups/{}",
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.id)
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

/// Updates a group's information in the directory and manages group membership.
///
/// **Note:** Renaming groups after they've synced to your Atlassian organization isn't supported in this
/// release of User Provisioning API. To rename a group, create a new group with the desired
/// name, update membership, and then delete the old group.
///
/// #### Example
///
/// Some HTTP headers omitted and JSON payloads formatted for readability.
///
/// ```text
/// # Request
/// PATCH /scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Groups/50202593-bc47-45df-8fa0-3f63343aa3c1 HTTP/1.1
/// Accept: application/scim+json
/// Accept-Charset: utf-8
/// Content-Type: application/scim+json; charset=utf-8
/// Authorization: Bearer 0j6lDgrjU7HmGagocgLe
/// Host: api.atlassian.com
///
/// {
///    "schemas":[
///       "urn:ietf:params:scim:api:messages:2.0:PatchOp"
///    ],
///    "Operations":[
///       {
///          "op":"add",
///          "path":"members",
///          "value":[
///             {
///                "value":"c6993c94-dbda-40f1-b6f0-18c855522ade",
///                "display":"dave.meyer@demotime.authteam.com"
///             },
///             {
///                "value":"f0ae48f7-1466-445e-85ea-e83ef754aefd",
///                "display":"lingbo.lu@demotime.authteam.com"
///             },
///             {
///                "value":"432d6f10-2e28-454e-be99-0f8c732a046f",
///                "display":"joanna@demotime.authteam.com"
///             }
///          ]
///       }
///    ]
/// }
///
/// # Response
/// HTTP/1.1 200
/// Content-Type: application/scim+json
///
/// {
///    "schemas":[
///       "urn:ietf:params:scim:schemas:core:2.0:Group"
///    ],
///    "id":"50202593-bc47-45df-8fa0-3f63343aa3c1",
///    "displayName":"demotime-confluence-users",
///    "members":[
///       {
///          "type":"User",
///          "value":"f0ae48f7-1466-445e-85ea-e83ef754aefd",
///          "display":"lingbo.lu@demotime.authteam.com",
///          "$ref":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Users/f0ae48f7-1466-445e-85ea-e83ef754aefd"
///       },
///       {
///          "type":"User",
///          "value":"c6993c94-dbda-40f1-b6f0-18c855522ade",
///          "display":"dave.meyer@demotime.authteam.com",
///          "$ref":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Users/c6993c94-dbda-40f1-b6f0-18c855522ade"
///       },
///       {
///          "type":"User",
///          "value":"432d6f10-2e28-454e-be99-0f8c732a046f",
///          "display":"joanna@demotime.authteam.com",
///          "$ref":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Users/432d6f10-2e28-454e-be99-0f8c732a046f"
///       }
///    ],
///    "meta":{
///       "resourceType":"Group",
///       "location":"https://api.atlassian.com/scim/directory/2fb21891-7bee-4c2d-a61a-ade3834c8b2b/Groups/50202593-bc47-45df-8fa0-3f63343aa3c1",
///       "lastModified":"2018-09-26T17:49:09.420654Z",
///       "created":"2018-09-26T17:41:35.49073Z"
///    }
/// }
/// ```text
#[derive(Clone)]
pub struct PatchGroupRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    id: String,
    request_payload_to_patch: RequestPayloadToPatch,
}

impl<'a> PatchGroupRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        directory_id: impl Into<String>,
        id: impl Into<String>,
        request_payload_to_patch: RequestPayloadToPatch,
    ) -> Self {
        Self { client, directory_id: directory_id.into(), id: id.into(), request_payload_to_patch }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PATCH,
            format!(
                "/scim/directory/{}/Groups/{}",
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.id)
            ),
        );

        let body = match serde_json::to_value(&self.request_payload_to_patch)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimGroup> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get groups from the directory. Filter the groups by name supported with a single exact match (`eq`) against the `displayName` attribute.
///
/// **Note**: While this API enables pagination, sorting functionality is not supported.
#[derive(Clone)]
pub struct GetGroupsRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    filter: Option<String>,
    start_index: Option<i64>,
    count: Option<i64>,
}

impl<'a> GetGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>) -> Self {
        Self { client, directory_id: directory_id.into(), filter: None, start_index: None, count: None }
    }

    /// Filter for `displayName`. Example: `displayName eq "SCIM_GROUP"`
    #[must_use]
    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());

        self
    }

    /// A 1-based index of the first query result.
    #[must_use]
    pub fn start_index(mut self, value: i64) -> Self {
        self.start_index = Some(value);

        self
    }

    /// Desired maximum number of query results in the list response page.
    #[must_use]
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/scim/directory/{}/Groups", crate::core::encode_path_segment(&self.directory_id)),
        );

        if let Some(value) = &self.filter {
            config.query.push(("filter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_index {
            config.query.push(("startIndex".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.count {
            config.query.push(("count".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimGroupListResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a read-only group in the organization's directory. You can only edit groups from your identity provider.
///
/// **Note:** An attempt to create a group with an existing name will fail with a 409 (Conflict) error.
#[derive(Clone)]
pub struct CreateGroupRequest<'a> {
    client: &'a crate::core::Client,
    directory_id: String,
    body: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl<'a> CreateGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, directory_id: impl Into<String>) -> Self {
        Self { client, directory_id: directory_id.into(), body: None }
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
            format!("/scim/directory/{}/Groups", crate::core::encode_path_segment(&self.directory_id)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScimGroup> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
