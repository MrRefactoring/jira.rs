// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueTypes operations.
pub struct IssueTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all issue types.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issue types are only returned as follows:
    ///
    ///  *  if the user has the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), all issue types are returned.
    ///  *  if the user has the *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for one or more projects, the issue types associated with the projects the user has permission to browse are returned.
    ///  *  if the user is anonymous then they will be able to access projects with the *Browse projects* for anonymous users
    ///  *  if the user authentication is incorrect they will fall back to anonymous
    pub fn get_issue_all_types(&self) -> GetIssueAllTypesRequest<'a> {
        GetIssueAllTypesRequest::new(self.client)
    }

    /// Creates an issue type.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_issue_type(&self, issue_type_create: IssueTypeCreate) -> CreateIssueTypeRequest<'a> {
        CreateIssueTypeRequest::new(self.client, issue_type_create)
    }

    /// Returns an issue type.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) in a project the issue type is associated with or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_issue_type(&self, id: impl Into<String>) -> GetIssueTypeRequest<'a> {
        GetIssueTypeRequest::new(self.client, id)
    }

    /// Updates the issue type.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn update_issue_type(
        &self,
        id: impl Into<String>,
        issue_type_update: IssueTypeUpdate,
    ) -> UpdateIssueTypeRequest<'a> {
        UpdateIssueTypeRequest::new(self.client, id, issue_type_update)
    }

    /// Deletes the issue type. If the issue type is in use, all uses are updated with the alternative issue type (`alternativeIssueTypeId`). A list of alternative issue types are obtained from the [Get alternative issue types](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issuetype/#api-rest-api-3-issuetype-id-alternatives-get) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_issue_type(&self, id: impl Into<String>) -> DeleteIssueTypeRequest<'a> {
        DeleteIssueTypeRequest::new(self.client, id)
    }

    /// Returns a list of issue types that can be used to replace the issue type. The alternative issue types are those assigned to the same workflow scheme, field configuration scheme, and screen scheme.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_alternative_issue_types(&self, id: impl Into<String>) -> GetAlternativeIssueTypesRequest<'a> {
        GetAlternativeIssueTypesRequest::new(self.client, id)
    }

    /// Loads an avatar for the issue type.
    ///
    /// Specify the avatar's local file location in the body of the request. Also, include the following headers:
    ///
    ///  *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#special-request-headers).
    ///  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.
    ///
    /// For example:
    /// `curl --request POST \ --user email@example.com:<api_token> \ --header 'X-Atlassian-Token: no-check' \ --header 'Content-Type: image/< image_type>' \ --data-binary "<@/path/to/file/with/your/avatar>" \ --url 'https://your-domain.atlassian.net/rest/api/3/issuetype/{issueTypeId}'This`
    ///
    /// The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.
    ///
    /// The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.
    ///
    /// After creating the avatar, use [ Update issue type](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issuetype/#api-rest-api-3-issuetype-id-put) to set it as the issue type's displayed avatar.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_issue_type_avatar(
        &self,
        id: impl Into<String>,
        size: i64,
        body: impl Into<bytes::Bytes>,
    ) -> CreateIssueTypeAvatarRequest<'a> {
        CreateIssueTypeAvatarRequest::new(self.client, id, size, body)
    }
}

/// Returns all issue types.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issue types are only returned as follows:
///
///  *  if the user has the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), all issue types are returned.
///  *  if the user has the *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for one or more projects, the issue types associated with the projects the user has permission to browse are returned.
///  *  if the user is anonymous then they will be able to access projects with the *Browse projects* for anonymous users
///  *  if the user authentication is incorrect they will fall back to anonymous
pub struct GetIssueAllTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIssueAllTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/issuetype".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<IssueTypeDetails>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates an issue type.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct CreateIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_create: IssueTypeCreate,
}

impl<'a> CreateIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_create: IssueTypeCreate) -> Self {
        Self { client, issue_type_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/issuetype".to_owned());

        let body = match serde_json::to_value(&self.issue_type_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeDetails> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns an issue type.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) in a project the issue type is associated with or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/issuetype/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeDetails> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the issue type.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct UpdateIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    issue_type_update: IssueTypeUpdate,
}

impl<'a> UpdateIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, issue_type_update: IssueTypeUpdate) -> Self {
        Self { client, id: id.into(), issue_type_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/issuetype/{}", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.issue_type_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeDetails> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes the issue type. If the issue type is in use, all uses are updated with the alternative issue type (`alternativeIssueTypeId`). A list of alternative issue types are obtained from the [Get alternative issue types](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issuetype/#api-rest-api-3-issuetype-id-alternatives-get) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct DeleteIssueTypeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    alternative_issue_type_id: Option<String>,
}

impl<'a> DeleteIssueTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), alternative_issue_type_id: None }
    }

    /// The ID of the replacement issue type.
    #[must_use]
    pub fn alternative_issue_type_id(mut self, value: impl Into<String>) -> Self {
        self.alternative_issue_type_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/issuetype/{}", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.alternative_issue_type_id {
            config.query.push(("alternativeIssueTypeId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Returns a list of issue types that can be used to replace the issue type. The alternative issue types are those assigned to the same workflow scheme, field configuration scheme, and screen scheme.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
pub struct GetAlternativeIssueTypesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetAlternativeIssueTypesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/issuetype/{}/alternatives", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<IssueTypeDetails>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Loads an avatar for the issue type.
///
/// Specify the avatar's local file location in the body of the request. Also, include the following headers:
///
///  *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#special-request-headers).
///  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.
///
/// For example:
/// `curl --request POST \ --user email@example.com:<api_token> \ --header 'X-Atlassian-Token: no-check' \ --header 'Content-Type: image/< image_type>' \ --data-binary "<@/path/to/file/with/your/avatar>" \ --url 'https://your-domain.atlassian.net/rest/api/3/issuetype/{issueTypeId}'This`
///
/// The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.
///
/// The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.
///
/// After creating the avatar, use [ Update issue type](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issuetype/#api-rest-api-3-issuetype-id-put) to set it as the issue type's displayed avatar.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct CreateIssueTypeAvatarRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    x: Option<i64>,
    y: Option<i64>,
    size: i64,
    body: bytes::Bytes,
    content_type: Option<String>,
}

impl<'a> CreateIssueTypeAvatarRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, size: i64, body: impl Into<bytes::Bytes>) -> Self {
        Self { client, id: id.into(), size, body: body.into(), x: None, y: None, content_type: None }
    }

    /// The X coordinate of the top-left corner of the crop region.
    #[must_use]
    pub fn x(mut self, value: i64) -> Self {
        self.x = Some(value);

        self
    }

    /// The Y coordinate of the top-left corner of the crop region.
    #[must_use]
    pub fn y(mut self, value: i64) -> Self {
        self.y = Some(value);

        self
    }

    /// The media type of the bytes being sent, e.g. `image/png`.
    #[must_use]
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/issuetype/{}/avatar2", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.x {
            config.query.push(("x".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.y {
            config.query.push(("y".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.query.push(("size".to_owned(), crate::core::QueryValue::Scalar(self.size.to_string())));

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body = Some(crate::core::Body::Bytes(self.body.clone()));

        config.content_type = self.content_type.clone().or(None);

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Avatar> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
