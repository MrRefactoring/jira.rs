// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::Serialize;

/// A comma-separated list of the parameters to expand.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum IssueGetIssueRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(IssueGetIssueRequestExpand { One, Many, Other });

/// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum IssueGetIssueRequestFields {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(IssueGetIssueRequestFields { One, Many, Other });

/// The Issue operations.
pub struct IssueService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Moves (ranks) issues before or after a given issue. At most 50 issues may be ranked at once. This operation may fail for some issues, although this will be rare. In that case the 207 status code is returned for the whole response and detailed information regarding each issue is available in the response body. If rankCustomFieldId is not defined, the default rank field will be used.
    pub fn rank_issues(&self, issue_rank_request: IssueRankRequest) -> RankIssuesRequest<'a> {
        RankIssuesRequest::new(self.client, issue_rank_request)
    }

    /// Returns a single issue, for a given issue Id or issue key. Issues returned from this resource include Agile fields, like sprint, closedSprints, flagged, and epic.
    pub fn get_issue(&self, issue_id_or_key: impl Into<String>) -> IssueGetIssueRequest<'a> {
        IssueGetIssueRequest::new(self.client, issue_id_or_key)
    }

    /// Returns the estimation of the issue and a fieldId of the field that is used for it.
    /// Original time internally stores and returns the estimation as a number of seconds.
    /// The field used for estimation on the given board can be obtained from board configuration resource.
    /// More information about the field are returned by edit meta resource or field resource.
    pub fn get_issue_estimation_for_board(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> GetIssueEstimationForBoardRequest<'a> {
        GetIssueEstimationForBoardRequest::new(self.client, issue_id_or_key)
    }

    /// Updates the estimation of the issue. boardId param is required. This param determines which field will be updated on a issue.
    /// Note that this resource changes the estimation field of the issue regardless of appearance the field on the screen.
    /// Original time tracking estimation field accepts estimation in formats like "1w", "2d", "3h", "20m" or number which represent number of minutes.
    /// However, internally the field stores and returns the estimation as a number of seconds.
    /// The field used for estimation on the given board can be obtained from [board configuration resource](https://developer.atlassian.com/server/jira/platform/rest/v11003/intro#agile/1.0/board-getConfiguration).
    /// More information about the field are returned by edit meta resource or field resource.
    pub fn estimate_issue_for_board(
        &self,
        issue_id_or_key: impl Into<String>,
        field_edit: FieldEdit,
    ) -> EstimateIssueForBoardRequest<'a> {
        EstimateIssueForBoardRequest::new(self.client, issue_id_or_key, field_edit)
    }
}

/// Moves (ranks) issues before or after a given issue. At most 50 issues may be ranked at once. This operation may fail for some issues, although this will be rare. In that case the 207 status code is returned for the whole response and detailed information regarding each issue is available in the response body. If rankCustomFieldId is not defined, the default rank field will be used.
#[derive(Clone)]
pub struct RankIssuesRequest<'a> {
    client: &'a crate::core::Client,
    issue_rank_request: IssueRankRequest,
}

impl<'a> RankIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_rank_request: IssueRankRequest) -> Self {
        Self { client, issue_rank_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/agile/1.0/issue/rank".to_owned());

        let body = match serde_json::to_value(&self.issue_rank_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Option<PartialSuccess>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a single issue, for a given issue Id or issue key. Issues returned from this resource include Agile fields, like sprint, closedSprints, flagged, and epic.
#[derive(Clone)]
pub struct IssueGetIssueRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<IssueGetIssueRequestExpand>,
    issue_id_or_key: String,
    fields: Option<IssueGetIssueRequestFields>,
    update_history: Option<bool>,
}

impl<'a> IssueGetIssueRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), expand: None, fields: None, update_history: None }
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: IssueGetIssueRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: IssueGetIssueRequestFields) -> Self {
        self.fields = Some(value);

        self
    }

    /// A boolean indicating whether the issue retrieved by this method should be added to the current user's issue history.
    #[must_use]
    pub fn update_history(mut self, value: bool) -> Self {
        self.update_history = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/issue/{}", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.update_history {
            config.query.push(("updateHistory".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Issue> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the estimation of the issue and a fieldId of the field that is used for it.
/// Original time internally stores and returns the estimation as a number of seconds.
/// The field used for estimation on the given board can be obtained from board configuration resource.
/// More information about the field are returned by edit meta resource or field resource.
#[derive(Clone)]
pub struct GetIssueEstimationForBoardRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    board_id: Option<i64>,
}

impl<'a> GetIssueEstimationForBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), board_id: None }
    }

    /// The id of the board required to determine which field is used for estimation.
    #[must_use]
    pub fn board_id(mut self, value: i64) -> Self {
        self.board_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/issue/{}/estimation", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.board_id {
            config.query.push(("boardId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<FieldValue> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the estimation of the issue. boardId param is required. This param determines which field will be updated on a issue.
/// Note that this resource changes the estimation field of the issue regardless of appearance the field on the screen.
/// Original time tracking estimation field accepts estimation in formats like "1w", "2d", "3h", "20m" or number which represent number of minutes.
/// However, internally the field stores and returns the estimation as a number of seconds.
/// The field used for estimation on the given board can be obtained from [board configuration resource](https://developer.atlassian.com/server/jira/platform/rest/v11003/intro#agile/1.0/board-getConfiguration).
/// More information about the field are returned by edit meta resource or field resource.
#[derive(Clone)]
pub struct EstimateIssueForBoardRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    board_id: Option<i64>,
    field_edit: FieldEdit,
}

impl<'a> EstimateIssueForBoardRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, field_edit: FieldEdit) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), field_edit, board_id: None }
    }

    /// The id of the board required to determine which field is used for estimation.
    #[must_use]
    pub fn board_id(mut self, value: i64) -> Self {
        self.board_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/agile/1.0/issue/{}/estimation", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.board_id {
            config.query.push(("boardId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.field_edit)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<FieldValue> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
