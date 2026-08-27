// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Teams operations.
pub struct TeamsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> TeamsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// This returns a list of all teams contained under an organization. This may be used as an option to export teams data within your organization.
    pub fn query_teams(&self, org_id: impl Into<String>) -> QueryTeamsRequest<'a> {
        QueryTeamsRequest::new(self.client, org_id)
    }

    /// Creates a team, and adds the requesting user as the initial member.
    pub fn create_team(
        &self,
        org_id: impl Into<String>,
        team_creation_payload: TeamCreationPayload,
    ) -> CreateTeamRequest<'a> {
        CreateTeamRequest::new(self.client, org_id, team_creation_payload)
    }

    pub fn archive_teams(
        &self,
        org_id: impl Into<String>,
        bulk_operation_request: BulkOperationRequest,
    ) -> ArchiveTeamsRequest<'a> {
        ArchiveTeamsRequest::new(self.client, org_id, bulk_operation_request)
    }

    pub fn unarchive_teams(
        &self,
        org_id: impl Into<String>,
        bulk_operation_request: BulkOperationRequest,
    ) -> UnarchiveTeamsRequest<'a> {
        UnarchiveTeamsRequest::new(self.client, org_id, bulk_operation_request)
    }

    pub fn get_team(&self, org_id: impl Into<String>, team_id: impl Into<String>) -> GetTeamRequest<'a> {
        GetTeamRequest::new(self.client, org_id, team_id)
    }

    pub fn delete_team(&self, org_id: impl Into<String>, team_id: impl Into<String>) -> DeleteTeamRequest<'a> {
        DeleteTeamRequest::new(self.client, org_id, team_id)
    }

    /// This will only update the fields that get passed in and leave the rest as unmodified.
    pub fn update_team(
        &self,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        team_update_payload: TeamUpdatePayload,
    ) -> UpdateTeamRequest<'a> {
        UpdateTeamRequest::new(self.client, org_id, team_id, team_update_payload)
    }

    pub fn restore_team(&self, org_id: impl Into<String>, team_id: impl Into<String>) -> RestoreTeamRequest<'a> {
        RestoreTeamRequest::new(self.client, org_id, team_id)
    }

    /// This updates the cover photo of the team. The cover photo must be a valid image file.
    pub fn upload_and_set_team_cover_photo(
        &self,
        team_id: impl Into<String>,
        file: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> UploadAndSetTeamCoverPhotoRequest<'a> {
        UploadAndSetTeamCoverPhotoRequest::new(self.client, team_id, file)
    }
}

/// This returns a list of all teams contained under an organization. This may be used as an option to export teams data within your organization.
pub struct QueryTeamsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    site_id: Option<String>,
    size: Option<i64>,
    cursor: Option<String>,
}

impl<'a> QueryTeamsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), site_id: None, size: None, cursor: None }
    }

    /// \[Optional\] The ID of the site to retrieve teams which are site scoped. Please note that if the org is site-scoped, teams will not be included in response if siteId is not provided. \[Deprecated\] Omitting siteId is deprecated. With the introduction of Units, orgId alone is no longer sufficient to resolve the scope of teams. Always provide a valid siteId to ensure this operation continues to work in the future.
    #[must_use]
    pub fn site_id(mut self, value: impl Into<String>) -> Self {
        self.site_id = Some(value.into());

        self
    }

    /// The page size for the number of teams to return (max 300)
    #[must_use]
    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);

        self
    }

    /// An optional cursor token. Leave off for the first request.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/gateway/api/public/teams/v1/org/{}/teams", self.org_id),
        );

        if let Some(value) = &self.site_id {
            config.query.push(("siteId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.size {
            config.query.push(("size".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TeamPaginationResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a team, and adds the requesting user as the initial member.
pub struct CreateTeamRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_creation_payload: TeamCreationPayload,
}

impl<'a> CreateTeamRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        team_creation_payload: TeamCreationPayload,
    ) -> Self {
        Self { client, org_id: org_id.into(), team_creation_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/gateway/api/public/teams/v1/org/{}/teams", self.org_id),
        );

        let body = match serde_json::to_value(&self.team_creation_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TeamResponseWithMembers> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

pub struct ArchiveTeamsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    bulk_operation_request: BulkOperationRequest,
}

impl<'a> ArchiveTeamsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        bulk_operation_request: BulkOperationRequest,
    ) -> Self {
        Self { client, org_id: org_id.into(), bulk_operation_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/gateway/api/public/teams/v1/org/{}/teams/archive", self.org_id),
        );

        let body = match serde_json::to_value(&self.bulk_operation_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkOperationResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

pub struct UnarchiveTeamsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    bulk_operation_request: BulkOperationRequest,
}

impl<'a> UnarchiveTeamsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        bulk_operation_request: BulkOperationRequest,
    ) -> Self {
        Self { client, org_id: org_id.into(), bulk_operation_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/gateway/api/public/teams/v1/org/{}/teams/unarchive", self.org_id),
        );

        let body = match serde_json::to_value(&self.bulk_operation_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkOperationResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

pub struct GetTeamRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
    site_id: Option<String>,
}

impl<'a> GetTeamRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, team_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into(), site_id: None }
    }

    /// \[Optional\] The ID of the site to retrieve teams which are site scoped. Please note that if the org is site-scoped, teams will not be included in response if siteId is not provided. \[Deprecated\] Omitting siteId is deprecated. With the introduction of Units, orgId alone is no longer sufficient to resolve the scope of teams. Always provide a valid siteId to ensure this operation continues to work in the future.
    #[must_use]
    pub fn site_id(mut self, value: impl Into<String>) -> Self {
        self.site_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/gateway/api/public/teams/v1/org/{}/teams/{}", self.org_id, self.team_id),
        );

        if let Some(value) = &self.site_id {
            config.query.push(("siteId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TeamResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

pub struct DeleteTeamRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
}

impl<'a> DeleteTeamRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, team_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/gateway/api/public/teams/v1/org/{}/teams/{}", self.org_id, self.team_id),
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

/// This will only update the fields that get passed in and leave the rest as unmodified.
pub struct UpdateTeamRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
    team_update_payload: TeamUpdatePayload,
}

impl<'a> UpdateTeamRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        team_update_payload: TeamUpdatePayload,
    ) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into(), team_update_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PATCH,
            format!("/gateway/api/public/teams/v1/org/{}/teams/{}", self.org_id, self.team_id),
        );

        let body = match serde_json::to_value(&self.team_update_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TeamResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

pub struct RestoreTeamRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
}

impl<'a> RestoreTeamRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, team_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/gateway/api/public/teams/v1/org/{}/teams/{}/restore", self.org_id, self.team_id),
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

/// This updates the cover photo of the team. The cover photo must be a valid image file.
pub struct UploadAndSetTeamCoverPhotoRequest<'a> {
    client: &'a crate::core::Client,
    team_id: String,
    file: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> UploadAndSetTeamCoverPhotoRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        team_id: impl Into<String>,
        file: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> Self {
        Self { client, team_id: team_id.into(), file: file.into_iter().collect(), content_type: None }
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
            crate::core::Method::PUT,
            format!("/gateway/api/public/teams/v1/{}/cover-photo", self.team_id),
        );

        config.body = Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("file", self.file.clone())));

        config.content_type = self.content_type.clone().or(None);

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
