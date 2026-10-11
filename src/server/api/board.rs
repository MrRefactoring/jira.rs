// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// Filters results to sprints in specified states. Valid values: future, active, closed. You can define multiple states separated by commas, e.g. state=active,closed
    pub enum GetAllSprintsRequestState {
        Future => "future",
        Active => "active",
        Closed => "closed",
    }
}

/// The Board operations.
pub struct BoardService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> BoardService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all boards. This only includes boards that the user has permission to view.
    pub fn get_all_boards(&self) -> GetAllBoardsRequest<'a> {
        GetAllBoardsRequest::new(self.client)
    }

    /// Creates a new board. Board name, type and filter Id is required.
    /// - name - Must be less than 255 characters.
    /// - type - Valid values: scrum, kanban
    /// - filterId - Id of a filter that the user has permissions to view. Note, if the user does not have the 'Create shared objects' permission and tries to create a shared board, a private board will be created instead (remember that board sharing depends on the filter sharing).
    /// Note:
    /// - If you want to create a new project with an associated board, use the JIRA platform REST API. For more information, see the Create project method. The projectTypeKey for software boards must be 'software' and the projectTemplateKey must be either com.pyxis.greenhopper.jira:gh-kanban-template or com.pyxis.greenhopper.jira:gh-scrum-template.
    /// - You can create a filter using the JIRA REST API. For more information, see the Create filter method.
    /// - If you do not ORDER BY the Rank field for the filter of your board, you will not be able to reorder issues on the board.
    pub fn create_board(&self, board_create: BoardCreate) -> CreateBoardRequest<'a> {
        CreateBoardRequest::new(self.client, board_create)
    }

    /// Returns a single board, for a given board Id.
    pub fn get_board(&self, board_id: i64) -> GetBoardRequest<'a> {
        GetBoardRequest::new(self.client, board_id)
    }

    /// Deletes the board.
    pub fn delete_board(&self, board_id: i64) -> DeleteBoardRequest<'a> {
        DeleteBoardRequest::new(self.client, board_id)
    }

    /// Returns all issues from a board's backlog, for a given board Id.
    pub fn get_issues_for_backlog(&self, board_id: i64) -> GetIssuesForBacklogRequest<'a> {
        GetIssuesForBacklogRequest::new(self.client, board_id)
    }

    /// Get the board configuration.
    /// The response contains the following fields:
    /// - id - Id of the board.
    /// - name - Name of the board.
    /// - filter - Reference to the filter used by the given board.
    /// - subQuery (Kanban only) - JQL subquery used by the given board.
    /// - columnConfig - The column configuration lists the columns for the board, in the order defined in the column configuration.
    /// For each column, it shows the issue status mapping
    /// as well as the constraint type (Valid values: none, issueCount, issueCountExclSubs) for the min/max number of issues.
    /// Note, the last column with statuses mapped to it is treated as the "Done" column,
    /// which means that issues in that column will be marked as already completed.
    /// - estimation (Scrum only) - Contains information about type of estimation used for the board. Valid values: none, issueCount, field.
    /// If the estimation type is "field", the Id and display name of the field used for estimation is also returned.
    /// Note, estimates for an issue can be updated by a PUT /rest/api/2/issue/{issueIdOrKey} request, however the fields must be on the screen.
    /// "timeoriginalestimate" field will never be on the screen, so in order to update it "originalEstimate" in "timetracking" field should be updated.
    /// - ranking - Contains information about custom field used for ranking in the given board.
    pub fn get_configuration(&self, board_id: i64) -> BoardGetConfigurationRequest<'a> {
        BoardGetConfigurationRequest::new(self.client, board_id)
    }

    /// Returns all epics from the board, for the given board Id. This only includes epics that the user has permission to view. Note, if the user does not have permission to view the board, no epics will be returned at all.
    pub fn get_epics(&self, board_id: i64) -> GetEpicsRequest<'a> {
        GetEpicsRequest::new(self.client, board_id)
    }

    /// Returns all issues that do not belong to any epic on a board, for a given board Id.
    pub fn get_issues_without_epic_for_board(&self, board_id: i64) -> GetIssuesWithoutEpicForBoardRequest<'a> {
        GetIssuesWithoutEpicForBoardRequest::new(self.client, board_id)
    }

    /// Returns all issues that belong to an epic on the board, for the given epic Id and the board Id.
    pub fn get_board_issues_for_epic(&self, board_id: i64, epic_id: i64) -> GetBoardIssuesForEpicRequest<'a> {
        GetBoardIssuesForEpicRequest::new(self.client, board_id, epic_id)
    }

    /// Returns all issues from a board, for a given board Id. This only includes issues that the user has permission to view. Note, if the user does not have permission to view the board, no issues will be returned at all. Issues returned from this resource include Agile fields, like sprint, closedSprints, flagged, and epic. By default, the returned issues are ordered by rank.
    pub fn get_issues_for_board(&self, board_id: i64) -> GetIssuesForBoardRequest<'a> {
        GetIssuesForBoardRequest::new(self.client, board_id)
    }

    /// Returns all projects that are associated with the board, for the given board Id. A project is associated with a board only if the board filter explicitly filters issues by the project and guaranties that all issues will come for one of those projects e.g. board's filter with "project in (PR-1, PR-1) OR reporter = admin" jql Projects are returned only if user can browse all projects that are associated with the board. Note, if the user does not have permission to view the board, no projects will be returned at all. Returned projects are ordered by the name.
    pub fn get_projects(&self, board_id: i64) -> GetProjectsRequest<'a> {
        GetProjectsRequest::new(self.client, board_id)
    }

    /// Returns the keys of all properties for the board identified by the id. The user who retrieves the property keys is required to have permissions to view the board.
    pub fn get_board_property_keys(&self, board_id: impl Into<String>) -> GetBoardPropertyKeysRequest<'a> {
        GetBoardPropertyKeysRequest::new(self.client, board_id)
    }

    /// Returns the value of the property with a given key from the board identified by the provided id. The user who retrieves the property is required to have permissions to view the board.
    pub fn get_board_property(
        &self,
        board_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetBoardPropertyRequest<'a> {
        GetBoardPropertyRequest::new(self.client, board_id, property_key)
    }

    /// Sets the value of the specified board's property. You can use this resource to store a custom data against the board identified by the id. The user who stores the data is required to have permissions to modify the board.
    pub fn set_board_property(
        &self,
        board_id: impl Into<String>,
        property_key: impl Into<String>,
        property_value: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetBoardPropertyRequest<'a> {
        SetBoardPropertyRequest::new(self.client, board_id, property_key, property_value)
    }

    /// Removes the property from the board identified by the id. Ths user removing the property is required to have permissions to modify the board.
    pub fn delete_board_property(
        &self,
        board_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteBoardPropertyRequest<'a> {
        DeleteBoardPropertyRequest::new(self.client, board_id, property_key)
    }

    /// Returns the value of the setting for refined velocity chart
    pub fn get_refined_velocity(&self, board_id: i64) -> GetRefinedVelocityRequest<'a> {
        GetRefinedVelocityRequest::new(self.client, board_id)
    }

    /// Sets the value of the specified board's refined velocity setting.
    pub fn set_refined_velocity(
        &self,
        board_id: i64,
        boolean_setting: BooleanSetting,
    ) -> SetRefinedVelocityRequest<'a> {
        SetRefinedVelocityRequest::new(self.client, board_id, boolean_setting)
    }

    /// Returns all sprints from a board, for a given board Id. This only includes sprints that the user has permission to view.
    pub fn get_all_sprints(&self, board_id: i64) -> GetAllSprintsRequest<'a> {
        GetAllSprintsRequest::new(self.client, board_id)
    }

    /// Get all issues you have access to that belong to the sprint from the board. Issue returned from this resource contains additional fields like: sprint, closedSprints, flagged and epic. Issues are returned ordered by rank. JQL order has higher priority than default rank.
    pub fn get_board_issues_for_sprint(&self, board_id: i64, sprint_id: i64) -> GetBoardIssuesForSprintRequest<'a> {
        GetBoardIssuesForSprintRequest::new(self.client, board_id, sprint_id)
    }

    /// Returns all versions from a board, for a given board Id. This only includes versions that the user has permission to view. Note, if the user does not have permission to view the board, no versions will be returned at all. Returned versions are ordered by the name of the project from which they belong and then by sequence defined by user.
    pub fn get_all_versions(&self, board_id: i64) -> GetAllVersionsRequest<'a> {
        GetAllVersionsRequest::new(self.client, board_id)
    }
}

/// Returns all boards. This only includes boards that the user has permission to view.
#[derive(Clone)]
pub struct GetAllBoardsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    name: Option<String>,
    project_key_or_id: Option<String>,
    r#type: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> GetAllBoardsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, name: None, project_key_or_id: None, r#type: None, start_at: None }
    }

    /// The maximum number of boards to return per page. Default: 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Filters results to boards that match or partially match the specified name.
    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());

        self
    }

    /// Filters results to boards that are relevant to a project.
    #[must_use]
    pub fn project_key_or_id(mut self, value: impl Into<String>) -> Self {
        self.project_key_or_id = Some(value.into());

        self
    }

    /// Filters results to boards of the specified type. Valid values: scrum, kanban.
    #[must_use]
    pub fn r#type(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.r#type = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned boards. Base index: 0.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/agile/1.0/board".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.name {
            config.query.push(("name".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_key_or_id {
            config.query.push(("projectKeyOrId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.r#type {
            config.query.push(("type".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Board>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Board>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a new board. Board name, type and filter Id is required.
/// - name - Must be less than 255 characters.
/// - type - Valid values: scrum, kanban
/// - filterId - Id of a filter that the user has permissions to view. Note, if the user does not have the 'Create shared objects' permission and tries to create a shared board, a private board will be created instead (remember that board sharing depends on the filter sharing).
/// Note:
/// - If you want to create a new project with an associated board, use the JIRA platform REST API. For more information, see the Create project method. The projectTypeKey for software boards must be 'software' and the projectTemplateKey must be either com.pyxis.greenhopper.jira:gh-kanban-template or com.pyxis.greenhopper.jira:gh-scrum-template.
/// - You can create a filter using the JIRA REST API. For more information, see the Create filter method.
/// - If you do not ORDER BY the Rank field for the filter of your board, you will not be able to reorder issues on the board.
#[derive(Clone)]
pub struct CreateBoardRequest<'a> {
    client: &'a crate::core::Client,
    board_create: BoardCreate,
}

impl<'a> CreateBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, board_create: BoardCreate) -> Self {
        Self { client, board_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/agile/1.0/board".to_owned());

        let body = match serde_json::to_value(&self.board_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Board> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a single board, for a given board Id.
#[derive(Clone)]
pub struct GetBoardRequest<'a> {
    client: &'a crate::core::Client,
    board_id: i64,
}

impl<'a> GetBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}", self.board_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Board> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes the board.
#[derive(Clone)]
pub struct DeleteBoardRequest<'a> {
    client: &'a crate::core::Client,
    board_id: i64,
}

impl<'a> DeleteBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/agile/1.0/board/{}", self.board_id),
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

/// Returns all issues from a board's backlog, for a given board Id.
#[derive(Clone)]
pub struct GetIssuesForBacklogRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    board_id: i64,
    fields: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> GetIssuesForBacklogRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self {
            client,
            board_id,
            expand: None,
            jql: None,
            max_results: None,
            validate_query: None,
            fields: None,
            start_at: None,
        }
    }

    /// This parameter is currently not used. This parameter is a comma-separated list.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The maximum number of issues to return per page. Default: 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned issues. Base index: 0.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/backlog", self.board_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get the board configuration.
/// The response contains the following fields:
/// - id - Id of the board.
/// - name - Name of the board.
/// - filter - Reference to the filter used by the given board.
/// - subQuery (Kanban only) - JQL subquery used by the given board.
/// - columnConfig - The column configuration lists the columns for the board, in the order defined in the column configuration.
/// For each column, it shows the issue status mapping
/// as well as the constraint type (Valid values: none, issueCount, issueCountExclSubs) for the min/max number of issues.
/// Note, the last column with statuses mapped to it is treated as the "Done" column,
/// which means that issues in that column will be marked as already completed.
/// - estimation (Scrum only) - Contains information about type of estimation used for the board. Valid values: none, issueCount, field.
/// If the estimation type is "field", the Id and display name of the field used for estimation is also returned.
/// Note, estimates for an issue can be updated by a PUT /rest/api/2/issue/{issueIdOrKey} request, however the fields must be on the screen.
/// "timeoriginalestimate" field will never be on the screen, so in order to update it "originalEstimate" in "timetracking" field should be updated.
/// - ranking - Contains information about custom field used for ranking in the given board.
#[derive(Clone)]
pub struct BoardGetConfigurationRequest<'a> {
    client: &'a crate::core::Client,
    board_id: i64,
}

impl<'a> BoardGetConfigurationRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/configuration", self.board_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetConfiguration> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all epics from the board, for the given board Id. This only includes epics that the user has permission to view. Note, if the user does not have permission to view the board, no epics will be returned at all.
#[derive(Clone)]
pub struct GetEpicsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    board_id: i64,
    done: Option<String>,
    start_at: Option<i64>,
}

impl<'a> GetEpicsRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id, max_results: None, done: None, start_at: None }
    }

    /// The maximum number of epics to return per page. Default: 50. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Filters results to epics that are either done or not done. Valid values: true, false.
    #[must_use]
    pub fn done(mut self, value: impl Into<String>) -> Self {
        self.done = Some(value.into());

        self
    }

    /// The starting index of the returned epics. Base index: 0. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/epic", self.board_id),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.done {
            config.query.push(("done".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Epic>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Epic>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all issues that do not belong to any epic on a board, for a given board Id.
#[derive(Clone)]
pub struct GetIssuesWithoutEpicForBoardRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    board_id: i64,
    fields: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> GetIssuesWithoutEpicForBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self {
            client,
            board_id,
            expand: None,
            jql: None,
            max_results: None,
            validate_query: None,
            fields: None,
            start_at: None,
        }
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The maximum number of issues to return per page. Default: 50. See the 'Pagination' section at the top of this page for more details. Note, the total number of issues returned is limited by the property 'jira.search.views.default.max' in your JIRA instance. If you exceed this limit, your results will be truncated.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned issues. Base index: 0. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/epic/none/issue", self.board_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all issues that belong to an epic on the board, for the given epic Id and the board Id.
#[derive(Clone)]
pub struct GetBoardIssuesForEpicRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    board_id: i64,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    epic_id: i64,
    fields: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> GetBoardIssuesForEpicRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64, epic_id: i64) -> Self {
        Self {
            client,
            board_id,
            epic_id,
            expand: None,
            jql: None,
            max_results: None,
            validate_query: None,
            fields: None,
            start_at: None,
        }
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The maximum number of issues to return per page. Default: 50. See the 'Pagination' section at the top of this page for more details. Note, the total number of issues returned is limited by the property 'jira.search.views.default.max' in your JIRA instance. If you exceed this limit, your results will be truncated.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned issues. Base index: 0. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/epic/{}/issue", self.board_id, self.epic_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all issues from a board, for a given board Id. This only includes issues that the user has permission to view. Note, if the user does not have permission to view the board, no issues will be returned at all. Issues returned from this resource include Agile fields, like sprint, closedSprints, flagged, and epic. By default, the returned issues are ordered by rank.
#[derive(Clone)]
pub struct GetIssuesForBoardRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    board_id: i64,
    fields: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> GetIssuesForBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self {
            client,
            board_id,
            expand: None,
            jql: None,
            max_results: None,
            validate_query: None,
            fields: None,
            start_at: None,
        }
    }

    /// This parameter is currently not used. This parameter is a comma-separated list.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The maximum number of issues to return per page. Default: 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned issues. Base index: 0.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/issue", self.board_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all projects that are associated with the board, for the given board Id. A project is associated with a board only if the board filter explicitly filters issues by the project and guaranties that all issues will come for one of those projects e.g. board's filter with "project in (PR-1, PR-1) OR reporter = admin" jql Projects are returned only if user can browse all projects that are associated with the board. Note, if the user does not have permission to view the board, no projects will be returned at all. Returned projects are ordered by the name.
#[derive(Clone)]
pub struct GetProjectsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    board_id: i64,
    start_at: Option<i64>,
}

impl<'a> GetProjectsRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id, max_results: None, start_at: None }
    }

    /// The maximum number of projects to return per page. Default: 50. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The starting index of the returned projects. Base index: 0. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/project", self.board_id),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<ProjectJson>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ProjectJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the keys of all properties for the board identified by the id. The user who retrieves the property keys is required to have permissions to view the board.
#[derive(Clone)]
pub struct GetBoardPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    board_id: String,
}

impl<'a> GetBoardPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: impl Into<String>) -> Self {
        Self { client, board_id: board_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/properties", crate::core::encode_path_segment(&self.board_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the board identified by the provided id. The user who retrieves the property is required to have permissions to view the board.
#[derive(Clone)]
pub struct GetBoardPropertyRequest<'a> {
    client: &'a crate::core::Client,
    board_id: String,
    property_key: String,
}

impl<'a> GetBoardPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, board_id: board_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/agile/1.0/board/{}/properties/{}",
                crate::core::encode_path_segment(&self.board_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of the specified board's property. You can use this resource to store a custom data against the board identified by the id. The user who stores the data is required to have permissions to modify the board.
#[derive(Clone)]
pub struct SetBoardPropertyRequest<'a> {
    client: &'a crate::core::Client,
    board_id: String,
    property_key: String,
    property_value: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetBoardPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        board_id: impl Into<String>,
        property_key: impl Into<String>,
        property_value: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, board_id: board_id.into(), property_key: property_key.into(), property_value }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/agile/1.0/board/{}/properties/{}",
                crate::core::encode_path_segment(&self.board_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.property_value)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Option<PropertyKeys>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes the property from the board identified by the id. Ths user removing the property is required to have permissions to modify the board.
#[derive(Clone)]
pub struct DeleteBoardPropertyRequest<'a> {
    client: &'a crate::core::Client,
    board_id: String,
    property_key: String,
}

impl<'a> DeleteBoardPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, board_id: board_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/agile/1.0/board/{}/properties/{}",
                crate::core::encode_path_segment(&self.board_id),
                crate::core::encode_path_segment(&self.property_key)
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

/// Returns the value of the setting for refined velocity chart
#[derive(Clone)]
pub struct GetRefinedVelocityRequest<'a> {
    client: &'a crate::core::Client,
    board_id: i64,
}

impl<'a> GetRefinedVelocityRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/settings/refined-velocity", self.board_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BooleanSetting> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of the specified board's refined velocity setting.
#[derive(Clone)]
pub struct SetRefinedVelocityRequest<'a> {
    client: &'a crate::core::Client,
    board_id: i64,
    boolean_setting: BooleanSetting,
}

impl<'a> SetRefinedVelocityRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64, boolean_setting: BooleanSetting) -> Self {
        Self { client, board_id, boolean_setting }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/agile/1.0/board/{}/settings/refined-velocity", self.board_id),
        );

        let body = match serde_json::to_value(&self.boolean_setting)? {
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

/// Returns all sprints from a board, for a given board Id. This only includes sprints that the user has permission to view.
#[derive(Clone)]
pub struct GetAllSprintsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    board_id: i64,
    state: Option<GetAllSprintsRequestState>,
    start_at: Option<i64>,
}

impl<'a> GetAllSprintsRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id, max_results: None, state: None, start_at: None }
    }

    /// The maximum number of sprints to return per page. Default: 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Filters results to sprints in specified states. Valid values: future, active, closed. You can define multiple states separated by commas, e.g. state=active,closed
    #[must_use]
    pub fn state(mut self, value: impl Into<GetAllSprintsRequestState>) -> Self {
        self.state = Some(value.into());

        self
    }

    /// The starting index of the returned sprints. Base index: 0.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/sprint", self.board_id),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.state {
            config.query.push(("state".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Sprint>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Sprint>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get all issues you have access to that belong to the sprint from the board. Issue returned from this resource contains additional fields like: sprint, closedSprints, flagged and epic. Issues are returned ordered by rank. JQL order has higher priority than default rank.
#[derive(Clone)]
pub struct GetBoardIssuesForSprintRequest<'a> {
    client: &'a crate::core::Client,
    board_id: i64,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    sprint_id: i64,
    fields: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> GetBoardIssuesForSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64, sprint_id: i64) -> Self {
        Self {
            client,
            board_id,
            sprint_id,
            expand: None,
            jql: None,
            max_results: None,
            validate_query: None,
            fields: None,
            start_at: None,
        }
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The maximum number of sprints to return per page. Default: 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The starting index of the returned issues. Base index: 0. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/sprint/{}/issue", self.board_id, self.sprint_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all versions from a board, for a given board Id. This only includes versions that the user has permission to view. Note, if the user does not have permission to view the board, no versions will be returned at all. Returned versions are ordered by the name of the project from which they belong and then by sequence defined by user.
#[derive(Clone)]
pub struct GetAllVersionsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    board_id: i64,
    released: Option<String>,
    start_at: Option<i64>,
}

impl<'a> GetAllVersionsRequest<'a> {
    fn new(client: &'a crate::core::Client, board_id: i64) -> Self {
        Self { client, board_id, max_results: None, released: None, start_at: None }
    }

    /// The maximum number of versions to return per page. Default: 50.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Filters results to versions that are either released or unreleased. Valid values: true, false.
    #[must_use]
    pub fn released(mut self, value: impl Into<String>) -> Self {
        self.released = Some(value.into());

        self
    }

    /// The starting index of the returned versions. Base index: 0.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/board/{}/version", self.board_id),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.released {
            config.query.push(("released".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<AgileVersion>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<AgileVersion>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
