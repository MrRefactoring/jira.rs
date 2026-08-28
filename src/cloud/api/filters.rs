// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum CreateFilterRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFilterRequestExpand {
    One(CreateFilterRequestExpandValue),
    Many(Vec<CreateFilterRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetFavouriteFiltersRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFavouriteFiltersRequestExpand {
    One(GetFavouriteFiltersRequestExpandValue),
    Many(Vec<GetFavouriteFiltersRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetMyFiltersRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetMyFiltersRequestExpand {
    One(GetMyFiltersRequestExpandValue),
    Many(Vec<GetMyFiltersRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by filter description. Note that this sorting works independently of whether the expand to display the description field is in use.
    ///  *  `favourite_count` Sorts by the count of how many users have this filter as a favorite.
    ///  *  `is_favourite` Sorts by whether the filter is marked as a favorite.
    ///  *  `id` Sorts by filter ID.
    ///  *  `name` Sorts by filter name.
    ///  *  `owner` Sorts by the ID of the filter owner.
    ///  *  `is_shared` Sorts by whether the filter is shared.
    pub enum GetFiltersPaginatedRequestOrderBy {
        Description => "description",
        DescriptionDescending => "-description",
        DescriptionAscending => "+description",
        FavouriteCount => "favourite_count",
        FavouriteCountDescending => "-favourite_count",
        FavouriteCountAscending => "+favourite_count",
        Id => "id",
        IdDescending => "-id",
        IdAscending => "+id",
        IsFavourite => "is_favourite",
        IsFavouriteDescending => "-is_favourite",
        IsFavouriteAscending => "+is_favourite",
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
        Owner => "owner",
        OwnerDescending => "-owner",
        OwnerAscending => "+owner",
        IsShared => "is_shared",
        IsSharedDescending => "-is_shared",
        IsSharedAscending => "+is_shared",
    }
}

crate::open_enum! {
    pub enum GetFiltersPaginatedRequestExpandValue {
        Description => "description",
        Favourite => "favourite",
        FavouritedCount => "favouritedCount",
        Jql => "jql",
        Owner => "owner",
        SearchUrl => "searchUrl",
        SharePermissions => "sharePermissions",
        EditPermissions => "editPermissions",
        IsWritable => "isWritable",
        ApproximateLastUsed => "approximateLastUsed",
        Subscriptions => "subscriptions",
        ViewUrl => "viewUrl",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `description` Returns the description of the filter.
///  *  `favourite` Returns an indicator of whether the user has set the filter as a favorite.
///  *  `favouritedCount` Returns a count of how many users have set this filter as a favorite.
///  *  `jql` Returns the JQL query that the filter uses.
///  *  `owner` Returns the owner of the filter.
///  *  `searchUrl` Returns a URL to perform the filter's JQL query.
///  *  `sharePermissions` Returns the share permissions defined for the filter.
///  *  `editPermissions` Returns the edit permissions defined for the filter.
///  *  `isWritable` Returns whether the current user has permission to edit the filter.
///  *  `approximateLastUsed` \\[Experimental\\] Returns the approximate date and time when the filter was last evaluated.
///  *  `subscriptions` Returns the users that are subscribed to the filter.
///  *  `viewUrl` Returns a URL to view the filter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFiltersPaginatedRequestExpand {
    One(GetFiltersPaginatedRequestExpandValue),
    Many(Vec<GetFiltersPaginatedRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetFilterRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFilterRequestExpand {
    One(GetFilterRequestExpandValue),
    Many(Vec<GetFilterRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum UpdateFilterRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UpdateFilterRequestExpand {
    One(UpdateFilterRequestExpandValue),
    Many(Vec<UpdateFilterRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum SetFavouriteForFilterRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SetFavouriteForFilterRequestExpand {
    One(SetFavouriteForFilterRequestExpandValue),
    Many(Vec<SetFavouriteForFilterRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum DeleteFavouriteForFilterRequestExpandValue {
        SharedUsers => "sharedUsers",
        Subscriptions => "subscriptions",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum DeleteFavouriteForFilterRequestExpand {
    One(DeleteFavouriteForFilterRequestExpandValue),
    Many(Vec<DeleteFavouriteForFilterRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Filters operations.
pub struct FiltersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FiltersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a filter. The filter is shared according to the [default share scope](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-filters/#api-rest-api-3-filter-post). The filter is not selected as a favorite.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn create_filter(&self, filter: Filter) -> CreateFilterRequest<'a> {
        CreateFilterRequest::new(self.client, filter)
    }

    /// Returns the visible favorite filters of the user.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** A favorite filter is only visible to the user where the filter is:
    ///
    ///  *  owned by the user.
    ///  *  shared with a group that the user is a member of.
    ///  *  shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  shared with a public project.
    ///  *  shared with the public.
    ///
    /// For example, if the user favorites a public filter that is subsequently made private that filter is not returned by this operation.
    pub fn get_favourite_filters(&self) -> GetFavouriteFiltersRequest<'a> {
        GetFavouriteFiltersRequest::new(self.client)
    }

    /// Returns the filters owned by the user. If `includeFavourites` is `true`, the user's visible favorite filters are also returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, a favorite filters is only visible to the user where the filter is:
    ///
    ///  *  owned by the user.
    ///  *  shared with a group that the user is a member of.
    ///  *  shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  shared with a public project.
    ///  *  shared with the public.
    ///
    /// For example, if the user favorites a public filter that is subsequently made private that filter is not returned by this operation.
    pub fn get_my_filters(&self) -> GetMyFiltersRequest<'a> {
        GetMyFiltersRequest::new(self.client)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of filters. Use this operation to get:
    ///
    ///  *  specific filters, by defining `id` only.
    ///  *  filters that match all of the specified attributes. For example, all filters for a user with a particular word in their name. When multiple attributes are specified only filters matching all attributes are returned.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, only the following filters that match the query parameters are returned:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn get_filters_paginated(&self) -> GetFiltersPaginatedRequest<'a> {
        GetFiltersPaginatedRequest::new(self.client)
    }

    /// Returns a filter.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, the filter is only returned where it is:
    ///
    ///  *  owned by the user.
    ///  *  shared with a group that the user is a member of.
    ///  *  shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  shared with a public project.
    ///  *  shared with the public.
    pub fn get_filter(&self, id: i64) -> GetFilterRequest<'a> {
        GetFilterRequest::new(self.client, id)
    }

    /// Updates a filter. Use this operation to update a filter's name, description, JQL, or sharing.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however the user must own the filter.
    pub fn update_filter(&self, id: i64, body: Filter) -> UpdateFilterRequest<'a> {
        UpdateFilterRequest::new(self.client, id, body)
    }

    /// Delete a filter.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however filters can only be deleted by the creator of the filter or a user with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_filter(&self, id: i64) -> DeleteFilterRequest<'a> {
        DeleteFilterRequest::new(self.client, id)
    }

    /// Returns the columns configured for a filter. The column configuration is used when the filter's results are viewed in *List View* with the *Columns* set to *Filter*.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, column details are only returned for:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn get_columns(&self, id: i64) -> GetColumnsRequest<'a> {
        GetColumnsRequest::new(self.client, id)
    }

    /// Sets the columns for a filter. Only navigable fields can be set as columns. Use [Get fields](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-fields/#api-rest-api-3-field-get) to get the list fields in Jira. A navigable field has `navigable` set to `true`.
    ///
    /// The parameters for this resource are expressed as HTML form data. For example, in curl:
    ///
    /// `curl -X PUT -d columns=summary -d columns=description https://your-domain.atlassian.net/rest/api/3/filter/10000/columns`
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, columns are only set for:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn set_columns(&self, id: i64, column_request_body: ColumnRequestBody) -> SetColumnsRequest<'a> {
        SetColumnsRequest::new(self.client, id, column_request_body)
    }

    /// Reset the user's column configuration for the filter to the default.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, columns are only reset for:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn reset_columns(&self, id: i64) -> ResetColumnsRequest<'a> {
        ResetColumnsRequest::new(self.client, id)
    }

    /// Add a filter as a favorite for the user.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, the user can only favorite:
    ///
    ///  *  filters owned by the user.
    ///  *  filters shared with a group that the user is a member of.
    ///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
    ///  *  filters shared with a public project.
    ///  *  filters shared with the public.
    pub fn set_favourite_for_filter(&self, id: i64) -> SetFavouriteForFilterRequest<'a> {
        SetFavouriteForFilterRequest::new(self.client, id)
    }

    /// Removes a filter as a favorite for the user. Note that this operation only removes filters visible to the user from the user's favorites list. For example, if the user favorites a public filter that is subsequently made private (and is therefore no longer visible on their favorites list) they cannot remove it from their favorites list.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn delete_favourite_for_filter(&self, id: i64) -> DeleteFavouriteForFilterRequest<'a> {
        DeleteFavouriteForFilterRequest::new(self.client, id)
    }
}

/// Creates a filter. The filter is shared according to the [default share scope](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-filters/#api-rest-api-3-filter-post). The filter is not selected as a favorite.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
pub struct CreateFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<CreateFilterRequestExpand>,
    override_share_permissions: Option<bool>,
    filter: Filter,
}

impl<'a> CreateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, filter: Filter) -> Self {
        Self { client, filter, expand: None, override_share_permissions: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: CreateFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// EXPERIMENTAL: Whether share permissions are overridden to enable filters with any share permissions to be created. Available to users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    #[must_use]
    pub fn override_share_permissions(mut self, value: bool) -> Self {
        self.override_share_permissions = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/filter".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.override_share_permissions {
            config
                .query
                .push(("overrideSharePermissions".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.filter)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the visible favorite filters of the user.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** A favorite filter is only visible to the user where the filter is:
///
///  *  owned by the user.
///  *  shared with a group that the user is a member of.
///  *  shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  shared with a public project.
///  *  shared with the public.
///
/// For example, if the user favorites a public filter that is subsequently made private that filter is not returned by this operation.
pub struct GetFavouriteFiltersRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetFavouriteFiltersRequestExpand>,
}

impl<'a> GetFavouriteFiltersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: GetFavouriteFiltersRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/filter/favourite".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Filter>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the filters owned by the user. If `includeFavourites` is `true`, the user's visible favorite filters are also returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, a favorite filters is only visible to the user where the filter is:
///
///  *  owned by the user.
///  *  shared with a group that the user is a member of.
///  *  shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  shared with a public project.
///  *  shared with the public.
///
/// For example, if the user favorites a public filter that is subsequently made private that filter is not returned by this operation.
pub struct GetMyFiltersRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetMyFiltersRequestExpand>,
    include_favourites: Option<bool>,
}

impl<'a> GetMyFiltersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, include_favourites: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: GetMyFiltersRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// Include the user's favorite filters in the response.
    #[must_use]
    pub fn include_favourites(mut self, value: bool) -> Self {
        self.include_favourites = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/filter/my".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.include_favourites {
            config.query.push(("includeFavourites".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Filter>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of filters. Use this operation to get:
///
///  *  specific filters, by defining `id` only.
///  *  filters that match all of the specified attributes. For example, all filters for a user with a particular word in their name. When multiple attributes are specified only filters matching all attributes are returned.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, only the following filters that match the query parameters are returned:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct GetFiltersPaginatedRequest<'a> {
    client: &'a crate::core::Client,
    filter_name: Option<String>,
    account_id: Option<String>,
    groupname: Option<String>,
    group_id: Option<String>,
    project_id: Option<i64>,
    id: Option<Vec<i64>>,
    order_by: Option<GetFiltersPaginatedRequestOrderBy>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    expand: Option<GetFiltersPaginatedRequestExpand>,
    override_share_permissions: Option<bool>,
    is_substring_match: Option<bool>,
}

impl<'a> GetFiltersPaginatedRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            filter_name: None,
            account_id: None,
            groupname: None,
            group_id: None,
            project_id: None,
            id: None,
            order_by: None,
            start_at: None,
            max_results: None,
            expand: None,
            override_share_permissions: None,
            is_substring_match: None,
        }
    }

    /// String used to perform a case-insensitive partial match with `name`.
    #[must_use]
    pub fn filter_name(mut self, value: impl Into<String>) -> Self {
        self.filter_name = Some(value.into());

        self
    }

    /// User account ID used to return filters with the matching `owner.accountId`. This parameter cannot be used with `owner`.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// As a group's name can change, use of `groupId` is recommended to identify a group. Group name used to returns filters that are shared with a group that matches `sharePermissions.group.groupname`. This parameter cannot be used with the `groupId` parameter.
    #[must_use]
    pub fn groupname(mut self, value: impl Into<String>) -> Self {
        self.groupname = Some(value.into());

        self
    }

    /// Group ID used to returns filters that are shared with a group that matches `sharePermissions.group.groupId`. This parameter cannot be used with the `groupname` parameter.
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// Project ID used to returns filters that are shared with a project that matches `sharePermissions.project.id`.
    #[must_use]
    pub fn project_id(mut self, value: i64) -> Self {
        self.project_id = Some(value);

        self
    }

    /// The list of filter IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`. Do not exceed 200 filter IDs.
    #[must_use]
    pub fn id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.id = Some(value.into_iter().collect());

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by filter description. Note that this sorting works independently of whether the expand to display the description field is in use.
    ///  *  `favourite_count` Sorts by the count of how many users have this filter as a favorite.
    ///  *  `is_favourite` Sorts by whether the filter is marked as a favorite.
    ///  *  `id` Sorts by filter ID.
    ///  *  `name` Sorts by filter name.
    ///  *  `owner` Sorts by the ID of the filter owner.
    ///  *  `is_shared` Sorts by whether the filter is shared.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetFiltersPaginatedRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
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

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `description` Returns the description of the filter.
    ///  *  `favourite` Returns an indicator of whether the user has set the filter as a favorite.
    ///  *  `favouritedCount` Returns a count of how many users have set this filter as a favorite.
    ///  *  `jql` Returns the JQL query that the filter uses.
    ///  *  `owner` Returns the owner of the filter.
    ///  *  `searchUrl` Returns a URL to perform the filter's JQL query.
    ///  *  `sharePermissions` Returns the share permissions defined for the filter.
    ///  *  `editPermissions` Returns the edit permissions defined for the filter.
    ///  *  `isWritable` Returns whether the current user has permission to edit the filter.
    ///  *  `approximateLastUsed` \\[Experimental\\] Returns the approximate date and time when the filter was last evaluated.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter.
    ///  *  `viewUrl` Returns a URL to view the filter.
    #[must_use]
    pub fn expand(mut self, value: GetFiltersPaginatedRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// EXPERIMENTAL: Whether share permissions are overridden to enable filters with any share permissions to be returned. Available to users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    #[must_use]
    pub fn override_share_permissions(mut self, value: bool) -> Self {
        self.override_share_permissions = Some(value);

        self
    }

    /// When `true` this will perform a case-insensitive substring match for the provided `filterName`. When `false` the filter name will be searched using [full text search syntax](https://support.atlassian.com/jira-software-cloud/docs/search-for-issues-using-the-text-field/).
    #[must_use]
    pub fn is_substring_match(mut self, value: bool) -> Self {
        self.is_substring_match = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/filter/search".to_owned());

        if let Some(value) = &self.filter_name {
            config.query.push(("filterName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.groupname {
            config.query.push(("groupname".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.group_id {
            config.query.push(("groupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.id {
            config.query.push(("id".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.override_share_permissions {
            config
                .query
                .push(("overrideSharePermissions".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.is_substring_match {
            config.query.push(("isSubstringMatch".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<FilterDetails>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a filter.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, the filter is only returned where it is:
///
///  *  owned by the user.
///  *  shared with a group that the user is a member of.
///  *  shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  shared with a public project.
///  *  shared with the public.
pub struct GetFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    expand: Option<GetFilterRequestExpand>,
    override_share_permissions: Option<bool>,
}

impl<'a> GetFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, expand: None, override_share_permissions: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: GetFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// EXPERIMENTAL: Whether share permissions are overridden to enable filters with any share permissions to be returned. Available to users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    #[must_use]
    pub fn override_share_permissions(mut self, value: bool) -> Self {
        self.override_share_permissions = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/3/filter/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.override_share_permissions {
            config
                .query
                .push(("overrideSharePermissions".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a filter. Use this operation to update a filter's name, description, JQL, or sharing.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however the user must own the filter.
pub struct UpdateFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    expand: Option<UpdateFilterRequestExpand>,
    override_share_permissions: Option<bool>,
    body: Filter,
}

impl<'a> UpdateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, body: Filter) -> Self {
        Self { client, id, body, expand: None, override_share_permissions: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: UpdateFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// EXPERIMENTAL: Whether share permissions are overridden to enable the addition of any share permissions to filters. Available to users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    #[must_use]
    pub fn override_share_permissions(mut self, value: bool) -> Self {
        self.override_share_permissions = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, format!("/rest/api/3/filter/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.override_share_permissions {
            config
                .query
                .push(("overrideSharePermissions".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a filter.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however filters can only be deleted by the creator of the filter or a user with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct DeleteFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> DeleteFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, format!("/rest/api/3/filter/{}", self.id));

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

/// Returns the columns configured for a filter. The column configuration is used when the filter's results are viewed in *List View* with the *Columns* set to *Filter*.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, column details are only returned for:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct GetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/filter/{}/columns", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnItem>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the columns for a filter. Only navigable fields can be set as columns. Use [Get fields](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-fields/#api-rest-api-3-field-get) to get the list fields in Jira. A navigable field has `navigable` set to `true`.
///
/// The parameters for this resource are expressed as HTML form data. For example, in curl:
///
/// `curl -X PUT -d columns=summary -d columns=description https://your-domain.atlassian.net/rest/api/3/filter/10000/columns`
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, columns are only set for:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct SetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    column_request_body: ColumnRequestBody,
}

impl<'a> SetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, column_request_body: ColumnRequestBody) -> Self {
        Self { client, id, column_request_body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/filter/{}/columns", self.id),
        );

        let body = match serde_json::to_value(&self.column_request_body)? {
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

/// Reset the user's column configuration for the filter to the default.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, columns are only reset for:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct ResetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> ResetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/filter/{}/columns", self.id),
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

/// Add a filter as a favorite for the user.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira, however, the user can only favorite:
///
///  *  filters owned by the user.
///  *  filters shared with a group that the user is a member of.
///  *  filters shared with a private project that the user has *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for.
///  *  filters shared with a public project.
///  *  filters shared with the public.
pub struct SetFavouriteForFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    expand: Option<SetFavouriteForFilterRequestExpand>,
}

impl<'a> SetFavouriteForFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: SetFavouriteForFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/filter/{}/favourite", self.id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes a filter as a favorite for the user. Note that this operation only removes filters visible to the user from the user's favorites list. For example, if the user favorites a public filter that is subsequently made private (and is therefore no longer visible on their favorites list) they cannot remove it from their favorites list.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
pub struct DeleteFavouriteForFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    expand: Option<DeleteFavouriteForFilterRequestExpand>,
}

impl<'a> DeleteFavouriteForFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about filter in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `sharedUsers` Returns the users that the filter is shared with. This includes users that can browse projects that the filter is shared with. If you don't specify `sharedUsers`, then the `sharedUsers` object is returned but it doesn't list any users. The list of users returned is limited to 1000, to access additional users append `[start-index:end-index]` to the expand request. For example, to access the next 1000 users, use `?expand=sharedUsers[1001:2000]`.
    ///  *  `subscriptions` Returns the users that are subscribed to the filter. If you don't specify `subscriptions`, the `subscriptions` object is returned but it doesn't list any subscriptions. The list of subscriptions returned is limited to 1000, to access additional subscriptions append `[start-index:end-index]` to the expand request. For example, to access the next 1000 subscriptions, use `?expand=subscriptions[1001:2000]`.
    #[must_use]
    pub fn expand(mut self, value: DeleteFavouriteForFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/filter/{}/favourite", self.id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
