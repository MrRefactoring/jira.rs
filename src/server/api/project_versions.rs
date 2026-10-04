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

    /// Returns all versions for the specified project. Results are paginated. Results can be ordered by the following fields: sequence, name, startDate, releaseDate.
    pub fn get_project_versions_paginated(
        &self,
        project_id_or_key: impl Into<String>,
    ) -> GetProjectVersionsPaginatedRequest<'a> {
        GetProjectVersionsPaginatedRequest::new(self.client, project_id_or_key)
    }

    /// Contains a full representation of a the specified project's versions.
    pub fn get_project_versions(&self, project_id_or_key: impl Into<String>) -> GetProjectVersionsRequest<'a> {
        GetProjectVersionsRequest::new(self.client, project_id_or_key)
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
    pub fn merge_versions(&self, id: impl Into<String>, move_issues_to: impl Into<String>) -> MergeVersionsRequest<'a> {
        MergeVersionsRequest::new(self.client, id, move_issues_to)
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
    pub fn delete_and_replace_version(
        &self,
        id: impl Into<String>,
        delete_and_replace_version: DeleteAndReplaceVersion,
    ) -> DeleteAndReplaceVersionRequest<'a> {
        DeleteAndReplaceVersionRequest::new(self.client, id, delete_and_replace_version)
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

/// Returns all versions for the specified project. Results are paginated. Results can be ordered by the following fields: sequence, name, startDate, releaseDate.
#[derive(Clone)]
pub struct GetProjectVersionsPaginatedRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    project_id_or_key: String,
    max_results: Option<i64>,
    order_by: Option<String>,
    start_at: Option<i64>,
}

impl<'a> GetProjectVersionsPaginatedRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
            expand: None,
            max_results: None,
            order_by: None,
            start_at: None,
        }
    }

    /// Parameters to expand
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// How many results on the page should be included. Defaults to 50
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Ordering of the results
    #[must_use]
    pub fn order_by(mut self, value: impl Into<String>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The page offset, if not specified then defaults to 0
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/version", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Version>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values.unwrap_or_default();
            Ok(crate::core::PageStep::offset(
                items,
                page.start_at.unwrap_or(position),
                page.is_last.unwrap_or(false),
                page.total,
                None,
            ))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RustPageVersion> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Contains a full representation of a the specified project's versions.
#[derive(Clone)]
pub struct GetProjectVersionsRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    project_id_or_key: String,
}

impl<'a> GetProjectVersionsRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), expand: None }
    }

    /// Parameters to expand
    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/versions", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Version>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Retrieve paginated collection of versions matching given query optionally filtered by given project IDs.
#[derive(Clone)]
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

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Version>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Version>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a version.
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/version/{}", crate::core::encode_path_segment(&self.id)),
        );

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
#[derive(Clone)]
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
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/version/{}", crate::core::encode_path_segment(&self.id)),
        );

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
#[derive(Clone)]
pub struct MergeVersionsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    move_issues_to: String,
}

impl<'a> MergeVersionsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, move_issues_to: impl Into<String>) -> Self {
        Self { client, id: id.into(), move_issues_to: move_issues_to.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/version/{}/mergeto/{}",
                crate::core::encode_path_segment(&self.id),
                crate::core::encode_path_segment(&self.move_issues_to)
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

/// Modify a version's sequence within a project.
/// The move version bean has 2 alternative field value pairs:
/// - position: An absolute position, which may have a value of 'First', 'Last', 'Earlier' or 'Later'
/// - after: A version to place this version after.  The value should be the self link of another version
#[derive(Clone)]
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
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/version/{}/move", crate::core::encode_path_segment(&self.id)),
        );

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
#[derive(Clone)]
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
            format!("/rest/api/2/version/{}/relatedIssueCounts", crate::core::encode_path_segment(&self.id)),
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
#[derive(Clone)]
pub struct DeleteAndReplaceVersionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    delete_and_replace_version: DeleteAndReplaceVersion,
}

impl<'a> DeleteAndReplaceVersionRequest<'a> {
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
            format!("/rest/api/2/version/{}/removeAndSwap", crate::core::encode_path_segment(&self.id)),
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
#[derive(Clone)]
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
            format!("/rest/api/2/version/{}/unresolvedIssueCount", crate::core::encode_path_segment(&self.id)),
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
#[derive(Clone)]
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
            format!("/rest/api/2/version/{}/remotelink", crate::core::encode_path_segment(&self.version_id)),
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
#[derive(Clone)]
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
            format!("/rest/api/2/version/{}/remotelink", crate::core::encode_path_segment(&self.version_id)),
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
#[derive(Clone)]
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
            format!("/rest/api/2/version/{}/remotelink", crate::core::encode_path_segment(&self.version_id)),
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
#[derive(Clone)]
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
            format!(
                "/rest/api/2/version/{}/remotelink/{}",
                crate::core::encode_path_segment(&self.version_id),
                crate::core::encode_path_segment(&self.global_id)
            ),
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
#[derive(Clone)]
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
            format!(
                "/rest/api/2/version/{}/remotelink/{}",
                crate::core::encode_path_segment(&self.version_id),
                crate::core::encode_path_segment(&self.global_id)
            ),
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
#[derive(Clone)]
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
            format!(
                "/rest/api/2/version/{}/remotelink/{}",
                crate::core::encode_path_segment(&self.version_id),
                crate::core::encode_path_segment(&self.global_id)
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
