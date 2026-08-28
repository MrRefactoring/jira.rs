// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The filter applied to the list of dashboards. Valid values are:
    ///
    ///  *  `favourite` Returns dashboards the user has marked as favorite.
    ///  *  `my` Returns dashboards owned by the user.
    pub enum GetAllDashboardsRequestFilter {
        My => "my",
        Favourite => "favourite",
    }
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by dashboard description. Note that this sort works independently of whether the expand to display the description field is in use.
    ///  *  `favourite_count` Sorts by dashboard popularity.
    ///  *  `id` Sorts by dashboard ID.
    ///  *  `is_favourite` Sorts by whether the dashboard is marked as a favorite.
    ///  *  `name` Sorts by dashboard name.
    ///  *  `owner` Sorts by dashboard owner name.
    pub enum GetDashboardsPaginatedRequestOrderBy {
        Description => "description",
        DescriptionDescending => "-description",
        DescriptionAscending => "+description",
        FavoriteCount => "favorite_count",
        FavoriteCountDescending => "-favorite_count",
        FavoriteCountAscending => "+favorite_count",
        Id => "id",
        IdDescending => "-id",
        IdAscending => "+id",
        IsFavorite => "is_favorite",
        IsFavoriteDescending => "-is_favorite",
        IsFavoriteAscending => "+is_favorite",
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
        Owner => "owner",
        OwnerDescending => "-owner",
        OwnerAscending => "+owner",
    }
}

crate::open_enum! {
    /// The status to filter by. It may be active, archived or deleted.
    pub enum GetDashboardsPaginatedRequestStatus {
        Active => "active",
        Archived => "archived",
        Deleted => "deleted",
    }
}

crate::open_enum! {
    pub enum GetDashboardsPaginatedRequestExpandValue {
        Description => "description",
        Owner => "owner",
        ViewUrl => "viewUrl",
        Favourite => "favourite",
        FavouritedCount => "favouritedCount",
        SharePermissions => "sharePermissions",
        EditPermissions => "editPermissions",
        IsWritable => "isWritable",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about dashboard in the response. This parameter accepts a comma-separated list. Expand options include:
///
///  *  `description` Returns the description of the dashboard.
///  *  `owner` Returns the owner of the dashboard.
///  *  `viewUrl` Returns the URL that is used to view the dashboard.
///  *  `favourite` Returns `isFavourite`, an indicator of whether the user has set the dashboard as a favorite.
///  *  `favouritedCount` Returns `popularity`, a count of how many users have set this dashboard as a favorite.
///  *  `sharePermissions` Returns details of the share permissions defined for the dashboard.
///  *  `editPermissions` Returns details of the edit permissions defined for the dashboard.
///  *  `isWritable` Returns whether the current user has permission to edit the dashboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetDashboardsPaginatedRequestExpand {
    One(GetDashboardsPaginatedRequestExpandValue),
    Many(Vec<GetDashboardsPaginatedRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Dashboards operations.
pub struct DashboardsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> DashboardsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of dashboards owned by or shared with the user. The list may be filtered to include only favorite or owned dashboards.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_all_dashboards(&self) -> GetAllDashboardsRequest<'a> {
        GetAllDashboardsRequest::new(self.client)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of dashboards. This operation is similar to [Get dashboards](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-dashboard/#api-rest-api-3-dashboard-get) except that the results can be refined to include dashboards that have specific attributes. For example, dashboards with a particular name. When multiple attributes are specified only filters matching all attributes are returned.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The following dashboards that match the query parameters are returned:
    ///
    ///  *  Dashboards owned by the user. Not returned for anonymous users.
    ///  *  Dashboards shared with a group that the user is a member of. Not returned for anonymous users.
    ///  *  Dashboards shared with a private project that the user can browse. Not returned for anonymous users.
    ///  *  Dashboards shared with a public project.
    ///  *  Dashboards shared with the public.
    pub fn get_dashboards_paginated(&self) -> GetDashboardsPaginatedRequest<'a> {
        GetDashboardsPaginatedRequest::new(self.client)
    }

    /// Returns the keys of all properties for a dashboard item.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have read permission of the dashboard or have the dashboard shared with them.
    pub fn get_dashboard_item_property_keys(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> GetDashboardItemPropertyKeysRequest<'a> {
        GetDashboardItemPropertyKeysRequest::new(self.client, dashboard_id, item_id)
    }

    /// Returns the key and value of a dashboard item property.
    ///
    /// A dashboard item enables an app to add user-specific information to a user dashboard. Dashboard items are exposed to users as gadgets that users can add to their dashboards. For more information on how users do this, see [Adding and customizing gadgets](https://confluence.atlassian.com/x/7AeiLQ).
    ///
    /// When an app creates a dashboard item it registers a callback to receive the dashboard item ID. The callback fires whenever the item is rendered or, where the item is configurable, the user edits the item. The app then uses this resource to store the item's content or configuration details. For more information on working with dashboard items, see [ Building a dashboard item for a JIRA Connect add-on](https://developer.atlassian.com/server/jira/platform/guide-building-a-dashboard-item-for-a-jira-connect-add-on-33746254/) and the [Dashboard Item](https://developer.atlassian.com/cloud/jira/platform/modules/dashboard-item/) documentation.
    ///
    /// There is no resource to set or get dashboard items.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have read permission of the dashboard or have the dashboard shared with them.
    pub fn get_dashboard_item_property(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetDashboardItemPropertyRequest<'a> {
        GetDashboardItemPropertyRequest::new(self.client, dashboard_id, item_id, property_key)
    }

    /// Sets the value of a dashboard item property. Use this resource in apps to store custom data against a dashboard item.
    ///
    /// A dashboard item enables an app to add user-specific information to a user dashboard. Dashboard items are exposed to users as gadgets that users can add to their dashboards. For more information on how users do this, see [Adding and customizing gadgets](https://confluence.atlassian.com/x/7AeiLQ).
    ///
    /// When an app creates a dashboard item it registers a callback to receive the dashboard item ID. The callback fires whenever the item is rendered or, where the item is configurable, the user edits the item. The app then uses this resource to store the item's content or configuration details. For more information on working with dashboard items, see [ Building a dashboard item for a JIRA Connect add-on](https://developer.atlassian.com/server/jira/platform/guide-building-a-dashboard-item-for-a-jira-connect-add-on-33746254/) and the [Dashboard Item](https://developer.atlassian.com/cloud/jira/platform/modules/dashboard-item/) documentation.
    ///
    /// There is no resource to set or get dashboard items.
    ///
    /// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have edit permisson of the dashboard.
    pub fn set_dashboard_item_property(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetDashboardItemPropertyRequest<'a> {
        SetDashboardItemPropertyRequest::new(self.client, dashboard_id, item_id, property_key, body)
    }

    /// Deletes a dashboard item property.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have edit permission of the dashboard.
    pub fn delete_dashboard_item_property(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteDashboardItemPropertyRequest<'a> {
        DeleteDashboardItemPropertyRequest::new(self.client, dashboard_id, item_id, property_key)
    }

    /// Returns a dashboard.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    ///
    /// However, to get a dashboard, the dashboard must be shared with the user or the user must own it. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard. The System dashboard is considered to be shared with all other users.
    pub fn get_dashboard(&self, id: impl Into<String>) -> GetDashboardRequest<'a> {
        GetDashboardRequest::new(self.client, id)
    }
}

/// Returns a list of dashboards owned by or shared with the user. The list may be filtered to include only favorite or owned dashboards.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
pub struct GetAllDashboardsRequest<'a> {
    client: &'a crate::core::Client,
    filter: Option<GetAllDashboardsRequestFilter>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetAllDashboardsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, filter: None, start_at: None, max_results: None }
    }

    /// The filter applied to the list of dashboards. Valid values are:
    ///
    ///  *  `favourite` Returns dashboards the user has marked as favorite.
    ///  *  `my` Returns dashboards owned by the user.
    #[must_use]
    pub fn filter(mut self, value: impl Into<GetAllDashboardsRequestFilter>) -> Self {
        self.filter = Some(value.into());

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

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/dashboard".to_owned());

        if let Some(value) = &self.filter {
            config.query.push(("filter".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageOfDashboards> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of dashboards. This operation is similar to [Get dashboards](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-dashboard/#api-rest-api-3-dashboard-get) except that the results can be refined to include dashboards that have specific attributes. For example, dashboards with a particular name. When multiple attributes are specified only filters matching all attributes are returned.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The following dashboards that match the query parameters are returned:
///
///  *  Dashboards owned by the user. Not returned for anonymous users.
///  *  Dashboards shared with a group that the user is a member of. Not returned for anonymous users.
///  *  Dashboards shared with a private project that the user can browse. Not returned for anonymous users.
///  *  Dashboards shared with a public project.
///  *  Dashboards shared with the public.
pub struct GetDashboardsPaginatedRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_name: Option<String>,
    account_id: Option<String>,
    groupname: Option<String>,
    group_id: Option<String>,
    project_id: Option<i64>,
    order_by: Option<GetDashboardsPaginatedRequestOrderBy>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    status: Option<GetDashboardsPaginatedRequestStatus>,
    expand: Option<GetDashboardsPaginatedRequestExpand>,
}

impl<'a> GetDashboardsPaginatedRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            dashboard_name: None,
            account_id: None,
            groupname: None,
            group_id: None,
            project_id: None,
            order_by: None,
            start_at: None,
            max_results: None,
            status: None,
            expand: None,
        }
    }

    /// String used to perform a case-insensitive partial match with `name`.
    #[must_use]
    pub fn dashboard_name(mut self, value: impl Into<String>) -> Self {
        self.dashboard_name = Some(value.into());

        self
    }

    /// User account ID used to return dashboards with the matching `owner.accountId`. This parameter cannot be used with the `owner` parameter.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// As a group's name can change, use of `groupId` is recommended. Group name used to return dashboards that are shared with a group that matches `sharePermissions.group.name`. This parameter cannot be used with the `groupId` parameter.
    #[must_use]
    pub fn groupname(mut self, value: impl Into<String>) -> Self {
        self.groupname = Some(value.into());

        self
    }

    /// Group ID used to return dashboards that are shared with a group that matches `sharePermissions.group.groupId`. This parameter cannot be used with the `groupname` parameter.
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// Project ID used to returns dashboards that are shared with a project that matches `sharePermissions.project.id`.
    #[must_use]
    pub fn project_id(mut self, value: i64) -> Self {
        self.project_id = Some(value);

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by dashboard description. Note that this sort works independently of whether the expand to display the description field is in use.
    ///  *  `favourite_count` Sorts by dashboard popularity.
    ///  *  `id` Sorts by dashboard ID.
    ///  *  `is_favourite` Sorts by whether the dashboard is marked as a favorite.
    ///  *  `name` Sorts by dashboard name.
    ///  *  `owner` Sorts by dashboard owner name.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetDashboardsPaginatedRequestOrderBy>) -> Self {
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

    /// The status to filter by. It may be active, archived or deleted.
    #[must_use]
    pub fn status(mut self, value: impl Into<GetDashboardsPaginatedRequestStatus>) -> Self {
        self.status = Some(value.into());

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about dashboard in the response. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `description` Returns the description of the dashboard.
    ///  *  `owner` Returns the owner of the dashboard.
    ///  *  `viewUrl` Returns the URL that is used to view the dashboard.
    ///  *  `favourite` Returns `isFavourite`, an indicator of whether the user has set the dashboard as a favorite.
    ///  *  `favouritedCount` Returns `popularity`, a count of how many users have set this dashboard as a favorite.
    ///  *  `sharePermissions` Returns details of the share permissions defined for the dashboard.
    ///  *  `editPermissions` Returns details of the edit permissions defined for the dashboard.
    ///  *  `isWritable` Returns whether the current user has permission to edit the dashboard.
    #[must_use]
    pub fn expand(mut self, value: GetDashboardsPaginatedRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/dashboard/search".to_owned());

        if let Some(value) = &self.dashboard_name {
            config.query.push(("dashboardName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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
    pub async fn send(self) -> crate::core::Result<Page<Dashboard>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the keys of all properties for a dashboard item.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have read permission of the dashboard or have the dashboard shared with them.
pub struct GetDashboardItemPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
}

impl<'a> GetDashboardItemPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, dashboard_id: impl Into<String>, item_id: impl Into<String>) -> Self {
        Self { client, dashboard_id: dashboard_id.into(), item_id: item_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/dashboard/{}/items/{}/properties",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id)
            ),
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

/// Returns the key and value of a dashboard item property.
///
/// A dashboard item enables an app to add user-specific information to a user dashboard. Dashboard items are exposed to users as gadgets that users can add to their dashboards. For more information on how users do this, see [Adding and customizing gadgets](https://confluence.atlassian.com/x/7AeiLQ).
///
/// When an app creates a dashboard item it registers a callback to receive the dashboard item ID. The callback fires whenever the item is rendered or, where the item is configurable, the user edits the item. The app then uses this resource to store the item's content or configuration details. For more information on working with dashboard items, see [ Building a dashboard item for a JIRA Connect add-on](https://developer.atlassian.com/server/jira/platform/guide-building-a-dashboard-item-for-a-jira-connect-add-on-33746254/) and the [Dashboard Item](https://developer.atlassian.com/cloud/jira/platform/modules/dashboard-item/) documentation.
///
/// There is no resource to set or get dashboard items.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have read permission of the dashboard or have the dashboard shared with them.
pub struct GetDashboardItemPropertyRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
    property_key: String,
}

impl<'a> GetDashboardItemPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, dashboard_id: dashboard_id.into(), item_id: item_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/dashboard/{}/items/{}/properties/{}",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id),
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

/// Sets the value of a dashboard item property. Use this resource in apps to store custom data against a dashboard item.
///
/// A dashboard item enables an app to add user-specific information to a user dashboard. Dashboard items are exposed to users as gadgets that users can add to their dashboards. For more information on how users do this, see [Adding and customizing gadgets](https://confluence.atlassian.com/x/7AeiLQ).
///
/// When an app creates a dashboard item it registers a callback to receive the dashboard item ID. The callback fires whenever the item is rendered or, where the item is configurable, the user edits the item. The app then uses this resource to store the item's content or configuration details. For more information on working with dashboard items, see [ Building a dashboard item for a JIRA Connect add-on](https://developer.atlassian.com/server/jira/platform/guide-building-a-dashboard-item-for-a-jira-connect-add-on-33746254/) and the [Dashboard Item](https://developer.atlassian.com/cloud/jira/platform/modules/dashboard-item/) documentation.
///
/// There is no resource to set or get dashboard items.
///
/// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have edit permisson of the dashboard.
pub struct SetDashboardItemPropertyRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetDashboardItemPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            client,
            dashboard_id: dashboard_id.into(),
            item_id: item_id.into(),
            property_key: property_key.into(),
            body,
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/dashboard/{}/items/{}/properties/{}",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
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

/// Deletes a dashboard item property.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** The user must have edit permission of the dashboard.
pub struct DeleteDashboardItemPropertyRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
    property_key: String,
}

impl<'a> DeleteDashboardItemPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, dashboard_id: dashboard_id.into(), item_id: item_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/dashboard/{}/items/{}/properties/{}",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id),
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

/// Returns a dashboard.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
///
/// However, to get a dashboard, the dashboard must be shared with the user or the user must own it. Note, users with the *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) are considered owners of the System dashboard. The System dashboard is considered to be shared with all other users.
pub struct GetDashboardRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetDashboardRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/dashboard/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Dashboard> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
