// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The TeamMembers operations.
pub struct TeamMembersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> TeamMembersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a set of account IDs who are members of the team, alongside a pagination cursor to retrieve the next page (if available).
    pub fn fetch_members(&self, org_id: impl Into<String>, team_id: impl Into<String>) -> FetchMembersRequest<'a> {
        FetchMembersRequest::new(self.client, org_id, team_id)
    }

    /// The account IDs specified will be added to the team.
    pub fn add_members(
        &self,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        membership_add_payload: MembershipAddPayload,
    ) -> AddMembersRequest<'a> {
        AddMembersRequest::new(self.client, org_id, team_id, membership_add_payload)
    }

    /// The account IDs specified will be removed from the team.
    pub fn remove_members(
        &self,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        membership_remove_payload: MembershipRemovePayload,
    ) -> RemoveMembersRequest<'a> {
        RemoveMembersRequest::new(self.client, org_id, team_id, membership_remove_payload)
    }
}

/// Returns a set of account IDs who are members of the team, alongside a pagination cursor to retrieve the next page (if available).
pub struct FetchMembersRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
    site_id: Option<String>,
    membership_fetch_payload: Option<MembershipFetchPayload>,
}

impl<'a> FetchMembersRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, team_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into(), site_id: None, membership_fetch_payload: None }
    }

    /// \[Optional\] The ID of the site you are fetching members for. \[Deprecated\] Omitting siteId is deprecated. With the introduction of Units, orgId alone is no longer sufficient to resolve the scope of teams. Always provide a valid siteId to ensure this operation continues to work in the future.
    #[deprecated(note = "\\[Deprecated\\] Omitting siteId is deprecated.")]
    #[must_use]
    pub fn site_id(mut self, value: impl Into<String>) -> Self {
        self.site_id = Some(value.into());

        self
    }

    #[must_use]
    pub fn membership_fetch_payload(mut self, value: MembershipFetchPayload) -> Self {
        self.membership_fetch_payload = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/gateway/api/public/teams/v1/org/{}/teams/{}/members",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.team_id)
            ),
        );

        if let Some(value) = &self.site_id {
            config.query.push(("siteId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.membership_fetch_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MembershipPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// The account IDs specified will be added to the team.
pub struct AddMembersRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
    membership_add_payload: MembershipAddPayload,
}

impl<'a> AddMembersRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        membership_add_payload: MembershipAddPayload,
    ) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into(), membership_add_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/gateway/api/public/teams/v1/org/{}/teams/{}/members/add",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.team_id)
            ),
        );

        let body = match serde_json::to_value(&self.membership_add_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MembershipAddResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// The account IDs specified will be removed from the team.
pub struct RemoveMembersRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    team_id: String,
    membership_remove_payload: MembershipRemovePayload,
}

impl<'a> RemoveMembersRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        team_id: impl Into<String>,
        membership_remove_payload: MembershipRemovePayload,
    ) -> Self {
        Self { client, org_id: org_id.into(), team_id: team_id.into(), membership_remove_payload }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/gateway/api/public/teams/v1/org/{}/teams/{}/members/remove",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.team_id)
            ),
        );

        let body = match serde_json::to_value(&self.membership_remove_payload)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MembershipRemoveResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
