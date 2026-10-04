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

    /// Updates an avatar for a project. This is step 3/3 of changing an avatar for a project.
    pub fn update_project_avatar(
        &self,
        project_id_or_key: impl Into<String>,
        avatar: Avatar,
    ) -> UpdateProjectAvatarRequest<'a> {
        UpdateProjectAvatarRequest::new(self.client, project_id_or_key, avatar)
    }

    /// Deletes avatar
    pub fn delete_project_avatar(
        &self,
        project_id_or_key: impl Into<String>,
        id: i64,
    ) -> DeleteProjectAvatarRequest<'a> {
        DeleteProjectAvatarRequest::new(self.client, project_id_or_key, id)
    }

    /// Returns all avatars which are visible for the currently logged in user. The avatars are grouped into system and custom.
    pub fn get_all_project_avatars(&self, project_id_or_key: impl Into<String>) -> GetAllProjectAvatarsRequest<'a> {
        GetAllProjectAvatarsRequest::new(self.client, project_id_or_key)
    }
}

/// Updates an avatar for a project. This is step 3/3 of changing an avatar for a project.
#[derive(Clone)]
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
            format!("/rest/api/2/project/{}/avatar", crate::core::encode_path_segment(&self.project_id_or_key)),
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

/// Deletes avatar
#[derive(Clone)]
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
                "/rest/api/2/project/{}/avatar/{}",
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

/// Returns all avatars which are visible for the currently logged in user. The avatars are grouped into system and custom.
#[derive(Clone)]
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
            format!("/rest/api/2/project/{}/avatars", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetAllProjectAvatars> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
