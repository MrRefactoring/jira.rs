# Changelog

All notable changes to this crate are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) — before `1.0.0`, a breaking change raises the minor
version.

## [0.2.0] - 2026-10-05

### Added

- `cargo xtask streams --check` requires requests that look paginated from their setters or response fields across all
  surfaces to have `stream()` or a reason in `xtask/streams.txt`; it follows `Option` and one nested model level,
  and fails on unresolved requests.
- [The Cloud and Data Center parity matrix](https://github.com/MrRefactoring/jira.rs/blob/master/docs/parity.md),
  showing paired operations and the differences that remain.

### Changed

- Data Center follows Cloud for the same operation: service accessor, method name, positional parameters and model
  type name now match where the APIs permit it. Code moving between `CloudClient` or `AgileClient` and `ServerClient`
  can keep the call shape; platform-only operations stay on their own surface. Existing Data Center calls and type
  references using the 0.1.1 names or signatures must be updated with the tables below.
- Data Center `projects().search_projects()` now calls `GET /rest/api/2/project` and returns every visible project in
  one page. The project picker is `projects().get_projects_picker()`.

<details>
<summary>Operations and accessors</summary>

| 0.1.1 | 0.2.0 |
|---|---|
| `application_properties().get_application_properties` | `jira_settings().get_application_property` |
| `application_properties().get_advanced_settings` | `jira_settings().get_advanced_settings` |
| `application_properties().set_property_via_restful_table` | `jira_settings().set_application_property` |
| `application_roles().get_all` | `application_roles().get_all_application_roles` |
| `board().get_board_configuration` | `board().get_configuration` |
| `board().get_issues_for_board_epic` | `board().get_board_issues_for_epic` |
| `board().get_issues_for_board_sprint` | `board().get_board_issues_for_sprint` |
| `configuration().get_configuration` | `jira_settings().get_configuration` |
| `dashboards().list` | `dashboards().get_all_dashboards` |
| `filters().get_default_share_scope` | `filter_sharing().get_default_share_scope` |
| `filters().set_default_share_scope` | `filter_sharing().set_default_share_scope` |
| `filters().edit_filter` | `filters().update_filter` |
| `filters().get_filter_columns` | `filters().get_columns` |
| `filters().get_share_permissions` | `filter_sharing().get_share_permissions` |
| `filters().add_share_permission` | `filter_sharing().add_share_permission` |
| `filters().get_share_permission` | `filter_sharing().get_share_permission` |
| `filters().delete_share_permission` | `filter_sharing().delete_share_permission` |
| `issue_comments().get_comment_property_keys` | `issue_comment_properties().get_comment_property_keys` |
| `issue_comments().get_comment_property` | `issue_comment_properties().get_comment_property` |
| `issue_comments().set_comment_property` | `issue_comment_properties().set_comment_property` |
| `issue_comments().delete_comment_property` | `issue_comment_properties().delete_comment_property` |
| `issue_search().search` | `issue_search().search_issues` |
| `issue_types().get_issue_type_property_keys` | `issue_type_properties().get_issue_type_property_keys` |
| `issue_types().get_issue_type_property` | `issue_type_properties().get_issue_type_property` |
| `issue_types().set_issue_type_property` | `issue_type_properties().set_issue_type_property` |
| `issue_types().delete_issue_type_property` | `issue_type_properties().delete_issue_type_property` |
| `issues().rank_issues` | `issue().rank_issues` |
| `issues().get_agile_issue` | `issue().get_issue` |
| `issues().get_issue_estimation_for_board` | `issue().get_issue_estimation_for_board` |
| `issues().estimate_issue_for_board` | `issue().estimate_issue_for_board` |
| `issues().get_create_issue_meta_project_issue_types` | `issues().get_create_issue_meta_issue_types` |
| `issues().get_create_issue_meta_fields` | `issues().get_create_issue_meta_issue_type_id` |
| `issues().get_issue_picker_resource` | `issue_search().get_issue_picker_resource` |
| `issues().assign` | `issues().assign_issue` |
| `issues().add_attachment` | `issue_attachments().add_attachment` |
| `issues().get_comments` | `issue_comments().get_comments` |
| `issues().add_comment` | `issue_comments().add_comment` |
| `issues().get_comment` | `issue_comments().get_comment` |
| `issues().update_comment` | `issue_comments().update_comment` |
| `issues().delete_comment` | `issue_comments().delete_comment` |
| `issues().get_issue_property_keys` | `issue_properties().get_issue_property_keys` |
| `issues().get_issue_property` | `issue_properties().get_issue_property` |
| `issues().set_issue_property` | `issue_properties().set_issue_property` |
| `issues().delete_issue_property` | `issue_properties().delete_issue_property` |
| `issues().get_remote_issue_links` | `issue_remote_links().get_remote_issue_links` |
| `issues().create_or_update_remote_issue_link` | `issue_remote_links().create_or_update_remote_issue_link` |
| `issues().delete_remote_issue_link_by_global_id` | `issue_remote_links().delete_remote_issue_link_by_global_id` |
| `issues().get_remote_issue_link_by_id` | `issue_remote_links().get_remote_issue_link_by_id` |
| `issues().update_remote_issue_link` | `issue_remote_links().update_remote_issue_link` |
| `issues().delete_remote_issue_link_by_id` | `issue_remote_links().delete_remote_issue_link_by_id` |
| `issues().get_votes` | `issue_votes().get_votes` |
| `issues().add_vote` | `issue_votes().add_vote` |
| `issues().remove_vote` | `issue_votes().remove_vote` |
| `issues().get_issue_watchers` | `issue_watchers().get_issue_watchers` |
| `issues().add_watcher` | `issue_watchers().add_watcher` |
| `issues().remove_watcher` | `issue_watchers().remove_watcher` |
| `issues().get_issue_worklog` | `issue_worklogs().get_issue_worklog` |
| `issues().add_worklog` | `issue_worklogs().add_worklog` |
| `issues().get_worklog` | `issue_worklogs().get_worklog` |
| `issues().update_worklog` | `issue_worklogs().update_worklog` |
| `issues().delete_worklog` | `issue_worklogs().delete_worklog` |
| `jira_settings().get_issue_navigator_default_columns` | `issue_navigator_settings().get_issue_navigator_default_columns` |
| `jira_settings().set_issue_navigator_default_columns_form` | `issue_navigator_settings().set_issue_navigator_default_columns` |
| `my_preferences().get_preference` | `myself().get_preference` |
| `my_preferences().set_preference` | `myself().set_preference` |
| `my_preferences().remove_preference` | `myself().remove_preference` |
| `permission_schemes().get_permission_schemes` | `permission_schemes().get_all_permission_schemes` |
| `permissions().get_permissions` | `permissions().get_my_permissions` |
| `project_roles().get_project_roles_by_id` | `project_roles().get_project_role_by_id` |
| `project_roles().get_project_role_actors_for_role` | `project_role_actors().get_project_role_actors_for_role` |
| `project_roles().add_project_role_actors_to_role` | `project_role_actors().add_project_role_actors_to_role` |
| `project_roles().delete_project_role_actors_from_role` | `project_role_actors().delete_project_role_actors_from_role` |
| `project_versions().merge` | `project_versions().merge_versions` |
| `project_versions().delete_version_and_swap` | `project_versions().delete_and_replace_version` |
| `projects().get_all_project_types` | `project_types().get_all_project_types` |
| `projects().get_project_type_by_key` | `project_types().get_project_type_by_key` |
| `projects().get_accessible_project_type_by_key` | `project_types().get_accessible_project_type_by_key` |
| `projects().update_project_avatar` | `project_avatars().update_project_avatar` |
| `projects().delete_project_avatar` | `project_avatars().delete_project_avatar` |
| `projects().get_all_project_avatars` | `project_avatars().get_all_project_avatars` |
| `projects().get_project_components` | `project_components().get_project_components` |
| `projects().get_project_property_keys` | `project_properties().get_project_property_keys` |
| `projects().get_project_property` | `project_properties().get_project_property` |
| `projects().set_project_property` | `project_properties().set_project_property` |
| `projects().delete_project_property` | `project_properties().delete_project_property` |
| `projects().get_project_roles` | `project_roles().get_project_roles` |
| `projects().get_project_role` | `project_roles().get_project_role` |
| `projects().add_actor_users` | `project_role_actors().add_actor_users` |
| `projects().set_actors` | `project_role_actors().set_actors` |
| `projects().delete_actor` | `project_role_actors().delete_actor` |
| `projects().get_project_versions_paginated` | `project_versions().get_project_versions_paginated` |
| `projects().get_project_versions` | `project_versions().get_project_versions` |
| `projects().get_project_issue_security_scheme` | `project_permission_schemes().get_project_issue_security_scheme` |
| `projects().get_project_notification_scheme` | `projects().get_notification_scheme_for_project` |
| `projects().get_assigned_permission_scheme` | `project_permission_schemes().get_assigned_permission_scheme` |
| `projects().assign_permission_scheme` | `project_permission_schemes().assign_permission_scheme` |
| `projects().get_security_levels_for_project` | `project_permission_schemes().get_security_levels_for_project` |
| `projects().search_for_projects` | `projects().get_projects_picker` |
| `screens().get_all_screens` | `screens().get_screens` |
| `screens().get_fields_to_add` | `screens().get_available_screen_fields` |
| `screens().get_all_tabs` | `screen_tabs().get_all_screen_tabs` |
| `screens().add_tab` | `screen_tabs().add_screen_tab` |
| `screens().rename_tab` | `screen_tabs().rename_screen_tab` |
| `screens().delete_tab` | `screen_tabs().delete_screen_tab` |
| `screens().get_all_fields` | `screen_tab_fields().get_all_screen_tab_fields` |
| `screens().add_field` | `screen_tab_fields().add_screen_tab_field` |
| `screens().remove_field` | `screen_tab_fields().remove_screen_tab_field` |
| `screens().move_field` | `screen_tab_fields().move_screen_tab_field` |
| `screens().move_tab` | `screen_tabs().move_screen_tab` |
| `sprint().move_issues_to_sprint` | `sprint().move_issues_to_sprint_and_rank` |
| `sprint().get_sprint_property_keys` | `sprint().get_properties_keys` |
| `sprint().get_sprint_property` | `sprint().get_property` |
| `sprint().set_sprint_property` | `sprint().set_property` |
| `sprint().delete_sprint_property` | `sprint().delete_property` |
| `users().find_bulk_assignable_users` | `user_search().find_bulk_assignable_users` |
| `users().find_assignable_users` | `user_search().find_assignable_users` |
| `users().default_columns` | `users().get_user_default_columns` |
| `users().set_columns_url_encoded` | `users().set_user_columns` |
| `users().find_users_for_picker` | `user_search().find_users_for_picker` |
| `users().get_user_property_keys` | `user_properties().get_user_property_keys` |
| `users().get_user_property` | `user_properties().get_user_property` |
| `users().set_user_property` | `user_properties().set_user_property` |
| `users().delete_user_property` | `user_properties().delete_user_property` |
| `users().find_users` | `user_search().find_users` |
| `users().find_users_with_browse_permission` | `user_search().find_users_with_browse_permission` |
| `workflow_schemes().create_scheme` | `workflow_schemes().create_workflow_scheme` |
| `workflow_schemes().get_by_id` | `workflow_schemes().get_workflow_scheme` |
| `workflow_schemes().delete_scheme` | `workflow_schemes().delete_workflow_scheme` |
| `workflow_schemes().create_draft_for_parent` | `workflow_scheme_drafts().create_workflow_scheme_draft_from_parent` |
| `workflow_schemes().get_default` | `workflow_schemes().get_default_workflow` |
| `workflow_schemes().update_default` | `workflow_schemes().update_default_workflow` |
| `workflow_schemes().delete_default` | `workflow_schemes().delete_default_workflow` |
| `workflow_schemes().get_draft_by_id` | `workflow_scheme_drafts().get_workflow_scheme_draft` |
| `workflow_schemes().update_draft` | `workflow_scheme_drafts().update_workflow_scheme_draft` |
| `workflow_schemes().delete_draft_by_id` | `workflow_scheme_drafts().delete_workflow_scheme_draft` |
| `workflow_schemes().get_draft_default` | `workflow_scheme_drafts().get_draft_default_workflow` |
| `workflow_schemes().update_draft_default` | `workflow_scheme_drafts().update_draft_default_workflow` |
| `workflow_schemes().delete_draft_default` | `workflow_scheme_drafts().delete_draft_default_workflow` |
| `workflow_schemes().get_draft_issue_type` | `workflow_scheme_drafts().get_workflow_scheme_draft_issue_type` |
| `workflow_schemes().set_draft_issue_type` | `workflow_scheme_drafts().set_workflow_scheme_draft_issue_type` |
| `workflow_schemes().delete_draft_issue_type` | `workflow_scheme_drafts().delete_workflow_scheme_draft_issue_type` |
| `workflow_schemes().get_draft_workflow` | `workflow_scheme_drafts().get_draft_workflow` |
| `workflow_schemes().update_draft_workflow_mapping` | `workflow_scheme_drafts().update_draft_workflow_mapping` |
| `workflow_schemes().delete_draft_workflow_mapping` | `workflow_scheme_drafts().delete_draft_workflow_mapping` |
| `workflow_schemes().set_issue_type` | `workflow_schemes().set_workflow_scheme_issue_type` |
| `projects().get_all_projects` | `projects().search_projects` |
| `issue_search().search_using_search_request` | `issue_search().search_issues_post` |

</details>

<details>
<summary>Signatures</summary>

| 0.1.1 | 0.2.0 |
|---|---|
| `DELETE /rest/api/2/universal_avatar/type/{}/owner/{}/avatar/{}`<br>`avatars().delete_avatar(id: i64, r#type: impl Into<String>, owning_object_id: impl Into<String>)` | `avatars().delete_avatar(r#type: impl Into<String>, owning_object_id: impl Into<String>, id: i64)` |
| `GET /rest/api/2/universal_avatar/type/{}/owner/{}`<br>`avatars().get_avatars(r#type: impl Into<String>, owning_object_id: impl Into<String>)` | `avatars().get_avatars(r#type: impl Into<String>, entity_id: impl Into<String>)` |
| `DELETE /rest/agile/1.0/board/{}/properties/{}`<br>`board().delete_board_property(property_key: impl Into<String>, board_id: i64)` | `board().delete_board_property(board_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/agile/1.0/board/{}/sprint`<br>`board().get_all_sprints(board_id: i64).state(GetAllSprintsRequestState)` | `board().get_all_sprints(board_id: i64).state(impl Into<GetAllSprintsRequestState>)` |
| `GET /rest/agile/1.0/board/{}/epic/{}/issue`<br>`board().get_board_issues_for_epic(epic_id: i64, board_id: i64).fields(GetBoardIssuesForEpicRequestFields)` | `board().get_board_issues_for_epic(board_id: i64, epic_id: i64).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `GET /rest/agile/1.0/board/{}/sprint/{}/issue`<br>`board().get_board_issues_for_sprint(sprint_id: i64, board_id: i64).fields(GetBoardIssuesForSprintRequestFields)` | `board().get_board_issues_for_sprint(board_id: i64, sprint_id: i64).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `GET /rest/agile/1.0/board/{}/properties/{}`<br>`board().get_board_property(property_key: impl Into<String>, board_id: i64)` | `board().get_board_property(board_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/agile/1.0/board/{}/properties`<br>`board().get_board_property_keys(board_id: i64)` | `board().get_board_property_keys(board_id: impl Into<String>)` |
| `GET /rest/agile/1.0/board/{}/backlog`<br>`board().get_issues_for_backlog(board_id: i64).fields(GetIssuesForBacklogRequestFields)` | `board().get_issues_for_backlog(board_id: i64).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `GET /rest/agile/1.0/board/{}/issue`<br>`board().get_issues_for_board(board_id: i64).fields(GetIssuesForBoardRequestFields)` | `board().get_issues_for_board(board_id: i64).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `GET /rest/agile/1.0/board/{}/epic/none/issue`<br>`board().get_issues_without_epic_for_board(board_id: i64).fields(GetIssuesWithoutEpicForBoardRequestFields)` | `board().get_issues_without_epic_for_board(board_id: i64).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `PUT /rest/agile/1.0/board/{}/properties/{}`<br>`board().set_board_property(property_key: impl Into<String>, board_id: i64, body: std::collections::HashMap<String, serde_json::Value>)` | `board().set_board_property(board_id: impl Into<String>, property_key: impl Into<String>, property_value: std::collections::HashMap<String, serde_json::Value>)` |
| `DELETE /rest/api/2/dashboard/{}/items/{}/properties/{}`<br>`dashboards().delete_dashboard_item_property(property_key: impl Into<String>, item_id: impl Into<String>, dashboard_id: impl Into<String>)` | `dashboards().delete_dashboard_item_property(dashboard_id: impl Into<String>, item_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/api/2/dashboard/{}/items/{}/properties/{}`<br>`dashboards().get_dashboard_item_property(property_key: impl Into<String>, item_id: impl Into<String>, dashboard_id: impl Into<String>)` | `dashboards().get_dashboard_item_property(dashboard_id: impl Into<String>, item_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/api/2/dashboard/{}/items/{}/properties`<br>`dashboards().get_dashboard_item_property_keys(item_id: impl Into<String>, dashboard_id: impl Into<String>)` | `dashboards().get_dashboard_item_property_keys(dashboard_id: impl Into<String>, item_id: impl Into<String>)` |
| `PUT /rest/api/2/dashboard/{}/items/{}/properties/{}`<br>`dashboards().set_dashboard_item_property(property_key: impl Into<String>, item_id: impl Into<String>, dashboard_id: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` | `dashboards().set_dashboard_item_property(dashboard_id: impl Into<String>, item_id: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` |
| `GET /rest/agile/1.0/epic/{}/issue`<br>`epic().get_issues_for_epic(epic_id_or_key: impl Into<String>).fields(GetIssuesForEpicRequestFields)` | `epic().get_issues_for_epic(epic_id_or_key: impl Into<String>).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `GET /rest/agile/1.0/epic/none/issue`<br>`epic().get_issues_without_epic().fields(GetIssuesWithoutEpicRequestFields)` | `epic().get_issues_without_epic().fields(impl IntoIterator<Item = impl Into<String>>)` |
| `POST /rest/api/2/filter/{}/permission`<br>`filter_sharing().add_share_permission(id: impl Into<String>).share_permission_input(SharePermissionInput)` | `filter_sharing().add_share_permission(id: i64, share_permission_input: SharePermissionInput)` |
| `DELETE /rest/api/2/filter/{}/permission/{}`<br>`filter_sharing().delete_share_permission(id: impl Into<String>, permission_id: impl Into<String>)` | `filter_sharing().delete_share_permission(id: i64, permission_id: i64)` |
| `GET /rest/api/2/filter/{}/permission/{}`<br>`filter_sharing().get_share_permission(permission_id: impl Into<String>, id: impl Into<String>)` | `filter_sharing().get_share_permission(id: i64, permission_id: i64)` |
| `GET /rest/api/2/filter/{}/permission`<br>`filter_sharing().get_share_permissions(id: impl Into<String>)` | `filter_sharing().get_share_permissions(id: i64)` |
| `PUT /rest/api/2/filter/defaultShareScope`<br>`filter_sharing().set_default_share_scope().default_share_scope(DefaultShareScope)` | `filter_sharing().set_default_share_scope(default_share_scope: DefaultShareScope)` |
| `POST /rest/api/2/filter`<br>`filters().create_filter().filter(Filter)` | `filters().create_filter(filter: Filter)` |
| `DELETE /rest/api/2/filter/{}`<br>`filters().delete_filter(id: impl Into<String>)` | `filters().delete_filter(id: i64)` |
| `GET /rest/api/2/filter/{}/columns`<br>`filters().get_columns(id: impl Into<String>)` | `filters().get_columns(id: i64)` |
| `GET /rest/api/2/filter/{}`<br>`filters().get_filter(id: impl Into<String>)` | `filters().get_filter(id: i64)` |
| `DELETE /rest/api/2/filter/{}/columns`<br>`filters().reset_columns(id: impl Into<String>)` | `filters().reset_columns(id: i64)` |
| `PUT /rest/api/2/filter/{}/columns`<br>`filters().set_columns(id: impl Into<String>).columns(impl IntoIterator<Item = impl Into<String>>)` | `filters().set_columns(id: i64, column_request_body: SetColumnsRequestColumnRequestBody)` |
| `PUT /rest/api/2/filter/{}`<br>`filters().update_filter(id: impl Into<String>).body(Filter)` | `filters().update_filter(id: i64, body: Filter)` |
| `GET /rest/api/2/groupuserpicker`<br>`group_and_user_picker().find_users_and_groups().query(impl Into<String>).show_avatar(impl Into<String>)` | `group_and_user_picker().find_users_and_groups(query: impl Into<String>).show_avatar(bool)` |
| `POST /rest/api/2/group/user`<br>`groups().add_user_to_group(groupname: impl Into<String>).update_user_to_group(UpdateUserToGroup)` | `groups().add_user_to_group(groupname: impl Into<String>, update_user_to_group: UpdateUserToGroup)` |
| `POST /rest/api/2/group`<br>`groups().create_group().add_group(AddGroup)` | `groups().create_group(add_group: AddGroup)` |
| `GET /rest/api/2/group/member`<br>`groups().get_users_from_group(groupname: impl Into<String>).include_inactive_users(impl Into<String>)` | `groups().get_users_from_group(groupname: impl Into<String>).include_inactive_users(bool)` |
| `GET /rest/agile/1.0/issue/{}`<br>`issue().get_issue(issue_id_or_key: impl Into<String>).fields(IssueGetIssueRequestFields)` | `issue().get_issue(issue_id_or_key: impl Into<String>).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `DELETE /rest/api/2/comment/{}/properties/{}`<br>`issue_comment_properties().delete_comment_property(property_key: impl Into<String>, comment_id: impl Into<String>)` | `issue_comment_properties().delete_comment_property(comment_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/api/2/comment/{}/properties/{}`<br>`issue_comment_properties().get_comment_property(property_key: impl Into<String>, comment_id: impl Into<String>)` | `issue_comment_properties().get_comment_property(comment_id: impl Into<String>, property_key: impl Into<String>)` |
| `PUT /rest/api/2/comment/{}/properties/{}`<br>`issue_comment_properties().set_comment_property(property_key: impl Into<String>, comment_id: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` | `issue_comment_properties().set_comment_property(comment_id: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` |
| `POST /rest/api/2/issue/{}/comment`<br>`issue_comments().add_comment(issue_id_or_key: impl Into<String>).comment_json(CommentJson).expand(impl Into<String>)` | `issue_comments().add_comment(issue_id_or_key: impl Into<String>, comment_input: CommentJson).expand(impl Into<AddCommentRequestExpand>)` |
| `GET /rest/api/2/issue/{}/comment/{}`<br>`issue_comments().get_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(impl Into<String>)` | `issue_comments().get_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(impl Into<GetCommentRequestExpand>)` |
| `GET /rest/api/2/issue/{}/comment`<br>`issue_comments().get_comments(issue_id_or_key: impl Into<String>).expand(impl Into<String>)` | `issue_comments().get_comments(issue_id_or_key: impl Into<String>).expand(impl Into<GetCommentsRequestExpand>)` |
| `PUT /rest/api/2/issue/{}/comment/{}`<br>`issue_comments().update_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).body(CommentJson).expand(impl Into<String>)` | `issue_comments().update_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: CommentJson).expand(impl Into<UpdateCommentRequestExpand>)` |
| `POST /rest/api/2/field`<br>`issue_fields().create_custom_field().custom_field_definition_json(CustomFieldDefinitionJson)` | `issue_fields().create_custom_field(custom_field_definition: CustomFieldDefinitionJson)` |
| `POST /rest/api/2/issueLinkType`<br>`issue_link_types().create_issue_link_type(issue_link_type_json: IssueLinkTypeJson)` | `issue_link_types().create_issue_link_type(issue_link_type: IssueLinkTypeJson)` |
| `PUT /rest/api/2/issueLinkType/{}`<br>`issue_link_types().update_issue_link_type(issue_link_type_id: impl Into<String>, issue_link_type_json: IssueLinkTypeJson)` | `issue_link_types().update_issue_link_type(issue_link_type_id: impl Into<String>, issue_link_type: IssueLinkTypeJson)` |
| `POST /rest/api/2/issueLink`<br>`issue_links().link_issues(link_issue_request_json: LinkIssueRequestJson)` | `issue_links().link_issues(link_issue_request: LinkIssueRequestJson)` |
| `PUT /rest/api/2/settings/columns`<br>`issue_navigator_settings().set_issue_navigator_default_columns().columns(impl IntoIterator<Item = impl Into<String>>)` | `issue_navigator_settings().set_issue_navigator_default_columns(column_request_body: SetIssueNavigatorDefaultColumnsRequestColumnRequestBody)` |
| `GET /rest/api/2/notificationscheme/{}`<br>`issue_notification_schemes().get_notification_scheme(id: i64).expand(impl Into<String>)` | `issue_notification_schemes().get_notification_scheme(id: i64).expand(impl Into<GetNotificationSchemeRequestExpand>)` |
| `GET /rest/api/2/notificationscheme`<br>`issue_notification_schemes().get_notification_schemes().expand(impl Into<String>)` | `issue_notification_schemes().get_notification_schemes().expand(impl Into<GetNotificationSchemesRequestExpand>)` |
| `DELETE /rest/api/2/issue/{}/properties/{}`<br>`issue_properties().delete_issue_property(property_key: impl Into<String>, issue_id_or_key: impl Into<String>)` | `issue_properties().delete_issue_property(issue_id_or_key: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/api/2/issue/{}/properties/{}`<br>`issue_properties().get_issue_property(property_key: impl Into<String>, issue_id_or_key: impl Into<String>)` | `issue_properties().get_issue_property(issue_id_or_key: impl Into<String>, property_key: impl Into<String>)` |
| `PUT /rest/api/2/issue/{}/properties/{}`<br>`issue_properties().set_issue_property(property_key: impl Into<String>, issue_id_or_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` | `issue_properties().set_issue_property(issue_id_or_key: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` |
| `POST /rest/api/2/issue/{}/remotelink`<br>`issue_remote_links().create_or_update_remote_issue_link(issue_id_or_key: impl Into<String>).remote_issue_link_create_or_update_request(RemoteIssueLinkCreateOrUpdateRequest)` | `issue_remote_links().create_or_update_remote_issue_link(issue_id_or_key: impl Into<String>, remote_issue_link_request: RemoteIssueLinkCreateOrUpdateRequest)` |
| `DELETE /rest/api/2/issue/{}/remotelink/{}`<br>`issue_remote_links().delete_remote_issue_link_by_id(link_id: impl Into<String>, issue_id_or_key: impl Into<String>)` | `issue_remote_links().delete_remote_issue_link_by_id(issue_id_or_key: impl Into<String>, link_id: impl Into<String>)` |
| `GET /rest/api/2/issue/{}/remotelink/{}`<br>`issue_remote_links().get_remote_issue_link_by_id(link_id: impl Into<String>, issue_id_or_key: impl Into<String>)` | `issue_remote_links().get_remote_issue_link_by_id(issue_id_or_key: impl Into<String>, link_id: impl Into<String>)` |
| `PUT /rest/api/2/issue/{}/remotelink/{}`<br>`issue_remote_links().update_remote_issue_link(link_id: impl Into<String>, issue_id_or_key: impl Into<String>).remote_issue_link_create_or_update_request(RemoteIssueLinkCreateOrUpdateRequest)` | `issue_remote_links().update_remote_issue_link(issue_id_or_key: impl Into<String>, link_id: impl Into<String>, remote_issue_link_request: RemoteIssueLinkCreateOrUpdateRequest)` |
| `GET /rest/api/2/issue/picker`<br>`issue_search().get_issue_picker_resource().show_sub_task_parent(impl Into<String>).show_sub_tasks(impl Into<String>)` | `issue_search().get_issue_picker_resource().show_sub_task_parent(bool).show_sub_tasks(bool)` |
| `GET /rest/api/2/search`<br>`issue_search().search_issues().fields(SearchIssuesRequestFields)` | `issue_search().search_issues().fields(impl IntoIterator<Item = impl Into<String>>)` |
| `POST /rest/api/2/search`<br>`issue_search().search_issues_post(search_request: SearchRequest)` | `issue_search().search_issues_post(search_and_reconcile_request: SearchRequest)` |
| `GET /rest/api/2/issuesecurityschemes/{}`<br>`issue_security_schemes().get_issue_security_scheme(id: impl Into<String>)` | `issue_security_schemes().get_issue_security_scheme(id: i64)` |
| `DELETE /rest/api/2/issuetype/{}/properties/{}`<br>`issue_type_properties().delete_issue_type_property(property_key: impl Into<String>, issue_type_id: impl Into<String>)` | `issue_type_properties().delete_issue_type_property(issue_type_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/api/2/issuetype/{}/properties/{}`<br>`issue_type_properties().get_issue_type_property(property_key: impl Into<String>, issue_type_id: impl Into<String>)` | `issue_type_properties().get_issue_type_property(issue_type_id: impl Into<String>, property_key: impl Into<String>)` |
| `PUT /rest/api/2/issuetype/{}/properties/{}`<br>`issue_type_properties().set_issue_type_property(property_key: impl Into<String>, issue_type_id: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` | `issue_type_properties().set_issue_type_property(issue_type_id: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` |
| `POST /rest/api/2/issuetypescheme`<br>`issue_type_schemes().create_issue_type_scheme(issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate)` | `issue_type_schemes().create_issue_type_scheme(issue_type_scheme_details: IssueTypeSchemeCreateUpdate)` |
| `DELETE /rest/api/2/issuetypescheme/{}`<br>`issue_type_schemes().delete_issue_type_scheme(scheme_id: impl Into<String>)` | `issue_type_schemes().delete_issue_type_scheme(issue_type_scheme_id: i64)` |
| `DELETE /rest/api/2/issuetypescheme/{}/associations/{}`<br>`issue_type_schemes().remove_project_association(proj_id_or_key: impl Into<String>, scheme_id: impl Into<String>)` | `issue_type_schemes().remove_project_association(scheme_id: impl Into<String>, proj_id_or_key: impl Into<String>)` |
| `PUT /rest/api/2/issuetypescheme/{}`<br>`issue_type_schemes().update_issue_type_scheme(scheme_id: impl Into<String>, issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate)` | `issue_type_schemes().update_issue_type_scheme(issue_type_scheme_id: i64, issue_type_scheme_update_details: IssueTypeSchemeCreateUpdate)` |
| `POST /rest/api/2/issue/{}/watchers`<br>`issue_watchers().add_watcher(issue_id_or_key: impl Into<String>).body(impl Into<String>)` | `issue_watchers().add_watcher(issue_id_or_key: impl Into<String>, body: impl Into<String>)` |
| `POST /rest/api/2/issue/{}/worklog`<br>`issue_worklogs().add_worklog(issue_id_or_key: impl Into<String>).adjust_estimate(impl Into<String>).worklog(Worklog)` | `issue_worklogs().add_worklog(issue_id_or_key: impl Into<String>, worklog_input: Worklog).adjust_estimate(impl Into<AddWorklogRequestAdjustEstimate>)` |
| `DELETE /rest/api/2/issue/{}/worklog/{}`<br>`issue_worklogs().delete_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>).adjust_estimate(impl Into<String>)` | `issue_worklogs().delete_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>).adjust_estimate(impl Into<DeleteWorklogRequestAdjustEstimate>)` |
| `PUT /rest/api/2/issue/{}/worklog/{}`<br>`issue_worklogs().update_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>).adjust_estimate(impl Into<String>).body(Worklog)` | `issue_worklogs().update_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: Worklog).adjust_estimate(impl Into<UpdateWorklogRequestAdjustEstimate>)` |
| `PUT /rest/api/2/issue/{}/assignee`<br>`issues().assign_issue(issue_id_or_key: impl Into<String>).user(User)` | `issues().assign_issue(issue_id_or_key: impl Into<String>, user: User)` |
| `POST /rest/api/2/issue`<br>`issues().create_issue().issue_update(IssueUpdate)` | `issues().create_issue(issue_update_details: IssueUpdate)` |
| `POST /rest/api/2/issue/bulk`<br>`issues().create_issues().issues_update(IssuesUpdate)` | `issues().create_issues(issues_update: IssuesUpdate)` |
| `DELETE /rest/api/2/issue/{}`<br>`issues().delete_issue(issue_id_or_key: impl Into<String>).delete_subtasks(impl Into<String>)` | `issues().delete_issue(issue_id_or_key: impl Into<String>).delete_subtasks(impl Into<DeleteIssueRequestDeleteSubtasks>)` |
| `POST /rest/api/2/issue/{}/transitions`<br>`issues().do_transition(issue_id_or_key: impl Into<String>).issue_update(IssueUpdate)` | `issues().do_transition(issue_id_or_key: impl Into<String>, issue_update_details: IssueUpdate)` |
| `PUT /rest/api/2/issue/{}`<br>`issues().edit_issue(issue_id_or_key: impl Into<String>).issue_update(IssueUpdate).notify_users(impl Into<String>)` | `issues().edit_issue(issue_id_or_key: impl Into<String>, issue_update_details: IssueUpdate).notify_users(bool)` |
| `GET /rest/api/2/issue/createmeta/{}/issuetypes/{}`<br>`issues().get_create_issue_meta_issue_type_id(issue_type_id: impl Into<String>, project_id_or_key: impl Into<String>)` | `issues().get_create_issue_meta_issue_type_id(project_id_or_key: impl Into<String>, issue_type_id: impl Into<String>)` |
| `GET /rest/api/2/issue/{}`<br>`issues().get_issue(issue_id_or_key: impl Into<String>).expand(impl Into<String>).fields(impl Into<String>).properties(impl Into<String>).update_history(impl Into<String>)` | `issues().get_issue(issue_id_or_key: impl Into<String>).expand(IssuesGetIssueRequestExpand).fields(impl IntoIterator<Item = impl Into<String>>).properties(impl IntoIterator<Item = impl Into<String>>).update_history(bool)` |
| `POST /rest/api/2/issue/{}/notify`<br>`issues().notify(issue_id_or_key: impl Into<String>).notification_json(NotificationJson)` | `issues().notify(issue_id_or_key: impl Into<String>, notification: NotificationJson)` |
| `GET /rest/api/2/mypreferences`<br>`myself().get_preference().key(impl Into<String>)` | `myself().get_preference(key: impl Into<String>)` |
| `DELETE /rest/api/2/mypreferences`<br>`myself().remove_preference().key(impl Into<String>)` | `myself().remove_preference(key: impl Into<String>)` |
| `PUT /rest/api/2/mypreferences`<br>`myself().set_preference().body(impl Into<String>).key(impl Into<String>)` | `myself().set_preference(key: impl Into<String>, body: impl Into<String>)` |
| `POST /rest/api/2/permissionscheme/{}/permission`<br>`permission_schemes().create_permission_grant(scheme_id: i64).permission_grant(PermissionGrant)` | `permission_schemes().create_permission_grant(scheme_id: i64, permission_grant: PermissionGrant)` |
| `POST /rest/api/2/permissionscheme`<br>`permission_schemes().create_permission_scheme().body(PermissionScheme)` | `permission_schemes().create_permission_scheme(body: PermissionScheme)` |
| `DELETE /rest/api/2/permissionscheme/{}/permission/{}`<br>`permission_schemes().delete_permission_scheme_entity(permission_id: i64, scheme_id: i64)` | `permission_schemes().delete_permission_scheme_entity(scheme_id: i64, permission_id: i64)` |
| `GET /rest/api/2/permissionscheme/{}/permission/{}`<br>`permission_schemes().get_permission_scheme_grant(permission_id: i64, scheme_id: i64)` | `permission_schemes().get_permission_scheme_grant(scheme_id: i64, permission_id: i64)` |
| `PUT /rest/api/2/permissionscheme/{}`<br>`permission_schemes().update_permission_scheme(scheme_id: i64).body(PermissionScheme)` | `permission_schemes().update_permission_scheme(scheme_id: i64, body: PermissionScheme)` |
| `POST /rest/api/2/component`<br>`project_components().create_component().component(Component)` | `project_components().create_component(project_component: Component)` |
| `PUT /rest/api/2/component/{}`<br>`project_components().update_component(id: impl Into<String>).body(Component)` | `project_components().update_component(id: impl Into<String>, body: Component)` |
| `DELETE /rest/api/2/project/{}/properties/{}`<br>`project_properties().delete_project_property(property_key: impl Into<String>, project_id_or_key: impl Into<String>)` | `project_properties().delete_project_property(project_id_or_key: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/api/2/project/{}/properties/{}`<br>`project_properties().get_project_property(property_key: impl Into<String>, project_id_or_key: impl Into<String>)` | `project_properties().get_project_property(project_id_or_key: impl Into<String>, property_key: impl Into<String>)` |
| `PUT /rest/api/2/project/{}/properties/{}`<br>`project_properties().set_project_property(property_key: impl Into<String>, project_id_or_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` | `project_properties().set_project_property(project_id_or_key: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>)` |
| `POST /rest/api/2/role/{}/actors`<br>`project_role_actors().add_project_role_actors_to_role(id: i64).actor_input(ActorInput)` | `project_role_actors().add_project_role_actors_to_role(id: i64, actor_input: ActorInput)` |
| `PUT /rest/api/2/role/{}`<br>`project_roles().fully_update_project_role(id: i64).create_update_role_request(CreateUpdateRoleRequest)` | `project_roles().fully_update_project_role(id: i64, create_update_role_request: CreateUpdateRoleRequest)` |
| `POST /rest/api/2/role/{}`<br>`project_roles().partial_update_project_role(id: i64).create_update_role_request(CreateUpdateRoleRequest)` | `project_roles().partial_update_project_role(id: i64, create_update_role_request: CreateUpdateRoleRequest)` |
| `PUT /rest/api/2/version/{}/mergeto/{}`<br>`project_versions().merge_versions(move_issues_to: impl Into<String>, id: impl Into<String>)` | `project_versions().merge_versions(id: impl Into<String>, move_issues_to: impl Into<String>)` |
| `GET /rest/api/2/project/{}/notificationscheme`<br>`projects().get_notification_scheme_for_project(project_key_or_id: impl Into<String>).expand(impl Into<String>)` | `projects().get_notification_scheme_for_project(project_key_or_id: impl Into<String>).expand(impl Into<GetNotificationSchemeForProjectRequestExpand>)` |
| `DELETE /rest/api/2/project/{}/priorityscheme/{}`<br>`projects().unassign_priority_scheme(scheme_id: i64, project_key_or_id: impl Into<String>)` | `projects().unassign_priority_scheme(project_key_or_id: impl Into<String>, scheme_id: i64)` |
| `PUT /rest/api/2/project/{}`<br>`projects().update_project(project_id_or_key: impl Into<String>, project_update: ProjectUpdate)` | `projects().update_project(project_id_or_key: impl Into<String>, update_project_details: ProjectUpdate)` |
| `POST /rest/api/2/screens/{}/tabs/{}/fields`<br>`screen_tab_fields().add_screen_tab_field(tab_id: i64, screen_id: i64).add_field(AddField)` | `screen_tab_fields().add_screen_tab_field(screen_id: i64, tab_id: i64, add_field: AddField)` |
| `GET /rest/api/2/screens/{}/tabs/{}/fields`<br>`screen_tab_fields().get_all_screen_tab_fields(tab_id: i64, screen_id: i64)` | `screen_tab_fields().get_all_screen_tab_fields(screen_id: i64, tab_id: i64)` |
| `POST /rest/api/2/screens/{}/tabs/{}/fields/{}/move`<br>`screen_tab_fields().move_screen_tab_field(tab_id: i64, screen_id: i64, id: impl Into<String>).move_field(MoveField)` | `screen_tab_fields().move_screen_tab_field(screen_id: i64, tab_id: i64, id: impl Into<String>, move_field: MoveField)` |
| `DELETE /rest/api/2/screens/{}/tabs/{}/fields/{}`<br>`screen_tab_fields().remove_screen_tab_field(tab_id: i64, screen_id: i64, id: impl Into<String>)` | `screen_tab_fields().remove_screen_tab_field(screen_id: i64, tab_id: i64, id: impl Into<String>)` |
| `POST /rest/api/2/screens/{}/tabs`<br>`screen_tabs().add_screen_tab(screen_id: i64).screenable_tab(ScreenableTab)` | `screen_tabs().add_screen_tab(screen_id: i64, screenable_tab: ScreenableTab)` |
| `DELETE /rest/api/2/screens/{}/tabs/{}`<br>`screen_tabs().delete_screen_tab(tab_id: i64, screen_id: i64)` | `screen_tabs().delete_screen_tab(screen_id: i64, tab_id: i64)` |
| `POST /rest/api/2/screens/{}/tabs/{}/move/{}`<br>`screen_tabs().move_screen_tab(tab_id: i64, screen_id: i64, pos: i64)` | `screen_tabs().move_screen_tab(screen_id: i64, tab_id: i64, pos: i64)` |
| `PUT /rest/api/2/screens/{}/tabs/{}`<br>`screen_tabs().rename_screen_tab(tab_id: i64, screen_id: i64).screenable_tab(ScreenableTab)` | `screen_tabs().rename_screen_tab(screen_id: i64, tab_id: i64, screenable_tab: ScreenableTab)` |
| `PUT /rest/api/2/screens/{}/tabs/{}/fields/{}/updateShowWhenEmptyIndicator/{}`<br>`screens().update_show_when_empty_indicator(tab_id: i64, screen_id: i64, new_value: bool, id: impl Into<String>)` | `screens().update_show_when_empty_indicator(screen_id: i64, tab_id: i64, id: impl Into<String>, new_value: bool)` |
| `DELETE /rest/agile/1.0/sprint/{}/properties/{}`<br>`sprint().delete_property(property_key: impl Into<String>, sprint_id: i64)` | `sprint().delete_property(sprint_id: impl Into<String>, property_key: impl Into<String>)` |
| `GET /rest/agile/1.0/sprint/{}/issue`<br>`sprint().get_issues_for_sprint(sprint_id: i64).fields(GetIssuesForSprintRequestFields)` | `sprint().get_issues_for_sprint(sprint_id: i64).fields(impl IntoIterator<Item = impl Into<String>>)` |
| `GET /rest/agile/1.0/sprint/{}/properties`<br>`sprint().get_properties_keys(sprint_id: i64)` | `sprint().get_properties_keys(sprint_id: impl Into<String>)` |
| `GET /rest/agile/1.0/sprint/{}/properties/{}`<br>`sprint().get_property(property_key: impl Into<String>, sprint_id: i64)` | `sprint().get_property(sprint_id: impl Into<String>, property_key: impl Into<String>)` |
| `PUT /rest/agile/1.0/sprint/{}/properties/{}`<br>`sprint().set_property(property_key: impl Into<String>, sprint_id: i64, body: std::collections::HashMap<String, serde_json::Value>)` | `sprint().set_property(sprint_id: impl Into<String>, property_key: impl Into<String>, property_value: std::collections::HashMap<String, serde_json::Value>)` |
| `GET /rest/api/2/user/assignable/search`<br>`user_search().find_assignable_users()` | `user_search().find_assignable_users().start_at(i64)` |
| `GET /rest/api/2/user/assignable/multiProjectSearch`<br>`user_search().find_bulk_assignable_users().project_keys(FindBulkAssignableUsersRequestProjectKeys)` | `user_search().find_bulk_assignable_users(project_keys: FindBulkAssignableUsersRequestProjectKeys).start_at(i64)` |
| `GET /rest/api/2/user/picker`<br>`user_search().find_users_for_picker().query(impl Into<String>)` | `user_search().find_users_for_picker(query: impl Into<String>)` |
| `GET /rest/api/2/user/viewissue/search`<br>`user_search().find_users_with_browse_permission()` | `user_search().find_users_with_browse_permission().start_at(i64)` |
| `POST /rest/api/2/user`<br>`users().create_user(user_write: UserWrite)` | `users().create_user(new_user_details: UserWrite)` |
| `PUT /rest/api/2/user/columns`<br>`users().set_user_columns().columns(impl IntoIterator<Item = impl Into<String>>).username(impl Into<String>)` | `users().set_user_columns(user_column_request_body: SetUserColumnsRequestUserColumnRequestBody)` |
| `DELETE /rest/api/2/workflowscheme/{}/draft/issuetype/{}`<br>`workflow_scheme_drafts().delete_workflow_scheme_draft_issue_type(issue_type: impl Into<String>, id: i64)` | `workflow_scheme_drafts().delete_workflow_scheme_draft_issue_type(id: i64, issue_type: impl Into<String>)` |
| `GET /rest/api/2/workflowscheme/{}/draft/issuetype/{}`<br>`workflow_scheme_drafts().get_workflow_scheme_draft_issue_type(issue_type: impl Into<String>, id: i64)` | `workflow_scheme_drafts().get_workflow_scheme_draft_issue_type(id: i64, issue_type: impl Into<String>)` |
| `PUT /rest/api/2/workflowscheme/{}/draft/issuetype/{}`<br>`workflow_scheme_drafts().set_workflow_scheme_draft_issue_type(issue_type: impl Into<String>, id: i64, body: IssueTypeMapping)` | `workflow_scheme_drafts().set_workflow_scheme_draft_issue_type(id: i64, issue_type: impl Into<String>, body: IssueTypeMapping)` |
| `PUT /rest/api/2/workflowscheme/{}/draft/default`<br>`workflow_scheme_drafts().update_draft_default_workflow(id: i64, default: DefaultModel)` | `workflow_scheme_drafts().update_draft_default_workflow(id: i64, default_workflow: DefaultModel)` |
| `DELETE /rest/api/2/workflowscheme/{}/issuetype/{}`<br>`workflow_schemes().delete_workflow_scheme_issue_type(issue_type: impl Into<String>, id: i64)` | `workflow_schemes().delete_workflow_scheme_issue_type(id: i64, issue_type: impl Into<String>)` |
| `GET /rest/api/2/workflowscheme/{}/issuetype/{}`<br>`workflow_schemes().get_workflow_scheme_issue_type(issue_type: impl Into<String>, id: i64)` | `workflow_schemes().get_workflow_scheme_issue_type(id: i64, issue_type: impl Into<String>)` |
| `PUT /rest/api/2/workflowscheme/{}/issuetype/{}`<br>`workflow_schemes().set_workflow_scheme_issue_type(issue_type: impl Into<String>, id: i64, body: IssueTypeMapping)` | `workflow_schemes().set_workflow_scheme_issue_type(id: i64, issue_type: impl Into<String>, body: IssueTypeMapping)` |
| `PUT /rest/api/2/workflowscheme/{}/default`<br>`workflow_schemes().update_default_workflow(id: i64, default: DefaultModel)` | `workflow_schemes().update_default_workflow(id: i64, default_workflow: DefaultModel)` |

</details>

<details>
<summary>Model types</summary>

| 0.1.1 | 0.2.0 |
|---|---|
| `jira::server::BoardConfig` | `jira::server::GetConfiguration` |
| `jira::server::EntityPropertiesKeys` | `jira::server::PropertyKeys` |
| `jira::server::Dashboards` | `jira::server::PageOfDashboards` |
| `jira::server::RustPageNotificationScheme` | `jira::server::PageOfNotificationSchemes` |
| `jira::server::RustPageVersion` | `jira::server::PageOfVersions` |
| `jira::server::FilterPermission` | `jira::server::SharePermission` |
| `jira::server::UsersAndGroups` | `jira::server::FoundUsersAndGroups` |
| `jira::server::GroupSuggestions` | `jira::server::FoundGroups` |
| `jira::server::AttachmentMeta` | `jira::server::AttachmentSettings` |
| `jira::server::CommentsWithPaginationJson` | `jira::server::PageOfComments` |
| `jira::server::CommentJson` | `jira::server::Comment` |
| `jira::server::Field` | `jira::server::FieldDetails` |
| `jira::server::IssueLinkTypeJson` | `jira::server::IssueLinkType` |
| `jira::server::PriorityJson` | `jira::server::Priority` |
| `jira::server::IssuePickerResult` | `jira::server::IssuePickerSuggestions` |
| `jira::server::SecurityLevelJson` | `jira::server::SecurityLevel` |
| `jira::server::SecuritySchemesJson` | `jira::server::SecuritySchemes` |
| `jira::server::SecuritySchemeJson` | `jira::server::SecurityScheme` |
| `jira::server::IssueTypeJson` | `jira::server::IssueTypeDetails` |
| `jira::server::Vote` | `jira::server::Votes` |
| `jira::server::WorklogWithPagination` | `jira::server::PageOfWorklogs` |
| `jira::server::WorklogChangedSince` | `jira::server::ChangedWorklogs` |
| `jira::server::IssueCreateResponse` | `jira::server::CreatedIssue` |
| `jira::server::IssuesCreateResponse` | `jira::server::CreatedIssues` |
| `jira::server::EditMeta` | `jira::server::IssueUpdateMetadata` |
| `jira::server::TransitionsMeta` | `jira::server::Transitions` |
| `jira::server::AutoCompleteResponse` | `jira::server::JQLReferenceData` |
| `jira::server::AutoCompleteResultWrapper` | `jira::server::AutoCompleteSuggestions` |
| `jira::server::PermissionsJson` | `jira::server::Permissions` |
| `jira::server::Component` | `jira::server::ProjectComponent` |
| `jira::server::ComponentIssueCounts` | `jira::server::ComponentIssuesCount` |
| `jira::server::SecurityListLevelJson` | `jira::server::ProjectIssueSecurityLevels` |
| `jira::server::VersionUnresolvedIssueCounts` | `jira::server::VersionUnresolvedIssuesCount` |
| `jira::server::ProjectIdentity` | `jira::server::ProjectIdentifiers` |
| `jira::server::ServerInfo` | `jira::server::ServerInformation` |
| `jira::server::UserPickerResults` | `jira::server::FoundUsers` |
| `jira::server::IssueTypeMapping` | `jira::server::IssueTypeWorkflowMapping` |
| `jira::server::StatusCategoryJson` | `jira::server::StatusCategory` |
| `jira::server::StatusJson` | `jira::server::StatusDetails` |
| `jira::server::IssueTypeWithStatusJson` | `jira::server::IssueTypeWithStatus` |
| `jira::server::IssueUpdate` | `jira::server::IssueUpdateDetails` |
| `jira::server::NotificationJson` | `jira::server::Notification` |
| `jira::server::RemoteIssueLinkCreateOrUpdateRequest` | `jira::server::RemoteIssueLinkRequest` |
| `jira::server::LinkIssueRequestJson` | `jira::server::LinkIssueRequest` |
| `jira::server::ApplicationPropertyValue` | `jira::server::SimpleApplicationProperty` |
| `jira::server::ProjectUpdate` | `jira::server::UpdateProjectDetails` |
| `jira::server::UserJson` | `jira::server::UserDetails` |
| `jira::server::VisibilityJson` | `jira::server::Visibility` |
| `jira::server::EntityPropertyKey` | `jira::server::PropertyKey` |
| `jira::server::GroupJson` | `jira::server::GroupName` |
| `jira::server::RankingConfig` | `jira::server::GetConfigurationRanking` |
| `jira::server::Subquery` | `jira::server::GetConfigurationSubQuery` |

</details>

<details>
<summary>Request-builder types</summary>

| 0.1.1 | 0.2.0 |
|---|---|
| `cloud::CreateCustomFieldRequest2` | `cloud::IssueFieldsCreateCustomFieldRequest` |
| `cloud::CreateProjectRequest2` | `cloud::ProjectsCreateProjectRequest` |
| `cloud::SetDefaultPriorityRequest2` | `cloud::IssuePrioritiesSetDefaultPriorityRequest` |
| `server::AddFieldRequest` | `server::AddScreenTabFieldRequest` |
| `server::AddTabRequest` | `server::AddScreenTabRequest` |
| `server::AssignRequest` | `server::AssignIssueRequest` |
| `server::CreateDraftForParentRequest` | `server::CreateWorkflowSchemeDraftFromParentRequest` |
| `server::CreateSchemeRequest` | `server::CreateWorkflowSchemeRequest` |
| `server::DefaultColumnsRequest` | `server::GetUserDefaultColumnsRequest` |
| `server::DeleteDefaultRequest` | `server::DeleteDefaultWorkflowRequest` |
| `server::DeleteDraftByIdRequest` | `server::DeleteWorkflowSchemeDraftRequest` |
| `server::DeleteDraftDefaultRequest` | `server::DeleteDraftDefaultWorkflowRequest` |
| `server::DeleteDraftIssueTypeRequest` | `server::DeleteWorkflowSchemeDraftIssueTypeRequest` |
| `server::DeleteSchemeRequest` | `server::DeleteWorkflowSchemeRequest` |
| `server::DeleteSprintPropertyRequest` | `server::DeletePropertyRequest` |
| `server::DeleteTabRequest` | `server::DeleteScreenTabRequest` |
| `server::DeleteVersionAndSwapRequest` | `server::DeleteAndReplaceVersionRequest` |
| `server::EditFilterRequest` | `server::UpdateFilterRequest` |
| `server::EditFilterRequestExpand` | `server::UpdateFilterRequestExpand` |
| `server::GetAgileIssueRequest` | `server::IssueGetIssueRequest` |
| `server::GetAgileIssueRequestExpand` | `server::IssueGetIssueRequestExpand` |
| `server::GetAgileIssueRequestFields` | `server::IssueGetIssueRequestFields` |
| `server::GetAllFieldsRequest` | `server::GetAllScreenTabFieldsRequest` |
| `server::GetAllProjectsRequest` | `server::SearchProjectsRequest` |
| `server::GetAllRequest` | `server::GetAllApplicationRolesRequest` |
| `server::GetAllScreensRequest` | `server::GetScreensRequest` |
| `server::GetAllTabsRequest` | `server::GetAllScreenTabsRequest` |
| `server::GetApplicationPropertiesRequest` | `server::GetApplicationPropertyRequest` |
| `server::GetBoardConfigurationRequest` | `server::BoardGetConfigurationRequest` |
| `server::GetByIdRequest` | `server::GetWorkflowSchemeRequest` |
| `server::GetConfigurationRequest` | `server::JiraSettingsGetConfigurationRequest` |
| `server::GetCreateIssueMetaFieldsRequest` | `server::GetCreateIssueMetaIssueTypeIdRequest` |
| `server::GetCreateIssueMetaProjectIssueTypesRequest` | `server::GetCreateIssueMetaIssueTypesRequest` |
| `server::GetDefaultRequest` | `server::GetDefaultWorkflowRequest` |
| `server::GetDraftByIdRequest` | `server::GetWorkflowSchemeDraftRequest` |
| `server::GetDraftDefaultRequest` | `server::GetDraftDefaultWorkflowRequest` |
| `server::GetDraftIssueTypeRequest` | `server::GetWorkflowSchemeDraftIssueTypeRequest` |
| `server::GetFieldsToAddRequest` | `server::GetAvailableScreenFieldsRequest` |
| `server::GetFilterColumnsRequest` | `server::GetColumnsRequest` |
| `server::GetIssueRequest` | `server::IssuesGetIssueRequest` |
| `server::GetIssuesForBoardEpicRequest` | `server::GetBoardIssuesForEpicRequest` |
| `server::GetIssuesForBoardEpicRequestExpand` | `server::GetBoardIssuesForEpicRequestExpand` |
| `server::GetIssuesForBoardEpicRequestFields` | `server::GetBoardIssuesForEpicRequestFields` |
| `server::GetIssuesForBoardSprintRequest` | `server::GetBoardIssuesForSprintRequest` |
| `server::GetIssuesForBoardSprintRequestExpand` | `server::GetBoardIssuesForSprintRequestExpand` |
| `server::GetIssuesForBoardSprintRequestFields` | `server::GetBoardIssuesForSprintRequestFields` |
| `server::GetPermissionSchemesRequest` | `server::GetAllPermissionSchemesRequest` |
| `server::GetPermissionSchemesRequestExpand` | `server::GetAllPermissionSchemesRequestExpand` |
| `server::GetPermissionsRequest` | `server::GetMyPermissionsRequest` |
| `server::GetProjectNotificationSchemeRequest` | `server::GetNotificationSchemeForProjectRequest` |
| `server::GetProjectRolesByIdRequest` | `server::GetProjectRoleByIdRequest` |
| `server::GetSprintPropertyKeysRequest` | `server::GetPropertiesKeysRequest` |
| `server::GetSprintPropertyRequest` | `server::GetPropertyRequest` |
| `server::ListRequest` | `server::GetAllDashboardsRequest` |
| `server::MergeRequest` | `server::MergeVersionsRequest` |
| `server::MoveFieldRequest` | `server::MoveScreenTabFieldRequest` |
| `server::MoveIssuesToSprintRequest` | `server::MoveIssuesToSprintAndRankRequest` |
| `server::MoveTabRequest` | `server::MoveScreenTabRequest` |
| `server::ReindexRequest2` | `server::IndexingReindexRequest` |
| `server::RemoveFieldRequest` | `server::RemoveScreenTabFieldRequest` |
| `server::RenameTabRequest` | `server::RenameScreenTabRequest` |
| `server::SearchForProjectsRequest` | `server::GetProjectsPickerRequest` |
| `server::SearchRequest2` | `server::SearchIssuesRequest` |
| `server::SearchRequest2Expand` | `server::SearchIssuesRequestExpand` |
| `server::SearchRequest2Fields` | `server::SearchIssuesRequestFields` |
| `server::SearchUsingSearchRequestRequest` | `server::SearchIssuesPostRequest` |
| `server::SetColumnsUrlEncodedRequest` | `server::SetUserColumnsRequest` |
| `server::SetDraftIssueTypeRequest` | `server::SetWorkflowSchemeDraftIssueTypeRequest` |
| `server::SetIssueNavigatorDefaultColumnsFormRequest` | `server::SetIssueNavigatorDefaultColumnsRequest` |
| `server::SetIssueTypeRequest` | `server::SetWorkflowSchemeIssueTypeRequest` |
| `server::SetPropertyViaRestfulTableRequest` | `server::SetApplicationPropertyRequest` |
| `server::SetSprintPropertyRequest` | `server::SetPropertyRequest` |
| `server::UpdateDefaultRequest` | `server::UpdateDefaultWorkflowRequest` |
| `server::UpdateDraftDefaultRequest` | `server::UpdateDraftDefaultWorkflowRequest` |
| `server::UpdateDraftRequest` | `server::UpdateWorkflowSchemeDraftRequest` |

</details>

### Fixed

- Data Center user searches return `Vec<User>` instead of one `User`, so successful list responses pass the schema check.
- Paging parameters declared as strings are now numbers on three Cloud requests, Data Center and Assets Data Center.
- The `ChangedWorklogs` stream stops when the response sets `lastPage`.

## [0.1.1] - 2026-10-01

### Changed

- An operation Jira may answer with no body now hands back an `Option` of its model, and the empty answer is `None`.
  Before, the empty answer failed the schema check although the operation had succeeded. This changes the return
  type of eight operations, so code that names the old type no longer compiles; the old type could not describe the
  answer Jira gives on success, so it ships as a fix rather than a minor release:
  - Agile: `move_issues_to_board`
  - Cloud: `update_precomputations`, `get_selected_time_tracking_implementation`, `publish_draft_workflow_scheme`,
    `update_schemes`
  - Data Center: `rank_issues`, `set_board_property`, `get_error`
- Cloud: `add_issue_types_to_context` takes `AddIssueTypesToContext`, which adds `isAnyIssueType` and a default value
  beside the issue type ids, following the specification Atlassian published.

### Fixed

- Service Management Data Center: a customer organization id is the string the instance sends, not a number.
- `audit`: a value met while an untagged enum tried a variant it then dropped is no longer reported. The
  `CompoundClauseOperator` value `=` was the operator of a field clause, read while the compound clause was tried first.
- The readme examples are whole snippets, without the lines only rustdoc hides.

## [0.1.0] - 2026-09-30

### Added

- The transport: a client carrying auth, retry policy and error handling, shared by every API surface.
- `webhooks`: the events Jira sends, the payloads it posts, and `verify_signature`, which is what tells a
  delivery from Jira apart from a POST anyone can make to the same URL.
- Authentication by API token, by personal access token, by a token minted per request, by Jira Cloud OAuth 2.0 (3LO)
  and by a Data Center instance's own OAuth 2.0 provider. Refreshes are single-flighted and a 401 buys one retry.
- Eleven API surfaces behind a cargo feature each, generated from the same OpenAPI pipeline as `jira.js`: Cloud,
  Agile, Service Management, Assets, Data Center, Service Management Data Center, Assets Data Center, administration,
  Teams, user management and user provisioning.
- `jira::Error`, one enum with predicates that read the status and the OAuth code — including the `X-Seraph-LoginReason`
  header, which is how Jira reports a refused credential behind a `200`.
- `with_retry`, applying the client's own transient-failure policy around a call.
- `jira::jql`, which builds a query out of values that came from somewhere else. A value reaches Jira quoted and
  escaped rather than able to end the literal it sits in and add a clause of its own.
- `stream` on the JQL search, which follows the endpoint's page token to the last page so a caller writes the query
  rather than the loop.
- `Extensible`, on every type that carries fields the schema does not describe: `custom` reads them into a type of
  the caller's own, `with_custom` and `with` write them beside the described fields, and a key the schema already
  describes is refused rather than sent twice.
- Wiki markup written as a string into a description, an environment, a comment or a worklog reaches the `v2` twin
  of the endpoint, which is the one that reads it — at creation and at edit alike — and comes back as the document
  Jira parsed it into.
- `get_tenant_context`, resolving a site's cloud id, organization id and host name.
- The `audit` feature, which collects the fields the API sends that the generated types do not describe.
- The `chrono` feature, which turns every `date-time` into `Option<chrono::DateTime<Utc>>`. Off by default,
  because the type of a field is not something two crates depending on this one get to disagree about. The
  reader takes the spelling Atlassian documents — which is not RFC 3339 — as well as RFC 3339, an instant
  without an offset, a bare date, and the epoch milliseconds the bulk queue sends; a value it does not
  recognise becomes `None` rather than failing the response around it.
  An instant is written back as the API it goes to reads it: `+0000` for Jira, RFC 3339 for Assets, which rejects
  the other spelling.
- `#[deprecated]` on the operations, parameters and fields Atlassian deprecates in prose rather than with the
  OpenAPI flag, which is how it deprecates nearly all of them. Where the prose points at a URL rather than a method,
  the note names the method of this crate that answers there, and the documentation links to it.
- `#[non_exhaustive]` on the types a caller only ever reads, so a field Atlassian adds is a minor release. The
  types a caller builds stay open, so a request body can still be assembled with `..Default::default()`.
- `PartialEq` on every generated type.
- `reqwest` is re-exported, so a caller passing their own client through `ClientBuilder::http_client` can name
  the version this crate was built against.
- The JQL search is `search_issues` rather than `search_and_reconsile_issues_using_jql`: the specification misspells
  the operation and spells the model it answers with correctly, in the same document, and this is the most called
  operation in the API. The generator renames it, so `jira.js` carries the same name with the old one kept as a
  deprecated alias; nothing was published under the old name here.
- Wiki markup where v3 wants a document: a rich-text field written as a plain string goes to the v2 twin of the
  endpoint, which converts it, and the result is read back through v3.
- `IssueFields`: the fields of an issue are a struct rather than a map of raw JSON. The system fields Jira
  documents in prose — summary, description, project, issue type, status, assignee, the timestamps, links, subtasks,
  attachments, comments and the rest — are typed, and a custom field lands in `additional`, keyed the way the site
  names it. The same struct is what `create_issue` and `edit_issue` take.
- `stream` on every request that pages by offset — projects, boards, users, filters, dashboards and the rest across
  the five surfaces that page this way — so the JQL search is no longer the only listing a caller does not write the
  loop for.
- `error_messages` and `field_errors` on `Error`, reading the two shapes Jira puts a refusal in: the list of
  messages, and the map from a field to what was wrong with it.
- The `tracing` feature: a `jira.request` span around every request, with an event per attempt. Off by default, and
  nothing in it is a credential or a body.
- `SECURITY.md`, dependency updates through Dependabot, and `cargo deny` in CI, so an advisory against a dependency
  fails the build rather than waits to be noticed.
- Every public item of the transport carries documentation, and the crate warns on one that does not.
- The administration surface names a role, an account status, a membership status, a claim status and a
  combined status once each. The specification restates the same lists per operation, which used to become
  twenty-six types for six concepts, so a role read from one call could not be passed to another.
- `assign_issue` takes the account as an `Option`: `Some(account_id)` assigns, `None` sends `accountId: null`, which
  is how Jira takes the assignee away.
- A value interpolated into a request path is escaped, so a `/`, a `?` or a `..` in a property key cannot split the
  segment, start a query string or address a different endpoint.
- A request body the specification types as binary is sent as bytes rather than as a JSON array of them, and carries
  a `content_type` the caller declares — Jira reads the declared type rather than sniffing the bytes.
- A `date-time` field reads a number as well as a string. Jira declares every timestamp a string and the bulk queue
  answers epoch milliseconds.
- A generated union ends in a catch-all, so a shape the specification does not list does not fail the response.

### Credits

- Harold Dost published `0.0.1` of this crate name in 2021. That release shares no code with this one, but it kept
  the name alive.

[0.2.0]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.2.0
[0.1.1]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.1.0
