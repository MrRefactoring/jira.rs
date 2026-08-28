// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueCustomFieldAssociations operations.
pub struct IssueCustomFieldAssociationsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueCustomFieldAssociationsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Associates fields with projects.
    ///
    /// Fields will be associated with each issue type on the requested projects.
    ///
    /// Fields will be associated with all projects that share the same field configuration which the provided projects are using. This means that while the field will be associated with the requested projects, it will also be associated with any other projects that share the same field configuration.
    ///
    /// If a success response is returned it means that the field association has been created in any applicable contexts where it wasn't already present.
    ///
    /// Up to 50 fields and up to 100 projects can be associated in a single request. If more fields or projects are provided a 400 response will be returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_associations(
        &self,
        field_associations_request: FieldAssociationsRequest,
    ) -> CreateAssociationsRequest<'a> {
        CreateAssociationsRequest::new(self.client, field_associations_request)
    }

    /// Unassociates a set of fields with a project and issue type context.
    ///
    /// Fields will be unassociated with all projects/issue types that share the same field configuration which the provided project and issue types are using. This means that while the field will be unassociated with the provided project and issue types, it will also be unassociated with any other projects and issue types that share the same field configuration.
    ///
    /// If a success response is returned it means that the field association has been removed in any applicable contexts where it was present.
    ///
    /// Up to 50 fields and up to 100 projects and issue types can be unassociated in a single request. If more fields or projects are provided a 400 response will be returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn remove_associations(
        &self,
        field_associations_request: FieldAssociationsRequest,
    ) -> RemoveAssociationsRequest<'a> {
        RemoveAssociationsRequest::new(self.client, field_associations_request)
    }
}

/// Associates fields with projects.
///
/// Fields will be associated with each issue type on the requested projects.
///
/// Fields will be associated with all projects that share the same field configuration which the provided projects are using. This means that while the field will be associated with the requested projects, it will also be associated with any other projects that share the same field configuration.
///
/// If a success response is returned it means that the field association has been created in any applicable contexts where it wasn't already present.
///
/// Up to 50 fields and up to 100 projects can be associated in a single request. If more fields or projects are provided a 400 response will be returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct CreateAssociationsRequest<'a> {
    client: &'a crate::core::Client,
    field_associations_request: FieldAssociationsRequest,
}

impl<'a> CreateAssociationsRequest<'a> {
    fn new(client: &'a crate::core::Client, field_associations_request: FieldAssociationsRequest) -> Self {
        Self { client, field_associations_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/field/association".to_owned());

        let body = match serde_json::to_value(&self.field_associations_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Unassociates a set of fields with a project and issue type context.
///
/// Fields will be unassociated with all projects/issue types that share the same field configuration which the provided project and issue types are using. This means that while the field will be unassociated with the provided project and issue types, it will also be unassociated with any other projects and issue types that share the same field configuration.
///
/// If a success response is returned it means that the field association has been removed in any applicable contexts where it was present.
///
/// Up to 50 fields and up to 100 projects and issue types can be unassociated in a single request. If more fields or projects are provided a 400 response will be returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct RemoveAssociationsRequest<'a> {
    client: &'a crate::core::Client,
    field_associations_request: FieldAssociationsRequest,
}

impl<'a> RemoveAssociationsRequest<'a> {
    fn new(client: &'a crate::core::Client, field_associations_request: FieldAssociationsRequest) -> Self {
        Self { client, field_associations_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/3/field/association".to_owned());

        let body = match serde_json::to_value(&self.field_associations_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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
