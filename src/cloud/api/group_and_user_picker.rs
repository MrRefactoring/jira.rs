// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// The size of the avatar to return. If an invalid value is provided, the default value is used.
    pub enum FindUsersAndGroupsRequestAvatarSize {
        Xsmall => "xsmall",
        Xsmall2x => "xsmall@2x",
        Xsmall3x => "xsmall@3x",
        Small => "small",
        Small2x => "small@2x",
        Small3x => "small@3x",
        Medium => "medium",
        Medium2x => "medium@2x",
        Medium3x => "medium@3x",
        Large => "large",
        Large2x => "large@2x",
        Large3x => "large@3x",
        Xlarge => "xlarge",
        Xlarge2x => "xlarge@2x",
        Xlarge3x => "xlarge@3x",
        Xxlarge => "xxlarge",
        Xxlarge2x => "xxlarge@2x",
        Xxlarge3x => "xxlarge@3x",
        Xxxlarge => "xxxlarge",
        Xxxlarge2x => "xxxlarge@2x",
        Xxxlarge3x => "xxxlarge@3x",
    }
}

/// The GroupAndUserPicker operations.
pub struct GroupAndUserPickerService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GroupAndUserPickerService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of users and groups matching a string. The string is used:
    ///
    ///  *  for users, to find a case-insensitive match with display name and e-mail address. Note that if a user has hidden their email address in their user profile, partial matches of the email address will not find the user. An exact match is required.
    ///  *  for groups, to find a case-sensitive match with group name.
    ///
    /// For example, if the string *tin* is used, records with the display name *Tina*, email address *sarah@tinplatetraining.com*, and the group *accounting* would be returned.
    ///
    /// Optionally, the search can be refined to:
    ///
    ///  *  the projects and issue types associated with a custom field, such as a user picker. The search can then be further refined to return only users and groups that have permission to view specific:
    ///
    ///      *  projects.
    ///      *  issue types.
    ///
    ///     If multiple projects or issue types are specified, they must be a subset of those enabled for the custom field or no results are returned. For example, if a field is enabled for projects A, B, and C then the search could be limited to projects B and C. However, if the search is limited to projects B and D, nothing is returned.
    ///  *  not return Connect app users and groups.
    ///  *  return groups that have a case-insensitive match with the query.
    ///
    /// The primary use case for this resource is to populate a picker field suggestion list with users or groups. To this end, the returned object includes an `html` field for each list. This field highlights the matched query term in the item name with the HTML strong tag. Also, each list is wrapped in a response object that contains a header for use in a picker, specifically *Showing X of Y matching groups*.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/yodKLg).
    pub fn find_users_and_groups(&self, query: impl Into<String>) -> FindUsersAndGroupsRequest<'a> {
        FindUsersAndGroupsRequest::new(self.client, query)
    }
}

/// Returns a list of users and groups matching a string. The string is used:
///
///  *  for users, to find a case-insensitive match with display name and e-mail address. Note that if a user has hidden their email address in their user profile, partial matches of the email address will not find the user. An exact match is required.
///  *  for groups, to find a case-sensitive match with group name.
///
/// For example, if the string *tin* is used, records with the display name *Tina*, email address *sarah@tinplatetraining.com*, and the group *accounting* would be returned.
///
/// Optionally, the search can be refined to:
///
///  *  the projects and issue types associated with a custom field, such as a user picker. The search can then be further refined to return only users and groups that have permission to view specific:
///
///      *  projects.
///      *  issue types.
///
///     If multiple projects or issue types are specified, they must be a subset of those enabled for the custom field or no results are returned. For example, if a field is enabled for projects A, B, and C then the search could be limited to projects B and C. However, if the search is limited to projects B and D, nothing is returned.
///  *  not return Connect app users and groups.
///  *  return groups that have a case-insensitive match with the query.
///
/// The primary use case for this resource is to populate a picker field suggestion list with users or groups. To this end, the returned object includes an `html` field for each list. This field highlights the matched query term in the item name with the HTML strong tag. Also, each list is wrapped in a response object that contains a header for use in a picker, specifically *Showing X of Y matching groups*.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/yodKLg).
pub struct FindUsersAndGroupsRequest<'a> {
    client: &'a crate::core::Client,
    query: String,
    max_results: Option<i64>,
    show_avatar: Option<bool>,
    field_id: Option<String>,
    project_id: Option<Vec<String>>,
    issue_type_id: Option<Vec<String>>,
    avatar_size: Option<FindUsersAndGroupsRequestAvatarSize>,
    case_insensitive: Option<bool>,
    exclude_connect_addons: Option<bool>,
    include_ai_agents: Option<bool>,
}

impl<'a> FindUsersAndGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client, query: impl Into<String>) -> Self {
        Self {
            client,
            query: query.into(),
            max_results: None,
            show_avatar: None,
            field_id: None,
            project_id: None,
            issue_type_id: None,
            avatar_size: None,
            case_insensitive: None,
            exclude_connect_addons: None,
            include_ai_agents: None,
        }
    }

    /// The maximum number of items to return in each list.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Whether the user avatar should be returned. If an invalid value is provided, the default value is used.
    #[must_use]
    pub fn show_avatar(mut self, value: bool) -> Self {
        self.show_avatar = Some(value);

        self
    }

    /// The custom field ID of the field this request is for.
    #[must_use]
    pub fn field_id(mut self, value: impl Into<String>) -> Self {
        self.field_id = Some(value.into());

        self
    }

    /// The ID of a project that returned users and groups must have permission to view. To include multiple projects, provide an ampersand-separated list. For example, `projectId=10000&projectId=10001`. This parameter is only used when `fieldId` is present.
    #[must_use]
    pub fn project_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.project_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The ID of an issue type that returned users and groups must have permission to view. To include multiple issue types, provide an ampersand-separated list. For example, `issueTypeId=10000&issueTypeId=10001`. Special values, such as `-1` (all standard issue types) and `-2` (all subtask issue types), are supported. This parameter is only used when `fieldId` is present.
    #[must_use]
    pub fn issue_type_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.issue_type_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The size of the avatar to return. If an invalid value is provided, the default value is used.
    #[must_use]
    pub fn avatar_size(mut self, value: impl Into<FindUsersAndGroupsRequestAvatarSize>) -> Self {
        self.avatar_size = Some(value.into());

        self
    }

    /// Whether the search for groups should be case insensitive.
    #[must_use]
    pub fn case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = Some(value);

        self
    }

    /// Whether Connect app users and groups should be excluded from the search results. If an invalid value is provided, the default value is used.
    #[must_use]
    pub fn exclude_connect_addons(mut self, value: bool) -> Self {
        self.exclude_connect_addons = Some(value);

        self
    }

    /// Whether AI Agents should be included in the search results. If an invalid value is provided, the default value is used.
    #[must_use]
    pub fn include_ai_agents(mut self, value: bool) -> Self {
        self.include_ai_agents = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/groupuserpicker".to_owned());

        config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(self.query.clone())));

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.show_avatar {
            config.query.push(("showAvatar".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.field_id {
            config.query.push(("fieldId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.issue_type_id {
            config.query.push(("issueTypeId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.avatar_size {
            config.query.push(("avatarSize".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.case_insensitive {
            config.query.push(("caseInsensitive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.exclude_connect_addons {
            config.query.push(("excludeConnectAddons".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.include_ai_agents {
            config.query.push(("includeAiAgents".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<FoundUsersAndGroups> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
