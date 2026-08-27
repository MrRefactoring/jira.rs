// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectVersions operations.
pub struct ProjectVersionsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectVersionsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Retrieve paginated collection of versions matching given query optionally filtered by given project IDs.
    pub fn get_paginated_versions(&self) -> GetPaginatedVersionsRequest<'a> {
        GetPaginatedVersionsRequest::new(self.client)
    }

    /// Creates a version.
    pub fn create_version(&self, version: Version) -> CreateVersionRequest<'a> {
        CreateVersionRequest::new(self.client, version)
    }

    /// Returns the remote version links for a given global ID.
    pub fn get_remote_version_links(&self) -> GetRemoteVersionLinksRequest<'a> {
        GetRemoteVersionLinksRequest::new(self.client)
    }

    /// Returns a version.
    pub fn get_version(&self, id: impl Into<String>) -> GetVersionRequest<'a> {
        GetVersionRequest::new(self.client, id)
    }

    /// Updates a version.
    pub fn update_version(&self, id: impl Into<String>, body: Version) -> UpdateVersionRequest<'a> {
        UpdateVersionRequest::new(self.client, id, body)
    }

    /// Merge versions
    pub fn merge(&self, move_issues_to: impl Into<String>, id: impl Into<String>) -> MergeRequest<'a> {
        MergeRequest::new(self.client, move_issues_to, id)
    }

    /// Modify a version's sequence within a project.
    /// The move version bean has 2 alternative field value pairs:
    /// - position: An absolute position, which may have a value of 'First', 'Last', 'Earlier' or 'Later'
    /// - after: A version to place this version after.  The value should be the self link of another version
    pub fn move_version(&self, id: impl Into<String>, version_move: VersionMove) -> MoveVersionRequest<'a> {
        MoveVersionRequest::new(self.client, id, version_move)
    }

    /// Returns a bean containing the number of fixed in and affected issues for the given version.
    pub fn get_version_related_issues(&self, id: impl Into<String>) -> GetVersionRelatedIssuesRequest<'a> {
        GetVersionRelatedIssuesRequest::new(self.client, id)
    }

    /// Delete a project version, removed values will be replaced with ones specified by the parameters.
    pub fn delete_version_and_swap(
        &self,
        id: impl Into<String>,
        delete_and_replace_version: DeleteAndReplaceVersion,
    ) -> DeleteVersionAndSwapRequest<'a> {
        DeleteVersionAndSwapRequest::new(self.client, id, delete_and_replace_version)
    }

    /// Returns the number of unresolved issues for the given version
    pub fn get_version_unresolved_issues(&self, id: impl Into<String>) -> GetVersionUnresolvedIssuesRequest<'a> {
        GetVersionUnresolvedIssuesRequest::new(self.client, id)
    }

    /// Returns the remote version links associated with the given version ID.
    pub fn get_remote_version_links_by_version_id(
        &self,
        version_id: impl Into<String>,
    ) -> GetRemoteVersionLinksByVersionIdRequest<'a> {
        GetRemoteVersionLinksByVersionIdRequest::new(self.client, version_id)
    }

    /// Create a remote version link via POST. The link's global ID will be taken from the JSON payload if provided; otherwise, it will be generated.
    pub fn create_or_update_remote_version_link(
        &self,
        version_id: impl Into<String>,
        remote_entity_link_json: RemoteEntityLinkJson,
    ) -> CreateOrUpdateRemoteVersionLinkRequest<'a> {
        CreateOrUpdateRemoteVersionLinkRequest::new(self.client, version_id, remote_entity_link_json)
    }

    /// Delete all remote version links for a given version ID.
    pub fn delete_remote_version_links_by_version_id(
        &self,
        version_id: impl Into<String>,
    ) -> DeleteRemoteVersionLinksByVersionIdRequest<'a> {
        DeleteRemoteVersionLinksByVersionIdRequest::new(self.client, version_id)
    }

    /// Returns the remote version link associated with the given version ID and global ID.
    pub fn get_remote_version_link(
        &self,
        version_id: impl Into<String>,
        global_id: impl Into<String>,
    ) -> GetRemoteVersionLinkRequest<'a> {
        GetRemoteVersionLinkRequest::new(self.client, version_id, global_id)
    }

    /// Create a remote version link via POST using the provided global ID.
    pub fn create_or_update_remote_version_link_by_global_id(
        &self,
        version_id: impl Into<String>,
        global_id: impl Into<String>,
        remote_entity_link_json: RemoteEntityLinkJson,
    ) -> CreateOrUpdateRemoteVersionLinkByGlobalIdRequest<'a> {
        CreateOrUpdateRemoteVersionLinkByGlobalIdRequest::new(
            self.client,
            version_id,
            global_id,
            remote_entity_link_json,
        )
    }

    /// Delete a specific remote version link with the given version ID and global ID.
    pub fn delete_remote_version_link(
        &self,
        version_id: impl Into<String>,
        global_id: impl Into<String>,
    ) -> DeleteRemoteVersionLinkRequest<'a> {
        DeleteRemoteVersionLinkRequest::new(self.client, version_id, global_id)
    }
}

/// Retrieve paginated collection of versions matching given query optionally filtered by given project IDs.
pub struct GetPaginatedVersionsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    query: Option<String>,
    project_ids: Option<Vec<i64>>,
    start_at: Option<i64>,
}

impl<'a> GetPaginatedVersionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, query: None, project_ids: None, start_at: None }
    }

    /// maximum number of versions to return
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// string that version names will be matched with
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// set of project IDs to filter versions with
    #[must_use]
    pub fn project_ids(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.project_ids = Some(value.into_iter().collect());

        self
    }

    /// index of the first version to return
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/version".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_ids {
            config.query.push(("projectIds".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Version> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a version.
pub struct CreateVersionRequest<'a> {
    client: &'a crate::core::Client,
    version: Version,
}

impl<'a> CreateVersionRequest<'a> {
    fn new(client: &'a crate::core::Client, version: Version) -> Self {
        Self { client, version }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/version".to_owned());

        let body = match serde_json::to_value(&self.version)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Version> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the remote version links for a given global ID.
pub struct GetRemoteVersionLinksRequest<'a> {
    client: &'a crate::core::Client,
    global_id: Option<String>,
}

impl<'a> GetRemoteVersionLinksRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, global_id: None }
    }

    /// The id of the remote issue link to be returned.
    #[must_use]
    pub fn global_id(mut self, value: impl Into<String>) -> Self {
        self.global_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/version/remotelink".to_owned());

        if let Some(value) = &self.global_id {
            config.query.push(("globalId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RemoteEntityLinksJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a version.
pub struct GetVersionRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    id: String,
}

impl<'a> GetVersionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/2/version/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Version> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a version.
pub struct UpdateVersionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    body: Version,
}

impl<'a> UpdateVersionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, body: Version) -> Self {
        Self { client, id: id.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, format!("/rest/api/2/version/{}", self.id));

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Merge versions
pub struct MergeRequest<'a> {
    client: &'a crate::core::Client,
    move_issues_to: String,
    id: String,
}

impl<'a> MergeRequest<'a> {
    fn new(client: &'a crate::core::Client, move_issues_to: impl Into<String>, id: impl Into<String>) -> Self {
        Self { client, move_issues_to: move_issues_to.into(), id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/version/{}/mergeto/{}", self.id, self.move_issues_to),
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

/// Modify a version's sequence within a project.
/// The move version bean has 2 alternative field value pairs:
/// - position: An absolute position, which may have a value of 'First', 'Last', 'Earlier' or 'Later'
/// - after: A version to place this version after.  The value should be the self link of another version
pub struct MoveVersionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    version_move: VersionMove,
}

impl<'a> MoveVersionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, version_move: VersionMove) -> Self {
        Self { client, id: id.into(), version_move }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, format!("/rest/api/2/version/{}/move", self.id));

        let body = match serde_json::to_value(&self.version_move)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Version> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a bean containing the number of fixed in and affected issues for the given version.
pub struct GetVersionRelatedIssuesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetVersionRelatedIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/version/{}/relatedIssueCounts", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<VersionIssueCounts> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a project version, removed values will be replaced with ones specified by the parameters.
pub struct DeleteVersionAndSwapRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    delete_and_replace_version: DeleteAndReplaceVersion,
}

impl<'a> DeleteVersionAndSwapRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        id: impl Into<String>,
        delete_and_replace_version: DeleteAndReplaceVersion,
    ) -> Self {
        Self { client, id: id.into(), delete_and_replace_version }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/version/{}/removeAndSwap", self.id),
        );

        let body = match serde_json::to_value(&self.delete_and_replace_version)? {
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

/// Returns the number of unresolved issues for the given version
pub struct GetVersionUnresolvedIssuesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetVersionUnresolvedIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/version/{}/unresolvedIssueCount", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<VersionUnresolvedIssueCounts> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the remote version links associated with the given version ID.
pub struct GetRemoteVersionLinksByVersionIdRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
}

impl<'a> GetRemoteVersionLinksByVersionIdRequest<'a> {
    fn new(client: &'a crate::core::Client, version_id: impl Into<String>) -> Self {
        Self { client, version_id: version_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/version/{}/remotelink", self.version_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RemoteEntityLinksJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a remote version link via POST. The link's global ID will be taken from the JSON payload if provided; otherwise, it will be generated.
pub struct CreateOrUpdateRemoteVersionLinkRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
    remote_entity_link_json: RemoteEntityLinkJson,
}

impl<'a> CreateOrUpdateRemoteVersionLinkRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        version_id: impl Into<String>,
        remote_entity_link_json: RemoteEntityLinkJson,
    ) -> Self {
        Self { client, version_id: version_id.into(), remote_entity_link_json }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/version/{}/remotelink", self.version_id),
        );

        let body = match serde_json::to_value(&self.remote_entity_link_json)? {
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

/// Delete all remote version links for a given version ID.
pub struct DeleteRemoteVersionLinksByVersionIdRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
}

impl<'a> DeleteRemoteVersionLinksByVersionIdRequest<'a> {
    fn new(client: &'a crate::core::Client, version_id: impl Into<String>) -> Self {
        Self { client, version_id: version_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/version/{}/remotelink", self.version_id),
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

/// Returns the remote version link associated with the given version ID and global ID.
pub struct GetRemoteVersionLinkRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
    global_id: String,
}

impl<'a> GetRemoteVersionLinkRequest<'a> {
    fn new(client: &'a crate::core::Client, version_id: impl Into<String>, global_id: impl Into<String>) -> Self {
        Self { client, version_id: version_id.into(), global_id: global_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/version/{}/remotelink/{}", self.version_id, self.global_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RemoteEntityLinkJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a remote version link via POST using the provided global ID.
pub struct CreateOrUpdateRemoteVersionLinkByGlobalIdRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
    global_id: String,
    remote_entity_link_json: RemoteEntityLinkJson,
}

impl<'a> CreateOrUpdateRemoteVersionLinkByGlobalIdRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        version_id: impl Into<String>,
        global_id: impl Into<String>,
        remote_entity_link_json: RemoteEntityLinkJson,
    ) -> Self {
        Self { client, version_id: version_id.into(), global_id: global_id.into(), remote_entity_link_json }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/version/{}/remotelink/{}", self.version_id, self.global_id),
        );

        let body = match serde_json::to_value(&self.remote_entity_link_json)? {
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

/// Delete a specific remote version link with the given version ID and global ID.
pub struct DeleteRemoteVersionLinkRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
    global_id: String,
}

impl<'a> DeleteRemoteVersionLinkRequest<'a> {
    fn new(client: &'a crate::core::Client, version_id: impl Into<String>, global_id: impl Into<String>) -> Self {
        Self { client, version_id: version_id.into(), global_id: global_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/version/{}/remotelink/{}", self.version_id, self.global_id),
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
