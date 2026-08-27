// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Avatars operations.
pub struct AvatarsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AvatarsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all system avatars of the given type.
    pub fn get_all_system_avatars(&self, r#type: impl Into<String>) -> GetAllSystemAvatarsRequest<'a> {
        GetAllSystemAvatarsRequest::new(self.client, r#type)
    }

    /// Returns a list of all avatars
    pub fn get_avatars(&self, r#type: impl Into<String>, owning_object_id: impl Into<String>) -> GetAvatarsRequest<'a> {
        GetAvatarsRequest::new(self.client, r#type, owning_object_id)
    }

    /// Creates avatar from temporary
    pub fn create_avatar_from_temporary(
        &self,
        r#type: impl Into<String>,
        owning_object_id: impl Into<String>,
    ) -> CreateAvatarFromTemporaryRequest<'a> {
        CreateAvatarFromTemporaryRequest::new(self.client, r#type, owning_object_id)
    }

    /// Deletes avatar
    pub fn delete_avatar(
        &self,
        id: i64,
        r#type: impl Into<String>,
        owning_object_id: impl Into<String>,
    ) -> DeleteAvatarRequest<'a> {
        DeleteAvatarRequest::new(self.client, id, r#type, owning_object_id)
    }

    /// Creates temporary avatar
    pub fn store_temporary_avatar_using_multi_part(
        &self,
        r#type: impl Into<String>,
        owning_object_id: impl Into<String>,
        avatar: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> StoreTemporaryAvatarUsingMultiPartRequest<'a> {
        StoreTemporaryAvatarUsingMultiPartRequest::new(self.client, r#type, owning_object_id, avatar)
    }
}

/// Returns all system avatars of the given type.
pub struct GetAllSystemAvatarsRequest<'a> {
    client: &'a crate::core::Client,
    r#type: String,
}

impl<'a> GetAllSystemAvatarsRequest<'a> {
    fn new(client: &'a crate::core::Client, r#type: impl Into<String>) -> Self {
        Self { client, r#type: r#type.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/avatar/{}/system", self.r#type),
        );

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

/// Returns a list of all avatars
pub struct GetAvatarsRequest<'a> {
    client: &'a crate::core::Client,
    r#type: String,
    owning_object_id: String,
}

impl<'a> GetAvatarsRequest<'a> {
    fn new(client: &'a crate::core::Client, r#type: impl Into<String>, owning_object_id: impl Into<String>) -> Self {
        Self { client, r#type: r#type.into(), owning_object_id: owning_object_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/universal_avatar/type/{}/owner/{}", self.r#type, self.owning_object_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetAvatars> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates avatar from temporary
pub struct CreateAvatarFromTemporaryRequest<'a> {
    client: &'a crate::core::Client,
    r#type: String,
    owning_object_id: String,
    avatar_cropping: Option<AvatarCropping>,
}

impl<'a> CreateAvatarFromTemporaryRequest<'a> {
    fn new(client: &'a crate::core::Client, r#type: impl Into<String>, owning_object_id: impl Into<String>) -> Self {
        Self { client, r#type: r#type.into(), owning_object_id: owning_object_id.into(), avatar_cropping: None }
    }

    #[must_use]
    pub fn avatar_cropping(mut self, value: AvatarCropping) -> Self {
        self.avatar_cropping = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/universal_avatar/type/{}/owner/{}/avatar", self.r#type, self.owning_object_id),
        );

        let body = match serde_json::to_value(&self.avatar_cropping)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Deletes avatar
pub struct DeleteAvatarRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    r#type: String,
    owning_object_id: String,
}

impl<'a> DeleteAvatarRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        id: i64,
        r#type: impl Into<String>,
        owning_object_id: impl Into<String>,
    ) -> Self {
        Self { client, id, r#type: r#type.into(), owning_object_id: owning_object_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/universal_avatar/type/{}/owner/{}/avatar/{}",
                self.r#type, self.owning_object_id, self.id
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

/// Creates temporary avatar
pub struct StoreTemporaryAvatarUsingMultiPartRequest<'a> {
    client: &'a crate::core::Client,
    r#type: String,
    owning_object_id: String,
    avatar: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> StoreTemporaryAvatarUsingMultiPartRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        r#type: impl Into<String>,
        owning_object_id: impl Into<String>,
        avatar: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> Self {
        Self {
            client,
            r#type: r#type.into(),
            owning_object_id: owning_object_id.into(),
            avatar: avatar.into_iter().collect(),
            content_type: None,
        }
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
            format!("/rest/api/2/universal_avatar/type/{}/owner/{}/temp", self.r#type, self.owning_object_id),
        );

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body =
            Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("avatar", self.avatar.clone())));

        config.content_type = self.content_type.clone().or(None);

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AvatarCropping> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
