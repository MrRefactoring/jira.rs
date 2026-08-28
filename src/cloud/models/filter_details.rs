// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a filter.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilterDetails {
    /// \\[Experimental\\] Approximate last used time. Returns the date and time when the filter was last used. Returns `null` if the filter hasn't been used after tracking was enabled. For performance reasons, timestamps aren't updated in real time and therefore may not be exactly accurate.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "approximateLastUsed",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub approximate_last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// \\[Experimental\\] Approximate last used time. Returns the date and time when the filter was last used. Returns `null` if the filter hasn't been used after tracking was enabled. For performance reasons, timestamps aren't updated in real time and therefore may not be exactly accurate.
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "approximateLastUsed",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub approximate_last_used: Option<String>,
    /// The description of the filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The groups and projects that can edit the filter. This can be specified when updating a filter, but not when creating a filter.
    #[serde(rename = "editPermissions", default, skip_serializing_if = "Option::is_none")]
    pub edit_permissions: Option<Vec<SharePermission>>,
    /// Expand options that include additional filter details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Whether the filter is selected as a favorite by any users, not including the filter owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    /// The count of how many users have selected this filter as a favorite, including the filter owner.
    #[serde(rename = "favouritedCount", default, skip_serializing_if = "Option::is_none")]
    pub favourited_count: Option<i64>,
    /// The unique identifier for the filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The JQL query for the filter. For example, *project = SSP AND issuetype = Bug*.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The name of the filter.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<DashboardUser>,
    /// A URL to view the filter results in Jira, using the [Search for issues using JQL](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-filters/#api-rest-api-3-filter-search-get) operation with the filter's JQL string to return the filter results. For example, *<https://your-domain.atlassian.net/rest/api/3/search?jql=project+%3D+SSP+AND+issuetype+%3D+Bug*>.
    #[serde(rename = "searchUrl", default, skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    /// The URL of the filter.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The groups and projects that the filter is shared with. This can be specified when updating a filter, but not when creating a filter.
    #[serde(rename = "sharePermissions", default, skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<SharePermission>>,
    /// The users that are subscribed to the filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<FilterSubscription>>,
    /// A URL to view the filter results in Jira, using the ID of the filter. For example, *<https://your-domain.atlassian.net/issues/?filter=10100*>.
    #[serde(rename = "viewUrl", default, skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}
