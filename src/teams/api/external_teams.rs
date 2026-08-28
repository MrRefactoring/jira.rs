// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ExternalTeams operations.
pub struct ExternalTeamsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ExternalTeamsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates an external linked team, and membership will be synced with the external reference.
    pub fn create_external_linked_team(
        &self,
        org_id: impl Into<String>,
        external_team_creation_payload: ExternalTeamCreationPayload,
    ) -> CreateExternalLinkedTeamRequest<'a> {
        CreateExternalLinkedTeamRequest::new(self.client, org_id, external_team_creation_payload)
    }

    /// Unlinks managed teams from their external references in bulk. Each team's membership setting will be transitioned from EXTERNAL to ORG_ADMIN_MANAGED.
    pub fn unlink_teams_from_external_source(
        &self,
        org_id: impl Into<String>,
        bulk_operation_request: BulkOperationRequest,
    ) -> UnlinkTeamsFromExternalSourceRequest<'a> {
        UnlinkTeamsFromExternalSourceRequest::new(self.client, org_id, bulk_operation_request)
    }

    /// Links an existing team to an external reference, and membership and team name will be synced with the external reference.
    pub fn link_team_to_external_source(
        &self,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        link_team_to_external_source_payload: LinkTeamToExternalSourcePayload,
    ) -> LinkTeamToExternalSourceRequest<'a> {
        LinkTeamToExternalSourceRequest::new(self.client, org_id, team_id, link_team_to_external_source_payload)
    }
}

/// Creates an external linked team, and membership will be synced with the external reference.
#[derive(Clone)]
pub struct CreateExternalLinkedTeamRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    external_team_creation_payload: ExternalTeamCreationPayload,
}

impl<'a> CreateExternalLinkedTeamRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        external_team_creation_payload: ExternalTeamCreationPayload,
    ) -> Self {
        Self { client, org_id: org_id.into(), external_team_creation_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/gateway/api/public/teams/v1/org/{}/teams/external",
                crate::core::encode_path_segment(&self.org_id)
            ),
        );

        let body = match serde_json::to_value(&self.external_team_creation_payload)? {
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

/// Unlinks managed teams from their external references in bulk. Each team's membership setting will be transitioned from EXTERNAL to ORG_ADMIN_MANAGED.
#[derive(Clone)]
pub struct UnlinkTeamsFromExternalSourceRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    bulk_operation_request: BulkOperationRequest,
}

impl<'a> UnlinkTeamsFromExternalSourceRequest<'a> {
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
            format!(
                "/gateway/api/public/teams/v1/org/{}/teams/external/bulk/unlink",
                crate::core::encode_path_segment(&self.org_id)
            ),
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

/// Links an existing team to an external reference, and membership and team name will be synced with the external reference.
#[derive(Clone)]
pub struct LinkTeamToExternalSourceRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
    link_team_to_external_source_payload: LinkTeamToExternalSourcePayload,
}

impl<'a> LinkTeamToExternalSourceRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        link_team_to_external_source_payload: LinkTeamToExternalSourcePayload,
    ) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into(), link_team_to_external_source_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/gateway/api/public/teams/v1/org/{}/teams/{}/external/link",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.team_id)
            ),
        );

        let body = match serde_json::to_value(&self.link_team_to_external_source_payload)? {
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
