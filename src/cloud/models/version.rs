// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a project version.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Version {
    /// If the expand option `approvers` is used, returns a list containing the approvers for this version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<VersionApprover>>,
    /// Indicates that the version is archived. Optional when creating or updating a version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// The description of the version. Optional when creating or updating a version. The maximum size is 16,384 bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The Atlassian account ID of the version driver. Optional when creating or updating a version. If the expand option `driver` is used, returns the Atlassian account ID of the driver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `operations` Returns the list of operations available for this version.
    ///  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.
    ///  *  `driver` Returns the Atlassian account ID of the version driver.
    ///  *  `approvers` Returns a list containing approvers for this version.
    ///
    /// Optional for create and update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "issuesStatusForFixVersion", default, skip_serializing_if = "Option::is_none")]
    pub issues_status_for_fix_version: Option<VersionIssuesStatus>,
    /// The URL of the self link to the version to which all unfixed issues are moved when a version is released. Not applicable when creating a version. Optional when updating a version.
    #[serde(rename = "moveUnfixedIssuesTo", default, skip_serializing_if = "Option::is_none")]
    pub move_unfixed_issues_to: Option<String>,
    /// The unique name of the version. Required when creating a version. Optional when updating a version. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If the expand option `operations` is used, returns the list of operations available for this version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<SimpleLink>>,
    /// Indicates that the version is overdue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overdue: Option<bool>,
    /// The ID of the project to which this version is attached. Required when creating a version. Not applicable when updating a version.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The release date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version.
    #[serde(rename = "releaseDate", default, skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Indicates that the version is released. If the version is released a request to release again is ignored. Not applicable when creating a version. Optional when updating a version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub released: Option<bool>,
    /// The URL of the version.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The start date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version.
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The date on which work on this version is expected to finish, expressed in the instance's *Day/Month/Year Format* date format.
    #[serde(rename = "userReleaseDate", default, skip_serializing_if = "Option::is_none")]
    pub user_release_date: Option<String>,
    /// The date on which work on this version is expected to start, expressed in the instance's *Day/Month/Year Format* date format.
    #[serde(rename = "userStartDate", default, skip_serializing_if = "Option::is_none")]
    pub user_start_date: Option<String>,
}
