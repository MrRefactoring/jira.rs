// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    pub enum GetNotificationSchemeForProjectRequestExpandValue {
        Group => "group",
        User => "user",
        ProjectRole => "projectRole",
        Field => "field",
    }
}

/// The Projects operations.
pub struct ProjectsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all projects which are visible for the currently logged in user. If no user is logged in, it returns the list of projects that are visible when using anonymous access.
    pub fn search_projects(&self) -> SearchProjectsRequest<'a> {
        SearchProjectsRequest::new(self.client)
    }

    /// Creates a new project
    pub fn create_project(&self, project_input: ProjectInput) -> CreateProjectRequest<'a> {
        CreateProjectRequest::new(self.client, project_input)
    }

    /// Returns a full representation of a project in JSON format. All project keys associated with the project will only be returned if `expand=projectKeys`.
    pub fn get_project(&self, project_id_or_key: impl Into<String>) -> GetProjectRequest<'a> {
        GetProjectRequest::new(self.client, project_id_or_key)
    }

    /// Updates a project. Only non null values sent in JSON will be updated in the project. Values available for the assigneeType field are: "PROJECT_LEAD" and "UNASSIGNED".
    pub fn update_project(
        &self,
        project_id_or_key: impl Into<String>,
        update_project_details: UpdateProjectDetails,
    ) -> UpdateProjectRequest<'a> {
        UpdateProjectRequest::new(self.client, project_id_or_key, update_project_details)
    }

    /// Deletes a project
    pub fn delete_project(&self, project_id_or_key: impl Into<String>) -> DeleteProjectRequest<'a> {
        DeleteProjectRequest::new(self.client, project_id_or_key)
    }

    /// Archives a project
    pub fn archive_project(&self, project_id_or_key: impl Into<String>) -> ArchiveProjectRequest<'a> {
        ArchiveProjectRequest::new(self.client, project_id_or_key)
    }

    /// Converts the temporary avatar into the final one. This is step 2/3 of changing an avatar for a project:
    /// - Upload (store temporary avatar)
    /// - Crop (create avatar from temporary)
    /// - Update (update project avatar)
    pub fn create_project_avatar_from_temporary(
        &self,
        project_id_or_key: impl Into<String>,
        avatar_cropping: AvatarCropping,
    ) -> CreateProjectAvatarFromTemporaryRequest<'a> {
        CreateProjectAvatarFromTemporaryRequest::new(self.client, project_id_or_key, avatar_cropping)
    }

    /// Creates temporary avatar using multipart. The response is sent back as JSON stored in a textarea. This is because
    /// the client uses remote iframing to submit avatars using multipart. So we must send them a valid HTML page back from
    /// which the client parses the JSON.
    pub fn store_temporary_project_avatar_using_multi_part(
        &self,
        project_id_or_key: impl Into<String>,
        avatar: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> StoreTemporaryProjectAvatarUsingMultiPartRequest<'a> {
        StoreTemporaryProjectAvatarUsingMultiPartRequest::new(self.client, project_id_or_key, avatar)
    }

    /// Restores an archived project. In case of success restored project should be re-indexed.
    pub fn restore_project(&self, project_id_or_key: impl Into<String>) -> RestoreProjectRequest<'a> {
        RestoreProjectRequest::new(self.client, project_id_or_key)
    }

    /// Get all issue types with valid status values for a project
    pub fn get_all_statuses(&self, project_id_or_key: impl Into<String>) -> GetAllStatusesRequest<'a> {
        GetAllStatusesRequest::new(self.client, project_id_or_key)
    }

    /// Updates the type of a project
    pub fn update_project_type(
        &self,
        project_id_or_key: impl Into<String>,
        new_project_type_key: impl Into<String>,
    ) -> UpdateProjectTypeRequest<'a> {
        UpdateProjectTypeRequest::new(self.client, project_id_or_key, new_project_type_key)
    }

    /// Gets a notification scheme associated with the project. Follow the documentation of /notificationscheme/{id} resource for all details about returned value.
    pub fn get_notification_scheme_for_project(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetNotificationSchemeForProjectRequest<'a> {
        GetNotificationSchemeForProjectRequest::new(self.client, project_key_or_id)
    }

    /// Gets a full representation of a priority scheme in JSON format used by specified project. User must be global administrator or project administrator. All project keys associated with the priority scheme will only be returned if additional query parameter is provided expand=projectKeys.
    pub fn get_assigned_priority_scheme(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetAssignedPrioritySchemeRequest<'a> {
        GetAssignedPrioritySchemeRequest::new(self.client, project_key_or_id)
    }

    /// Assigns project with priority scheme. Priority scheme assign with migration is possible from the UI. Operation will fail if migration is needed as a result of operation eg. there are issues with priorities invalid in the destination scheme. All project keys associated with the priority scheme will only be returned if additional query parameter is provided expand=projectKeys.
    pub fn assign_priority_scheme(
        &self,
        project_key_or_id: impl Into<String>,
        id: Id,
    ) -> AssignPrioritySchemeRequest<'a> {
        AssignPrioritySchemeRequest::new(self.client, project_key_or_id, id)
    }

    /// Unassigns project from priority scheme. Operation will fail for defualt priority scheme, project is not found or project is not associated with provided priority scheme. All project keys associated with the priority scheme will only be returned if additional query parameter is provided expand=projectKeys.
    pub fn unassign_priority_scheme(
        &self,
        project_key_or_id: impl Into<String>,
        scheme_id: i64,
    ) -> UnassignPrioritySchemeRequest<'a> {
        UnassignPrioritySchemeRequest::new(self.client, project_key_or_id, scheme_id)
    }

    /// Returns the workflow scheme that is associated with requested project.
    pub fn get_workflow_scheme_for_project(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetWorkflowSchemeForProjectRequest<'a> {
        GetWorkflowSchemeForProjectRequest::new(self.client, project_key_or_id)
    }

    /// Returns a list of projects visible to the user where project name and/or key is matching the given query.
    /// Passing an empty (or whitespace only) query will match no projects. The project matches will
    /// contain a field with the query highlighted.
    /// The number of projects returned can be controlled by passing a value for 'maxResults', but a hard limit of no
    /// more than 100 projects is enforced. The projects are wrapped in a single response object that contains
    /// a header for use in the picker, specifically 'Showing X of Y matching projects' and the total number
    /// of matches for the query.
    pub fn get_projects_picker(&self) -> GetProjectsPickerRequest<'a> {
        GetProjectsPickerRequest::new(self.client)
    }
}

/// Returns all projects which are visible for the currently logged in user. If no user is logged in, it returns the list of projects that are visible when using anonymous access.
#[derive(Clone)]
pub struct SearchProjectsRequest<'a> {
    client: &'a crate::core::Client,
    include_archived: Option<bool>,
    expand: Option<Vec<String>>,
    recent: Option<i64>,
    browse_archive: Option<bool>,
}

impl<'a> SearchProjectsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, include_archived: None, expand: None, recent: None, browse_archive: None }
    }

    /// Whether to include archived projects in response, default: false
    #[must_use]
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);

        self
    }

    /// Parameters to expand This parameter is a comma-separated list.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// If this parameter is set then only projects recently accessed by the current user (if not logged in then based on HTTP session) will be returned (maximum count limited to the specified number but no more than 20)
    #[must_use]
    pub fn recent(mut self, value: i64) -> Self {
        self.recent = Some(value);

        self
    }

    /// Whether to include only projects where current user can browse archive
    #[must_use]
    pub fn browse_archive(mut self, value: bool) -> Self {
        self.browse_archive = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/project".to_owned());

        if let Some(value) = &self.include_archived {
            config.query.push(("includeArchived".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.recent {
            config.query.push(("recent".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.browse_archive {
            config.query.push(("browseArchive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Project>> {
        let first = ();

        crate::core::stream_pages(self, first, |request, _position| async move {
            let page = request.send().await?;
            let items = page;
            Ok(crate::core::PageStep::new(items, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Project>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a new project
#[derive(Clone)]
pub struct CreateProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_input: ProjectInput,
}

impl<'a> CreateProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_input: ProjectInput) -> Self {
        Self { client, project_input }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/project".to_owned());

        let body = match serde_json::to_value(&self.project_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectIdentifiers> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a full representation of a project in JSON format. All project keys associated with the project will only be returned if `expand=projectKeys`.
#[derive(Clone)]
pub struct GetProjectRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    project_id_or_key: String,
}

impl<'a> GetProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), expand: None }
    }

    /// Parameters to expand This parameter is a comma-separated list.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Project> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a project. Only non null values sent in JSON will be updated in the project. Values available for the assigneeType field are: "PROJECT_LEAD" and "UNASSIGNED".
#[derive(Clone)]
pub struct UpdateProjectRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    project_id_or_key: String,
    update_project_details: UpdateProjectDetails,
}

impl<'a> UpdateProjectRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        update_project_details: UpdateProjectDetails,
    ) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), update_project_details, expand: None }
    }

    /// Parameters to expand This parameter is a comma-separated list.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/project/{}", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.update_project_details)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Project> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a project
#[derive(Clone)]
pub struct DeleteProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> DeleteProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/project/{}", crate::core::encode_path_segment(&self.project_id_or_key)),
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

/// Archives a project
#[derive(Clone)]
pub struct ArchiveProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> ArchiveProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/project/{}/archive", crate::core::encode_path_segment(&self.project_id_or_key)),
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

/// Converts the temporary avatar into the final one. This is step 2/3 of changing an avatar for a project:
/// - Upload (store temporary avatar)
/// - Crop (create avatar from temporary)
/// - Update (update project avatar)
#[derive(Clone)]
pub struct CreateProjectAvatarFromTemporaryRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    avatar_cropping: AvatarCropping,
}

impl<'a> CreateProjectAvatarFromTemporaryRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        avatar_cropping: AvatarCropping,
    ) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), avatar_cropping }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/project/{}/avatar", crate::core::encode_path_segment(&self.project_id_or_key)),
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

/// Creates temporary avatar using multipart. The response is sent back as JSON stored in a textarea. This is because
/// the client uses remote iframing to submit avatars using multipart. So we must send them a valid HTML page back from
/// which the client parses the JSON.
#[derive(Clone)]
pub struct StoreTemporaryProjectAvatarUsingMultiPartRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    avatar: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> StoreTemporaryProjectAvatarUsingMultiPartRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        avatar: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
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
            format!(
                "/rest/api/2/project/{}/avatar/temporary",
                crate::core::encode_path_segment(&self.project_id_or_key)
            ),
        );

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body =
            Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("avatar", self.avatar.clone())));

        config.content_type = self.content_type.clone().or(None);

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Restores an archived project. In case of success restored project should be re-indexed.
#[derive(Clone)]
pub struct RestoreProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> RestoreProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/project/{}/restore", crate::core::encode_path_segment(&self.project_id_or_key)),
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

/// Get all issue types with valid status values for a project
#[derive(Clone)]
pub struct GetAllStatusesRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> GetAllStatusesRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/statuses", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<IssueTypeWithStatus>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the type of a project
#[derive(Clone)]
pub struct UpdateProjectTypeRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    new_project_type_key: String,
}

impl<'a> UpdateProjectTypeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        new_project_type_key: impl Into<String>,
    ) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), new_project_type_key: new_project_type_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/project/{}/type/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                crate::core::encode_path_segment(&self.new_project_type_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Project> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Gets a notification scheme associated with the project. Follow the documentation of /notificationscheme/{id} resource for all details about returned value.
#[derive(Clone)]
pub struct GetNotificationSchemeForProjectRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<GetNotificationSchemeForProjectRequestExpandValue>>,
    project_key_or_id: String,
}

impl<'a> GetNotificationSchemeForProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), expand: None }
    }

    /// Optional information to be expanded in the response: group, user, projectRole or field. This parameter accepts a comma-separated list.
    #[must_use]
    pub fn expand(
        mut self,
        value: impl IntoIterator<Item = impl Into<GetNotificationSchemeForProjectRequestExpandValue>>,
    ) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/project/{}/notificationscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push((
                "expand".to_owned(),
                crate::core::QueryValue::Scalar(value.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")),
            ));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<NotificationScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Gets a full representation of a priority scheme in JSON format used by specified project. User must be global administrator or project administrator. All project keys associated with the priority scheme will only be returned if additional query parameter is provided expand=projectKeys.
#[derive(Clone)]
pub struct GetAssignedPrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
}

impl<'a> GetAssignedPrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/priorityscheme", crate::core::encode_path_segment(&self.project_key_or_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PriorityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Assigns project with priority scheme. Priority scheme assign with migration is possible from the UI. Operation will fail if migration is needed as a result of operation eg. there are issues with priorities invalid in the destination scheme. All project keys associated with the priority scheme will only be returned if additional query parameter is provided expand=projectKeys.
#[derive(Clone)]
pub struct AssignPrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
    id: Id,
}

impl<'a> AssignPrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>, id: Id) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/project/{}/priorityscheme", crate::core::encode_path_segment(&self.project_key_or_id)),
        );

        let body = match serde_json::to_value(&self.id)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PriorityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Unassigns project from priority scheme. Operation will fail for defualt priority scheme, project is not found or project is not associated with provided priority scheme. All project keys associated with the priority scheme will only be returned if additional query parameter is provided expand=projectKeys.
#[derive(Clone)]
pub struct UnassignPrioritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
    scheme_id: i64,
}

impl<'a> UnassignPrioritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>, scheme_id: i64) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), scheme_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/project/{}/priorityscheme/{}",
                crate::core::encode_path_segment(&self.project_key_or_id),
                self.scheme_id
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PriorityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the workflow scheme that is associated with requested project.
#[derive(Clone)]
pub struct GetWorkflowSchemeForProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
}

impl<'a> GetWorkflowSchemeForProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/workflowscheme", crate::core::encode_path_segment(&self.project_key_or_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of projects visible to the user where project name and/or key is matching the given query.
/// Passing an empty (or whitespace only) query will match no projects. The project matches will
/// contain a field with the query highlighted.
/// The number of projects returned can be controlled by passing a value for 'maxResults', but a hard limit of no
/// more than 100 projects is enforced. The projects are wrapped in a single response object that contains
/// a header for use in the picker, specifically 'Showing X of Y matching projects' and the total number
/// of matches for the query.
#[derive(Clone)]
pub struct GetProjectsPickerRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    query: Option<String>,
    allow_empty_query: Option<bool>,
}

impl<'a> GetProjectsPickerRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, query: None, allow_empty_query: None }
    }

    /// Maximum number of matches to return. Zero means a default limit of 100 and negative numbers return no results.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// A sequence of characters expected to be found in the word-prefix of project name and/or key.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// If true, and the query is empty, the method will return first results limited to the value of 'maxResults' or default limit of 100.
    #[must_use]
    pub fn allow_empty_query(mut self, value: bool) -> Self {
        self.allow_empty_query = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/projects/picker".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.allow_empty_query {
            config.query.push(("allowEmptyQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectPickerResultWrapper> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
