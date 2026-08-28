// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by version description.
    ///  *  `name` Sorts by version name.
    ///  *  `releaseDate` Sorts by release date, starting with the oldest date. Versions with no release date are listed last.
    ///  *  `sequence` Sorts by the order of appearance in the user interface.
    ///  *  `startDate` Sorts by start date, starting with the oldest date. Versions with no start date are listed last.
    pub enum GetProjectVersionsPaginatedRequestOrderBy {
        Description => "description",
        DescriptionDescending => "-description",
        DescriptionAscending => "+description",
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
        ReleaseDate => "releaseDate",
        ReleaseDateDescending => "-releaseDate",
        ReleaseDateAscending => "+releaseDate",
        Sequence => "sequence",
        SequenceDescending => "-sequence",
        SequenceAscending => "+sequence",
        StartDate => "startDate",
        StartDateDescending => "-startDate",
        StartDateAscending => "+startDate",
    }
}

/// A list of status values used to filter the results by version status. This parameter accepts a comma-separated list. The status values are `released`, `unreleased`, and `archived`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetProjectVersionsPaginatedRequestStatus {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetProjectVersionsPaginatedRequestExpandValue {
        Issuesstatus => "issuesstatus",
        Operations => "operations",
        Driver => "driver",
        Approvers => "approvers",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `issuesstatus` Returns the number of issues in each status category for each version.
///  *  `operations` Returns actions that can be performed on the specified version.
///  *  `driver` Returns the Atlassian account ID of the version driver.
///  *  `approvers` Returns a list containing the approvers for this version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetProjectVersionsPaginatedRequestExpand {
    One(GetProjectVersionsPaginatedRequestExpandValue),
    Many(Vec<GetProjectVersionsPaginatedRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetProjectVersionsRequestExpandValue {
        Operations => "operations",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts `operations`, which returns actions that can be performed on the version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetProjectVersionsRequestExpand {
    One(GetProjectVersionsRequestExpandValue),
    Many(Vec<GetProjectVersionsRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetVersionRequestExpandValue {
        Operations => "operations",
        Issuesstatus => "issuesstatus",
        Driver => "driver",
        Approvers => "approvers",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `operations` Returns the list of operations available for this version.
///  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property represents the number of issues with a status other than *to do*, *in progress*, and *done*.
///  *  `driver` Returns the Atlassian account ID of the version driver.
///  *  `approvers` Returns a list containing the Atlassian account IDs of approvers for this version.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetVersionRequestExpand {
    One(GetVersionRequestExpandValue),
    Many(Vec<GetVersionRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The ProjectVersions operations.
pub struct ProjectVersionsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectVersionsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all versions in a project. See the [Get project versions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-versions-get) resource if you want to get a full list of versions without pagination.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn get_project_versions_paginated(
        &self,
        project_id_or_key: impl Into<String>,
    ) -> GetProjectVersionsPaginatedRequest<'a> {
        GetProjectVersionsPaginatedRequest::new(self.client, project_id_or_key)
    }

    /// Returns all versions in a project. The response is not paginated. Use [Get project versions paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-version-get) if you want to get the versions in a project with pagination.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn get_project_versions(&self, project_id_or_key: impl Into<String>) -> GetProjectVersionsRequest<'a> {
        GetProjectVersionsRequest::new(self.client, project_id_or_key)
    }

    /// Creates a project version.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the version is added to.
    pub fn create_version(&self, version: Version) -> CreateVersionRequest<'a> {
        CreateVersionRequest::new(self.client, version)
    }

    /// Returns a project version.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the version.
    pub fn get_version(&self, id: impl Into<String>) -> GetVersionRequest<'a> {
        GetVersionRequest::new(self.client, id)
    }

    /// Updates a project version.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.
    pub fn update_version(&self, id: impl Into<String>, body: Version) -> UpdateVersionRequest<'a> {
        UpdateVersionRequest::new(self.client, id, body)
    }

    /// Merges two project versions. The merge is completed by deleting the version specified in `id` and replacing any occurrences of its ID in `fixVersion` with the version ID specified in `moveIssuesTo`.
    ///
    /// Consider using [ Delete and replace version](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-version/#api-rest-api-3-version-id-removeAndSwap-post) instead. This resource supports swapping version values in `fixVersion`, `affectedVersion`, and custom fields.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.
    pub fn merge_versions(&self, id: impl Into<String>, move_issues_to: impl Into<String>) -> MergeVersionsRequest<'a> {
        MergeVersionsRequest::new(self.client, id, move_issues_to)
    }

    /// Modifies the version's sequence within the project, which affects the display order of the versions in Jira.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* project permission for the project that contains the version.
    pub fn move_version(&self, id: impl Into<String>, version_move: VersionMove) -> MoveVersionRequest<'a> {
        MoveVersionRequest::new(self.client, id, version_move)
    }

    /// Returns the following counts for a version:
    ///
    ///  *  Number of issues where the `fixVersion` is set to the version.
    ///  *  Number of issues where the `affectedVersion` is set to the version.
    ///  *  Number of issues where a version custom field is set to the version.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* project permission for the project that contains the version.
    pub fn get_version_related_issues(&self, id: impl Into<String>) -> GetVersionRelatedIssuesRequest<'a> {
        GetVersionRelatedIssuesRequest::new(self.client, id)
    }

    /// Returns related work items for the given version id.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the version.
    pub fn get_related_work(&self, id: impl Into<String>) -> GetRelatedWorkRequest<'a> {
        GetRelatedWorkRequest::new(self.client, id)
    }

    /// Creates a related work for the given version. You can only create a generic link type of related works via this API. relatedWorkId will be auto-generated UUID, that does not need to be provided.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.
    pub fn create_related_work(
        &self,
        id: impl Into<String>,
        version_related_work: VersionRelatedWork,
    ) -> CreateRelatedWorkRequest<'a> {
        CreateRelatedWorkRequest::new(self.client, id, version_related_work)
    }

    /// Updates the given related work. You can only update generic link related works via Rest APIs. Any archived version related works can't be edited.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.
    pub fn update_related_work(
        &self,
        id: impl Into<String>,
        version_related_work: VersionRelatedWork,
    ) -> UpdateRelatedWorkRequest<'a> {
        UpdateRelatedWorkRequest::new(self.client, id, version_related_work)
    }

    /// Deletes a project version.
    ///
    /// Alternative versions can be provided to update issues that use the deleted version in `fixVersion`, `affectedVersion`, or any version picker custom fields. If alternatives are not provided, occurrences of `fixVersion`, `affectedVersion`, and any version picker custom field, that contain the deleted version, are cleared. Any replacement version must be in the same project as the version being deleted and cannot be the version being deleted.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.
    pub fn delete_and_replace_version(
        &self,
        id: impl Into<String>,
        delete_and_replace_version: DeleteAndReplaceVersion,
    ) -> DeleteAndReplaceVersionRequest<'a> {
        DeleteAndReplaceVersionRequest::new(self.client, id, delete_and_replace_version)
    }

    /// Returns counts of the issues and unresolved issues for the project version.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* project permission for the project that contains the version.
    pub fn get_version_unresolved_issues(&self, id: impl Into<String>) -> GetVersionUnresolvedIssuesRequest<'a> {
        GetVersionUnresolvedIssuesRequest::new(self.client, id)
    }

    /// Deletes the given related work for the given version.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.
    pub fn delete_related_work(
        &self,
        version_id: impl Into<String>,
        related_work_id: impl Into<String>,
    ) -> DeleteRelatedWorkRequest<'a> {
        DeleteRelatedWorkRequest::new(self.client, version_id, related_work_id)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all versions in a project. See the [Get project versions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-versions-get) resource if you want to get a full list of versions without pagination.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
pub struct GetProjectVersionsPaginatedRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    start_at: Option<i64>,
    max_results: Option<i64>,
    order_by: Option<GetProjectVersionsPaginatedRequestOrderBy>,
    query: Option<String>,
    status: Option<GetProjectVersionsPaginatedRequestStatus>,
    expand: Option<GetProjectVersionsPaginatedRequestExpand>,
}

impl<'a> GetProjectVersionsPaginatedRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
            start_at: None,
            max_results: None,
            order_by: None,
            query: None,
            status: None,
            expand: None,
        }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by version description.
    ///  *  `name` Sorts by version name.
    ///  *  `releaseDate` Sorts by release date, starting with the oldest date. Versions with no release date are listed last.
    ///  *  `sequence` Sorts by the order of appearance in the user interface.
    ///  *  `startDate` Sorts by start date, starting with the oldest date. Versions with no start date are listed last.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetProjectVersionsPaginatedRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// Filter the results using a literal string. Versions with matching `name` or `description` are returned (case insensitive).
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// A list of status values used to filter the results by version status. This parameter accepts a comma-separated list. The status values are `released`, `unreleased`, and `archived`.
    #[must_use]
    pub fn status(mut self, value: GetProjectVersionsPaginatedRequestStatus) -> Self {
        self.status = Some(value);

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `issuesstatus` Returns the number of issues in each status category for each version.
    ///  *  `operations` Returns actions that can be performed on the specified version.
    ///  *  `driver` Returns the Atlassian account ID of the version driver.
    ///  *  `approvers` Returns a list containing the approvers for this version.
    #[must_use]
    pub fn expand(mut self, value: GetProjectVersionsPaginatedRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/version", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.status {
            config.query.push(("status".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
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

/// Returns all versions in a project. The response is not paginated. Use [Get project versions paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-version-get) if you want to get the versions in a project with pagination.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
pub struct GetProjectVersionsRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    expand: Option<GetProjectVersionsRequestExpand>,
}

impl<'a> GetProjectVersionsRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts `operations`, which returns actions that can be performed on the version.
    #[must_use]
    pub fn expand(mut self, value: GetProjectVersionsRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/versions", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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

/// Creates a project version.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project the version is added to.
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
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/version".to_owned());

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

/// Returns a project version.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the version.
pub struct GetVersionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    expand: Option<GetVersionRequestExpand>,
}

impl<'a> GetVersionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `operations` Returns the list of operations available for this version.
    ///  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property represents the number of issues with a status other than *to do*, *in progress*, and *done*.
    ///  *  `driver` Returns the Atlassian account ID of the version driver.
    ///  *  `approvers` Returns a list containing the Atlassian account IDs of approvers for this version.
    #[must_use]
    pub fn expand(mut self, value: GetVersionRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/version/{}", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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

/// Updates a project version.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.
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
            format!("/rest/api/3/version/{}", crate::core::encode_path_segment(&self.id)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Merges two project versions. The merge is completed by deleting the version specified in `id` and replacing any occurrences of its ID in `fixVersion` with the version ID specified in `moveIssuesTo`.
///
/// Consider using [ Delete and replace version](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-version/#api-rest-api-3-version-id-removeAndSwap-post) instead. This resource supports swapping version values in `fixVersion`, `affectedVersion`, and custom fields.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.
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
                "/rest/api/3/version/{}/mergeto/{}",
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

/// Modifies the version's sequence within the project, which affects the display order of the versions in Jira.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* project permission for the project that contains the version.
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
            format!("/rest/api/3/version/{}/move", crate::core::encode_path_segment(&self.id)),
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

/// Returns the following counts for a version:
///
///  *  Number of issues where the `fixVersion` is set to the version.
///  *  Number of issues where the `affectedVersion` is set to the version.
///  *  Number of issues where a version custom field is set to the version.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* project permission for the project that contains the version.
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
            format!("/rest/api/3/version/{}/relatedIssueCounts", crate::core::encode_path_segment(&self.id)),
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

/// Returns related work items for the given version id.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the version.
pub struct GetRelatedWorkRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetRelatedWorkRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/version/{}/relatedwork", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<VersionRelatedWork>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a related work for the given version. You can only create a generic link type of related works via this API. relatedWorkId will be auto-generated UUID, that does not need to be provided.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.
pub struct CreateRelatedWorkRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    version_related_work: VersionRelatedWork,
}

impl<'a> CreateRelatedWorkRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, version_related_work: VersionRelatedWork) -> Self {
        Self { client, id: id.into(), version_related_work }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/version/{}/relatedwork", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.version_related_work)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<VersionRelatedWork> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the given related work. You can only update generic link related works via Rest APIs. Any archived version related works can't be edited.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.
pub struct UpdateRelatedWorkRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    version_related_work: VersionRelatedWork,
}

impl<'a> UpdateRelatedWorkRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, version_related_work: VersionRelatedWork) -> Self {
        Self { client, id: id.into(), version_related_work }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/version/{}/relatedwork", crate::core::encode_path_segment(&self.id)),
        );

        let body = match serde_json::to_value(&self.version_related_work)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<VersionRelatedWork> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a project version.
///
/// Alternative versions can be provided to update issues that use the deleted version in `fixVersion`, `affectedVersion`, or any version picker custom fields. If alternatives are not provided, occurrences of `fixVersion`, `affectedVersion`, and any version picker custom field, that contain the deleted version, are cleared. Any replacement version must be in the same project as the version being deleted and cannot be the version being deleted.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that contains the version.
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
            format!("/rest/api/3/version/{}/removeAndSwap", crate::core::encode_path_segment(&self.id)),
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

/// Returns counts of the issues and unresolved issues for the project version.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* project permission for the project that contains the version.
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
            format!("/rest/api/3/version/{}/unresolvedIssueCount", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<VersionUnresolvedIssuesCount> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes the given related work for the given version.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Resolve issues:* and *Edit issues* [Managing project permissions](https://confluence.atlassian.com/adminjiraserver/managing-project-permissions-938847145.html) for the project that contains the version.
pub struct DeleteRelatedWorkRequest<'a> {
    client: &'a crate::core::Client,
    version_id: String,
    related_work_id: String,
}

impl<'a> DeleteRelatedWorkRequest<'a> {
    fn new(client: &'a crate::core::Client, version_id: impl Into<String>, related_work_id: impl Into<String>) -> Self {
        Self { client, version_id: version_id.into(), related_work_id: related_work_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/version/{}/relatedwork/{}",
                crate::core::encode_path_segment(&self.version_id),
                crate::core::encode_path_segment(&self.related_work_id)
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
