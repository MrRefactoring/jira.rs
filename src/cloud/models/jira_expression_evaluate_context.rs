// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionEvaluateContext {
    /// The ID of the board that is available under the `board` variable when evaluating the expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub board: Option<i64>,
    /// Custom context variables and their types. These variable types are available for use in a custom context:
    ///
    ///  *  `user`: A [user](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#user) specified as an Atlassian account ID.
    ///  *  `issue`: An [issue](https://developer.atlassian.com/cloud/jira/platform/jira-expressions-type-reference#issue) specified by ID or key. All the fields of the issue object are available in the Jira expression.
    ///  *  `json`: A JSON object containing custom content.
    ///  *  `list`: A JSON list of `user`, `issue`, or `json` variable types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<CustomContextVariable>>,
    /// The ID of the customer request that is available under the `customerRequest` variable when evaluating the expression. This is the same as the ID of the underlying Jira issue, but the customer request context variable will have a different type.
    #[serde(rename = "customerRequest", default, skip_serializing_if = "Option::is_none")]
    pub customer_request: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<IdOrKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<JexpEvaluateCtxIssues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<IdOrKey>,
    /// The ID of the service desk that is available under the `serviceDesk` variable when evaluating the expression.
    #[serde(rename = "serviceDesk", default, skip_serializing_if = "Option::is_none")]
    pub service_desk: Option<i64>,
    /// The ID of the sprint that is available under the `sprint` variable when evaluating the expression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprint: Option<i64>,
}
