// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum MultiDirectoryGroupSearchRequestExpand {
        CountsResources => "counts.resources",
        CountsUsers => "counts.users",
    }
}

/// Filters for searching groups in a directory.
///
/// The request body is optional — sending an empty body returns the first page of all groups in the directory.
///
/// Use `expand` to include additional count fields. Other count toggles are not exposed on this endpoint; the `expand` array is the only way to request `counts.resources` or `counts.users`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiDirectoryGroupSearchRequest {
    /// Sets the starting point for the page of results to return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The number of results to return per page. Defaults to 20.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of sort fields. Currently only a single sort field is supported.
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<GroupSortBy>>,
    /// Filter by account IDs of group members.
    #[serde(rename = "accountIds", default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Filter by directory IDs.
    #[serde(rename = "directoryIds", default, skip_serializing_if = "Option::is_none")]
    pub directory_ids: Option<Vec<String>>,
    /// Filter by canonical Atlassian role IDs.
    #[serde(rename = "roleIds", default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    /// Filter by resource type keys.
    #[serde(rename = "resourceOwners", default, skip_serializing_if = "Option::is_none")]
    pub resource_owners: Option<Vec<String>>,
    /// Filter by resource IDs.
    #[serde(rename = "resourceIds", default, skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// Free-text search term. Matched against the group name.
    ///
    /// Mutually exclusive with `groupNames` — providing both returns a `400 Bad Request` error.
    #[serde(rename = "searchTerm", default, skip_serializing_if = "Option::is_none")]
    pub search_term: Option<String>,
    /// Filter by group IDs.
    ///
    /// Mutually exclusive with `groupNames` — providing both returns a `400 Bad Request` error.
    #[serde(rename = "groupIds", default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of full group names to filter by (case-insensitive). Only exact matches are returned.
    ///
    /// Mutually exclusive with `searchTerm` and `groupIds` — providing either combination returns a `400 Bad Request` error.
    #[serde(rename = "groupNames", default, skip_serializing_if = "Option::is_none")]
    pub group_names: Option<Vec<String>>,
    /// List of additional fields to include in the response. Available values:
    ///
    /// - `counts.resources` — the number of resources the group has access to.
    /// - `counts.users` — the number of users in the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<MultiDirectoryGroupSearchRequestExpand>>,
}
