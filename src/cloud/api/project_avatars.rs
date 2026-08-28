// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectAvatars operations.
pub struct ProjectAvatarsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectAvatarsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Sets the avatar displayed for a project.
    ///
    /// Use [Load project avatar](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-avatar2-post) to store avatars against the project, before using this operation to set the displayed avatar.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
    pub fn update_project_avatar(
        &self,
        project_id_or_key: impl Into<String>,
        avatar: Avatar,
    ) -> UpdateProjectAvatarRequest<'a> {
        UpdateProjectAvatarRequest::new(self.client, project_id_or_key, avatar)
    }

    /// Deletes a custom avatar from a project. Note that system avatars cannot be deleted.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
    pub fn delete_project_avatar(
        &self,
        project_id_or_key: impl Into<String>,
        id: i64,
    ) -> DeleteProjectAvatarRequest<'a> {
        DeleteProjectAvatarRequest::new(self.client, project_id_or_key, id)
    }

    /// Loads an avatar for a project.
    ///
    /// Specify the avatar's local file location in the body of the request. Also, include the following headers:
    ///
    ///  *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#special-request-headers).
    ///  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.
    ///
    /// For example:
    /// `curl --request POST `
    ///
    /// `--user email@example.com:<api_token> `
    ///
    /// `--header 'X-Atlassian-Token: no-check' `
    ///
    /// `--header 'Content-Type: image/< image_type>' `
    ///
    /// `--data-binary "<@/path/to/file/with/your/avatar>" `
    ///
    /// `--url 'https://your-domain.atlassian.net/rest/api/3/project/{projectIdOrKey}/avatar2'`
    ///
    /// The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.
    ///
    /// The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.
    ///
    /// After creating the avatar use [Set project avatar](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-avatar-put) to set it as the project's displayed avatar.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
    pub fn create_project_avatar(
        &self,
        project_id_or_key: impl Into<String>,
        body: impl Into<bytes::Bytes>,
    ) -> CreateProjectAvatarRequest<'a> {
        CreateProjectAvatarRequest::new(self.client, project_id_or_key, body)
    }

    /// Returns all project avatars, grouped by system and custom avatars.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn get_all_project_avatars(&self, project_id_or_key: impl Into<String>) -> GetAllProjectAvatarsRequest<'a> {
        GetAllProjectAvatarsRequest::new(self.client, project_id_or_key)
    }
}

/// Sets the avatar displayed for a project.
///
/// Use [Load project avatar](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-avatar2-post) to store avatars against the project, before using this operation to set the displayed avatar.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
pub struct UpdateProjectAvatarRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    avatar: Avatar,
}

impl<'a> UpdateProjectAvatarRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>, avatar: Avatar) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), avatar }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/project/{}/avatar", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        let body = match serde_json::to_value(&self.avatar)? {
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

/// Deletes a custom avatar from a project. Note that system avatars cannot be deleted.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
pub struct DeleteProjectAvatarRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    id: i64,
}

impl<'a> DeleteProjectAvatarRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>, id: i64) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/project/{}/avatar/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                self.id
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

/// Loads an avatar for a project.
///
/// Specify the avatar's local file location in the body of the request. Also, include the following headers:
///
///  *  `X-Atlassian-Token: no-check` To prevent XSRF protection blocking the request, for more information see [Special Headers](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#special-request-headers).
///  *  `Content-Type: image/image type` Valid image types are JPEG, GIF, or PNG.
///
/// For example:
/// `curl --request POST `
///
/// `--user email@example.com:<api_token> `
///
/// `--header 'X-Atlassian-Token: no-check' `
///
/// `--header 'Content-Type: image/< image_type>' `
///
/// `--data-binary "<@/path/to/file/with/your/avatar>" `
///
/// `--url 'https://your-domain.atlassian.net/rest/api/3/project/{projectIdOrKey}/avatar2'`
///
/// The avatar is cropped to a square. If no crop parameters are specified, the square originates at the top left of the image. The length of the square's sides is set to the smaller of the height or width of the image.
///
/// The cropped image is then used to create avatars of 16x16, 24x24, 32x32, and 48x48 in size.
///
/// After creating the avatar use [Set project avatar](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-avatar-put) to set it as the project's displayed avatar.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
pub struct CreateProjectAvatarRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    x: Option<i64>,
    y: Option<i64>,
    size: Option<i64>,
    body: bytes::Bytes,
    content_type: Option<String>,
}

impl<'a> CreateProjectAvatarRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        body: impl Into<bytes::Bytes>,
    ) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
            body: body.into(),
            x: None,
            y: None,
            size: None,
            content_type: None,
        }
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

    /// The length of each side of the crop region.
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);

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
            format!("/rest/api/3/project/{}/avatar2", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.x {
            config.query.push(("x".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.y {
            config.query.push(("y".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.size {
            config.query.push(("size".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

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

/// Returns all project avatars, grouped by system and custom avatars.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
pub struct GetAllProjectAvatarsRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> GetAllProjectAvatarsRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/avatars", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectAvatars> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
