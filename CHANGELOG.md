# Changelog

All notable changes to this crate are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the crate follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) — before `1.0.0`, a breaking change raises the minor
version.

## [Unreleased]

## [0.3.0] - 2026-10-14

### Added

- Cloud label listings accept the optional `query` and `scope` filters.
- Cloud models include the `longtext` custom-field default and the policy and bulk agent session schemas from the OpenAPI snapshot fetched on 2026-10-14. The nine new experimental operations remain excluded by the existing generator policy.
- Administration, Agile and Assets models expose the additional fields described by the current generator snapshot.

### Changed

- **Breaking:** Cloud `issues().delete_issue(..).delete_subtasks(..)` takes `bool` instead of `impl Into<DeleteIssueRequestDeleteSubtasks>`; the enum is removed. Pass `true` or `false` instead of a string or enum variant.
- **Breaking:** Data Center `issues().delete_issue(..).delete_subtasks(..)` takes `bool` instead of `impl Into<DeleteIssueRequestDeleteSubtasks>`; the enum is removed. Pass `true` or `false` instead of a string or enum variant.
- **Breaking:** Administration `WorkspaceModel.relationships` takes `WorkspaceModelRelationships` instead of a map of lists; `WorkspaceModelRelationships` is now a struct with typed relationship fields instead of an enum.
- **Breaking:** Administration `AllowIfContainedRule` has five additional optional fields. Update complete struct literals or use `..Default::default()`.
- **Breaking:** Cloud `CustomFieldContextDefaultValue` has a `CustomFieldContextDefaultValueLongText` variant. Update exhaustive matches.

### Removed

- **Breaking:** Service Management `request().create_customer_request_skipping_login_check(..)` and `CreateCustomerRequestSkippingLoginCheckRequest` are removed. The operation is absent from the current vendor document and returned HTTP 405 on the Cloud test site on 2026-10-14; the ordinary `create_customer_request(..)` operation remains available.

### Fixed

- Live administration checks report and skip throttled answers, which provide no evidence about access rights.
- The Agile board projects stream documentation records that Jira Cloud ignored `startAt` and `maxResults` and echoed `startAt: 0, maxResults: 50` when measured on a board with one project.

## [0.2.0] - 2026-10-11

### Added

- `stream()` is available on requests that page across every surface, whether the server uses an offset and total, a page token, a query or body cursor, a cursor under `links`, a page number, or a one-based index. Requests that only contain an embedded page, fetch a fixed list or one resource, or expose a polling feed without a final page are exceptions described in [`xtask/streams.txt`](xtask/streams.txt).
- [The Cloud and Data Center parity matrix](docs/parity.md) lists paired operations and the remaining differences.

### Changed

- Cloud `Field.schema` is now `Option<JsonType>` instead of `JsonType`.
- Data Center follows Cloud for shared operations: service accessor, method name, positional parameters, and model type names match where the APIs permit it. Update existing Data Center calls and type references using the tables below.

**Compiles and does something else.** In each row, the bold positional arguments share a Rust type and changed relative order. Some old calls still compile unchanged; others compile again after the accessor or method is renamed using the tables below while their arguments remain in the old order:

| Operation | 0.1.1 | 0.2.0 |
|---|---|---|
| `DELETE /rest/api/2/comment/{}/properties/{}` | `issue_comments().delete_comment_property`(**`property_key`**, **`comment_id`**) | `issue_comment_properties().delete_comment_property`(**`comment_id`**, **`property_key`**) |
| `DELETE /rest/api/2/dashboard/{}/items/{}/properties/{}` | `dashboards().delete_dashboard_item_property`(**`property_key`**, **`item_id`**, **`dashboard_id`**) | `dashboards().delete_dashboard_item_property`(**`dashboard_id`**, **`item_id`**, **`property_key`**) |
| `DELETE /rest/api/2/issue/{}/properties/{}` | `issues().delete_issue_property`(**`property_key`**, **`issue_id_or_key`**) | `issue_properties().delete_issue_property`(**`issue_id_or_key`**, **`property_key`**) |
| `DELETE /rest/api/2/issue/{}/remotelink/{}` | `issues().delete_remote_issue_link_by_id`(**`link_id`**, **`issue_id_or_key`**) | `issue_remote_links().delete_remote_issue_link_by_id`(**`issue_id_or_key`**, **`link_id`**) |
| `DELETE /rest/api/2/issuetype/{}/properties/{}` | `issue_types().delete_issue_type_property`(**`property_key`**, **`issue_type_id`**) | `issue_type_properties().delete_issue_type_property`(**`issue_type_id`**, **`property_key`**) |
| `DELETE /rest/api/2/issuetypescheme/{}/associations/{}` | `issue_type_schemes().remove_project_association`(**`proj_id_or_key`**, **`scheme_id`**) | `issue_type_schemes().remove_project_association`(**`scheme_id`**, **`proj_id_or_key`**) |
| `DELETE /rest/api/2/permissionscheme/{}/permission/{}` | `permission_schemes().delete_permission_scheme_entity`(**`permission_id`**, **`scheme_id`**) | `permission_schemes().delete_permission_scheme_entity`(**`scheme_id`**, **`permission_id`**) |
| `DELETE /rest/api/2/project/{}/properties/{}` | `projects().delete_project_property`(**`property_key`**, **`project_id_or_key`**) | `project_properties().delete_project_property`(**`project_id_or_key`**, **`property_key`**) |
| `DELETE /rest/api/2/screens/{}/tabs/{}/fields/{}` | `screens().remove_field`(**`tab_id`**, **`screen_id`**, `id`) | `screen_tab_fields().remove_screen_tab_field`(**`screen_id`**, **`tab_id`**, `id`) |
| `DELETE /rest/api/2/screens/{}/tabs/{}` | `screens().delete_tab`(**`tab_id`**, **`screen_id`**) | `screen_tabs().delete_screen_tab`(**`screen_id`**, **`tab_id`**) |
| `GET /rest/agile/1.0/board/{}/epic/{}/issue` | `board().get_issues_for_board_epic`(**`epic_id`**, **`board_id`**) | `board().get_board_issues_for_epic`(**`board_id`**, **`epic_id`**) |
| `GET /rest/agile/1.0/board/{}/sprint/{}/issue` | `board().get_issues_for_board_sprint`(**`sprint_id`**, **`board_id`**) | `board().get_board_issues_for_sprint`(**`board_id`**, **`sprint_id`**) |
| `GET /rest/api/2/comment/{}/properties/{}` | `issue_comments().get_comment_property`(**`property_key`**, **`comment_id`**) | `issue_comment_properties().get_comment_property`(**`comment_id`**, **`property_key`**) |
| `GET /rest/api/2/dashboard/{}/items/{}/properties/{}` | `dashboards().get_dashboard_item_property`(**`property_key`**, **`item_id`**, **`dashboard_id`**) | `dashboards().get_dashboard_item_property`(**`dashboard_id`**, **`item_id`**, **`property_key`**) |
| `GET /rest/api/2/dashboard/{}/items/{}/properties` | `dashboards().get_dashboard_item_property_keys`(**`item_id`**, **`dashboard_id`**) | `dashboards().get_dashboard_item_property_keys`(**`dashboard_id`**, **`item_id`**) |
| `GET /rest/api/2/issue/createmeta/{}/issuetypes/{}` | `issues().get_create_issue_meta_fields`(**`issue_type_id`**, **`project_id_or_key`**) | `issues().get_create_issue_meta_issue_type_id`(**`project_id_or_key`**, **`issue_type_id`**) |
| `GET /rest/api/2/issue/{}/properties/{}` | `issues().get_issue_property`(**`property_key`**, **`issue_id_or_key`**) | `issue_properties().get_issue_property`(**`issue_id_or_key`**, **`property_key`**) |
| `GET /rest/api/2/issue/{}/remotelink/{}` | `issues().get_remote_issue_link_by_id`(**`link_id`**, **`issue_id_or_key`**) | `issue_remote_links().get_remote_issue_link_by_id`(**`issue_id_or_key`**, **`link_id`**) |
| `GET /rest/api/2/issuetype/{}/properties/{}` | `issue_types().get_issue_type_property`(**`property_key`**, **`issue_type_id`**) | `issue_type_properties().get_issue_type_property`(**`issue_type_id`**, **`property_key`**) |
| `GET /rest/api/2/permissionscheme/{}/permission/{}` | `permission_schemes().get_permission_scheme_grant`(**`permission_id`**, **`scheme_id`**) | `permission_schemes().get_permission_scheme_grant`(**`scheme_id`**, **`permission_id`**) |
| `GET /rest/api/2/project/{}/properties/{}` | `projects().get_project_property`(**`property_key`**, **`project_id_or_key`**) | `project_properties().get_project_property`(**`project_id_or_key`**, **`property_key`**) |
| `GET /rest/api/2/screens/{}/tabs/{}/fields` | `screens().get_all_fields`(**`tab_id`**, **`screen_id`**) | `screen_tab_fields().get_all_screen_tab_fields`(**`screen_id`**, **`tab_id`**) |
| `POST /rest/api/2/screens/{}/tabs/{}/fields/{}/move` | `screens().move_field`(**`tab_id`**, **`screen_id`**, `id`) | `screen_tab_fields().move_screen_tab_field`(**`screen_id`**, **`tab_id`**, `id`, `move_field`) |
| `POST /rest/api/2/screens/{}/tabs/{}/fields` | `screens().add_field`(**`tab_id`**, **`screen_id`**) | `screen_tab_fields().add_screen_tab_field`(**`screen_id`**, **`tab_id`**, `add_field`) |
| `POST /rest/api/2/screens/{}/tabs/{}/move/{}` | `screens().move_tab`(**`tab_id`**, **`screen_id`**, `pos`) | `screen_tabs().move_screen_tab`(**`screen_id`**, **`tab_id`**, `pos`) |
| `PUT /rest/api/2/comment/{}/properties/{}` | `issue_comments().set_comment_property`(**`property_key`**, **`comment_id`**, `body`) | `issue_comment_properties().set_comment_property`(**`comment_id`**, **`property_key`**, `body`) |
| `PUT /rest/api/2/dashboard/{}/items/{}/properties/{}` | `dashboards().set_dashboard_item_property`(**`property_key`**, **`item_id`**, **`dashboard_id`**, `body`) | `dashboards().set_dashboard_item_property`(**`dashboard_id`**, **`item_id`**, **`property_key`**, `body`) |
| `PUT /rest/api/2/issue/{}/properties/{}` | `issues().set_issue_property`(**`property_key`**, **`issue_id_or_key`**, `body`) | `issue_properties().set_issue_property`(**`issue_id_or_key`**, **`property_key`**, `body`) |
| `PUT /rest/api/2/issue/{}/remotelink/{}` | `issues().update_remote_issue_link`(**`link_id`**, **`issue_id_or_key`**) | `issue_remote_links().update_remote_issue_link`(**`issue_id_or_key`**, **`link_id`**, `remote_issue_link_request`) |
| `PUT /rest/api/2/issuetype/{}/properties/{}` | `issue_types().set_issue_type_property`(**`property_key`**, **`issue_type_id`**, `body`) | `issue_type_properties().set_issue_type_property`(**`issue_type_id`**, **`property_key`**, `body`) |
| `PUT /rest/api/2/project/{}/properties/{}` | `projects().set_project_property`(**`property_key`**, **`project_id_or_key`**, `body`) | `project_properties().set_project_property`(**`project_id_or_key`**, **`property_key`**, `body`) |
| `PUT /rest/api/2/screens/{}/tabs/{}/fields/{}/updateShowWhenEmptyIndicator/{}` | `screens().update_show_when_empty_indicator`(**`tab_id`**, **`screen_id`**, `new_value`, `id`) | `screens().update_show_when_empty_indicator`(**`screen_id`**, **`tab_id`**, `id`, `new_value`) |
| `PUT /rest/api/2/screens/{}/tabs/{}` | `screens().rename_tab`(**`tab_id`**, **`screen_id`**) | `screen_tabs().rename_screen_tab`(**`screen_id`**, **`tab_id`**, `screenable_tab`) |
| `PUT /rest/api/2/version/{}/mergeto/{}` | `project_versions().merge`(**`move_issues_to`**, **`id`**) | `project_versions().merge_versions`(**`id`**, **`move_issues_to`**) |

Either path can address the wrong resource. A `404` or an "issue does not exist" response can be the first sign; check the argument order before treating it as a missing resource.

- `expand` and other comma-separated parameters now take lists on every surface, including Cloud. Several values are sent as one comma-separated query value; 0.1.1 sent repeated keys for several values. The `One`/`Many` wrapper enums are gone.
- Jira Software Cloud issue lists take `fields` as field names, for example `.fields(["summary", "status"])`.
- Data Center request bodies are positional arguments. Nine Jira Software requests unfold their bodies into required arguments and setters. Use `CommentInput`, `WorklogInput`, and `ColumnRequestBody` for writes where the old response model or request wrapper was accepted.
- Service Management `request().get_attachments_for_request(issue_id_or_key)` takes `start` and `limit` as setters.
- Request-builder names no longer use numeric counters. When two builders shared a name, both now include their service name; the table lists the exact names.
- Data Center `Worklog`, `SprintCreate`, `Sprint`, `SprintSwap`, and `FieldEdit`, and `GetAllSprintsRequestState` are `#[non_exhaustive]`. These structs can no longer be built with a literal, and the enum cannot be matched exhaustively outside the crate; use input models for writes.
- Data Center `projects().search_projects()` calls `GET /rest/api/2/project` and returns the visible projects in one response. The picker is `projects().get_projects_picker()`.

<details>
<summary>Operations and accessors</summary>

| Surface | 0.1.1 | 0.2.0 |
|---|---|---|
| server | `application_properties().get_advanced_settings` | `jira_settings().get_advanced_settings` |
| server | `application_properties().get_application_properties` | `jira_settings().get_application_property` |
| server | `application_properties().set_property_via_restful_table` | `jira_settings().set_application_property` |
| server | `application_roles().get_all` | `application_roles().get_all_application_roles` |
| server | `board().get_board_configuration` | `board().get_configuration` |
| server | `board().get_issues_for_board_epic` | `board().get_board_issues_for_epic` |
| server | `board().get_issues_for_board_sprint` | `board().get_board_issues_for_sprint` |
| server | `configuration().get_configuration` | `jira_settings().get_configuration` |
| server | `dashboards().list` | `dashboards().get_all_dashboards` |
| server | `filters().add_share_permission` | `filter_sharing().add_share_permission` |
| server | `filters().delete_share_permission` | `filter_sharing().delete_share_permission` |
| server | `filters().edit_filter` | `filters().update_filter` |
| server | `filters().get_default_share_scope` | `filter_sharing().get_default_share_scope` |
| server | `filters().get_filter_columns` | `filters().get_columns` |
| server | `filters().get_share_permission` | `filter_sharing().get_share_permission` |
| server | `filters().get_share_permissions` | `filter_sharing().get_share_permissions` |
| server | `filters().set_default_share_scope` | `filter_sharing().set_default_share_scope` |
| server | `issue_comments().delete_comment_property` | `issue_comment_properties().delete_comment_property` |
| server | `issue_comments().get_comment_property` | `issue_comment_properties().get_comment_property` |
| server | `issue_comments().get_comment_property_keys` | `issue_comment_properties().get_comment_property_keys` |
| server | `issue_comments().set_comment_property` | `issue_comment_properties().set_comment_property` |
| server | `issue_search().search` | `issue_search().search_issues` |
| server | `issue_search().search_using_search_request` | `issue_search().search_issues_post` |
| server | `issue_types().delete_issue_type_property` | `issue_type_properties().delete_issue_type_property` |
| server | `issue_types().get_issue_type_property` | `issue_type_properties().get_issue_type_property` |
| server | `issue_types().get_issue_type_property_keys` | `issue_type_properties().get_issue_type_property_keys` |
| server | `issue_types().set_issue_type_property` | `issue_type_properties().set_issue_type_property` |
| server | `issues().add_attachment` | `issue_attachments().add_attachment` |
| server | `issues().add_comment` | `issue_comments().add_comment` |
| server | `issues().add_vote` | `issue_votes().add_vote` |
| server | `issues().add_watcher` | `issue_watchers().add_watcher` |
| server | `issues().add_worklog` | `issue_worklogs().add_worklog` |
| server | `issues().assign` | `issues().assign_issue` |
| server | `issues().create_or_update_remote_issue_link` | `issue_remote_links().create_or_update_remote_issue_link` |
| server | `issues().delete_comment` | `issue_comments().delete_comment` |
| server | `issues().delete_issue_property` | `issue_properties().delete_issue_property` |
| server | `issues().delete_remote_issue_link_by_global_id` | `issue_remote_links().delete_remote_issue_link_by_global_id` |
| server | `issues().delete_remote_issue_link_by_id` | `issue_remote_links().delete_remote_issue_link_by_id` |
| server | `issues().delete_worklog` | `issue_worklogs().delete_worklog` |
| server | `issues().estimate_issue_for_board` | `issue().estimate_issue_for_board` |
| server | `issues().get_agile_issue` | `issue().get_issue` |
| server | `issues().get_comment` | `issue_comments().get_comment` |
| server | `issues().get_comments` | `issue_comments().get_comments` |
| server | `issues().get_create_issue_meta_fields` | `issues().get_create_issue_meta_issue_type_id` |
| server | `issues().get_create_issue_meta_project_issue_types` | `issues().get_create_issue_meta_issue_types` |
| server | `issues().get_issue_estimation_for_board` | `issue().get_issue_estimation_for_board` |
| server | `issues().get_issue_picker_resource` | `issue_search().get_issue_picker_resource` |
| server | `issues().get_issue_property` | `issue_properties().get_issue_property` |
| server | `issues().get_issue_property_keys` | `issue_properties().get_issue_property_keys` |
| server | `issues().get_issue_watchers` | `issue_watchers().get_issue_watchers` |
| server | `issues().get_issue_worklog` | `issue_worklogs().get_issue_worklog` |
| server | `issues().get_remote_issue_link_by_id` | `issue_remote_links().get_remote_issue_link_by_id` |
| server | `issues().get_remote_issue_links` | `issue_remote_links().get_remote_issue_links` |
| server | `issues().get_votes` | `issue_votes().get_votes` |
| server | `issues().get_worklog` | `issue_worklogs().get_worklog` |
| server | `issues().rank_issues` | `issue().rank_issues` |
| server | `issues().remove_vote` | `issue_votes().remove_vote` |
| server | `issues().remove_watcher` | `issue_watchers().remove_watcher` |
| server | `issues().set_issue_property` | `issue_properties().set_issue_property` |
| server | `issues().update_comment` | `issue_comments().update_comment` |
| server | `issues().update_remote_issue_link` | `issue_remote_links().update_remote_issue_link` |
| server | `issues().update_worklog` | `issue_worklogs().update_worklog` |
| server | `jira_settings().get_issue_navigator_default_columns` | `issue_navigator_settings().get_issue_navigator_default_columns` |
| server | `jira_settings().set_issue_navigator_default_columns_form` | `issue_navigator_settings().set_issue_navigator_default_columns` |
| server | `my_preferences().get_preference` | `myself().get_preference` |
| server | `my_preferences().remove_preference` | `myself().remove_preference` |
| server | `my_preferences().set_preference` | `myself().set_preference` |
| server | `permission_schemes().get_permission_schemes` | `permission_schemes().get_all_permission_schemes` |
| server | `permissions().get_permissions` | `permissions().get_my_permissions` |
| server | `project_roles().add_project_role_actors_to_role` | `project_role_actors().add_project_role_actors_to_role` |
| server | `project_roles().delete_project_role_actors_from_role` | `project_role_actors().delete_project_role_actors_from_role` |
| server | `project_roles().get_project_role_actors_for_role` | `project_role_actors().get_project_role_actors_for_role` |
| server | `project_roles().get_project_roles_by_id` | `project_roles().get_project_role_by_id` |
| server | `project_versions().delete_version_and_swap` | `project_versions().delete_and_replace_version` |
| server | `project_versions().merge` | `project_versions().merge_versions` |
| server | `projects().add_actor_users` | `project_role_actors().add_actor_users` |
| server | `projects().assign_permission_scheme` | `project_permission_schemes().assign_permission_scheme` |
| server | `projects().delete_actor` | `project_role_actors().delete_actor` |
| server | `projects().delete_project_avatar` | `project_avatars().delete_project_avatar` |
| server | `projects().delete_project_property` | `project_properties().delete_project_property` |
| server | `projects().get_accessible_project_type_by_key` | `project_types().get_accessible_project_type_by_key` |
| server | `projects().get_all_project_avatars` | `project_avatars().get_all_project_avatars` |
| server | `projects().get_all_project_types` | `project_types().get_all_project_types` |
| server | `projects().get_all_projects` | `projects().search_projects` |
| server | `projects().get_assigned_permission_scheme` | `project_permission_schemes().get_assigned_permission_scheme` |
| server | `projects().get_project_components` | `project_components().get_project_components` |
| server | `projects().get_project_issue_security_scheme` | `project_permission_schemes().get_project_issue_security_scheme` |
| server | `projects().get_project_notification_scheme` | `projects().get_notification_scheme_for_project` |
| server | `projects().get_project_property` | `project_properties().get_project_property` |
| server | `projects().get_project_property_keys` | `project_properties().get_project_property_keys` |
| server | `projects().get_project_role` | `project_roles().get_project_role` |
| server | `projects().get_project_roles` | `project_roles().get_project_roles` |
| server | `projects().get_project_type_by_key` | `project_types().get_project_type_by_key` |
| server | `projects().get_project_versions` | `project_versions().get_project_versions` |
| server | `projects().get_project_versions_paginated` | `project_versions().get_project_versions_paginated` |
| server | `projects().get_security_levels_for_project` | `project_permission_schemes().get_security_levels_for_project` |
| server | `projects().search_for_projects` | `projects().get_projects_picker` |
| server | `projects().set_actors` | `project_role_actors().set_actors` |
| server | `projects().set_project_property` | `project_properties().set_project_property` |
| server | `projects().update_project_avatar` | `project_avatars().update_project_avatar` |
| server | `screens().add_field` | `screen_tab_fields().add_screen_tab_field` |
| server | `screens().add_tab` | `screen_tabs().add_screen_tab` |
| server | `screens().delete_tab` | `screen_tabs().delete_screen_tab` |
| server | `screens().get_all_fields` | `screen_tab_fields().get_all_screen_tab_fields` |
| server | `screens().get_all_screens` | `screens().get_screens` |
| server | `screens().get_all_tabs` | `screen_tabs().get_all_screen_tabs` |
| server | `screens().get_fields_to_add` | `screens().get_available_screen_fields` |
| server | `screens().move_field` | `screen_tab_fields().move_screen_tab_field` |
| server | `screens().move_tab` | `screen_tabs().move_screen_tab` |
| server | `screens().remove_field` | `screen_tab_fields().remove_screen_tab_field` |
| server | `screens().rename_tab` | `screen_tabs().rename_screen_tab` |
| server | `sprint().delete_sprint_property` | `sprint().delete_property` |
| server | `sprint().get_sprint_property` | `sprint().get_property` |
| server | `sprint().get_sprint_property_keys` | `sprint().get_properties_keys` |
| server | `sprint().move_issues_to_sprint` | `sprint().move_issues_to_sprint_and_rank` |
| server | `sprint().set_sprint_property` | `sprint().set_property` |
| server | `users().default_columns` | `users().get_user_default_columns` |
| server | `users().delete_user_property` | `user_properties().delete_user_property` |
| server | `users().find_assignable_users` | `user_search().find_assignable_users` |
| server | `users().find_bulk_assignable_users` | `user_search().find_bulk_assignable_users` |
| server | `users().find_users` | `user_search().find_users` |
| server | `users().find_users_for_picker` | `user_search().find_users_for_picker` |
| server | `users().find_users_with_browse_permission` | `user_search().find_users_with_browse_permission` |
| server | `users().get_user_property` | `user_properties().get_user_property` |
| server | `users().get_user_property_keys` | `user_properties().get_user_property_keys` |
| server | `users().set_columns_url_encoded` | `users().set_user_columns` |
| server | `users().set_user_property` | `user_properties().set_user_property` |
| server | `workflow_schemes().create_draft_for_parent` | `workflow_scheme_drafts().create_workflow_scheme_draft_from_parent` |
| server | `workflow_schemes().create_scheme` | `workflow_schemes().create_workflow_scheme` |
| server | `workflow_schemes().delete_default` | `workflow_schemes().delete_default_workflow` |
| server | `workflow_schemes().delete_draft_by_id` | `workflow_scheme_drafts().delete_workflow_scheme_draft` |
| server | `workflow_schemes().delete_draft_default` | `workflow_scheme_drafts().delete_draft_default_workflow` |
| server | `workflow_schemes().delete_draft_issue_type` | `workflow_scheme_drafts().delete_workflow_scheme_draft_issue_type` |
| server | `workflow_schemes().delete_draft_workflow_mapping` | `workflow_scheme_drafts().delete_draft_workflow_mapping` |
| server | `workflow_schemes().delete_scheme` | `workflow_schemes().delete_workflow_scheme` |
| server | `workflow_schemes().get_by_id` | `workflow_schemes().get_workflow_scheme` |
| server | `workflow_schemes().get_default` | `workflow_schemes().get_default_workflow` |
| server | `workflow_schemes().get_draft_by_id` | `workflow_scheme_drafts().get_workflow_scheme_draft` |
| server | `workflow_schemes().get_draft_default` | `workflow_scheme_drafts().get_draft_default_workflow` |
| server | `workflow_schemes().get_draft_issue_type` | `workflow_scheme_drafts().get_workflow_scheme_draft_issue_type` |
| server | `workflow_schemes().get_draft_workflow` | `workflow_scheme_drafts().get_draft_workflow` |
| server | `workflow_schemes().set_draft_issue_type` | `workflow_scheme_drafts().set_workflow_scheme_draft_issue_type` |
| server | `workflow_schemes().set_issue_type` | `workflow_schemes().set_workflow_scheme_issue_type` |
| server | `workflow_schemes().update_default` | `workflow_schemes().update_default_workflow` |
| server | `workflow_schemes().update_draft` | `workflow_scheme_drafts().update_workflow_scheme_draft` |
| server | `workflow_schemes().update_draft_default` | `workflow_scheme_drafts().update_draft_default_workflow` |
| server | `workflow_schemes().update_draft_workflow_mapping` | `workflow_scheme_drafts().update_draft_workflow_mapping` |

</details>

<details>
<summary>Signatures</summary>

| Surface / endpoint | 0.1.1 | 0.2.0 |
|---|---|---|
| agile GET /rest/agile/1.0/board | `board().get_all_boards().expand(value: GetAllBoardsRequestExpand) -> crate::core::Result<Page<Board>>` | `board().get_all_boards().expand(value: impl IntoIterator<Item = impl Into<GetAllBoardsRequestExpandValue>>) -> crate::core::Result<Page<Board>>` |
| agile GET /rest/software/1.0/board/{}/epic/{}/issue | `board().get_board_issues_for_epic(board_id: i64, epic_id: i64).expand(value: GetBoardIssuesForEpicRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `board().get_board_issues_for_epic(board_id: i64, epic_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/software/1.0/board/{}/sprint/{}/issue | `board().get_board_issues_for_sprint(board_id: i64, sprint_id: i64).expand(value: GetBoardIssuesForSprintRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `board().get_board_issues_for_sprint(board_id: i64, sprint_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/software/1.0/board/{}/backlog | `board().get_issues_for_backlog(board_id: i64).expand(value: GetIssuesForBacklogRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `board().get_issues_for_backlog(board_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/software/1.0/board/{}/issue | `board().get_issues_for_board(board_id: i64).expand(value: GetIssuesForBoardRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `board().get_issues_for_board(board_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/software/1.0/board/{}/epic/none/issue | `board().get_issues_without_epic_for_board(board_id: i64).expand(value: GetIssuesWithoutEpicForBoardRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `board().get_issues_without_epic_for_board(board_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/agile/1.0/board/{}/project/full | `board().get_projects_full(board_id: i64) -> crate::core::Result<GetProjectsFull>` | `board().get_projects_full(board_id: i64) -> crate::core::Result<GetProjectsFull>` |
| agile GET /rest/software/1.0/epic/{}/issue | `epic().get_issues_for_epic(epic_id_or_key: impl Into<String>).expand(value: GetIssuesForEpicRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `epic().get_issues_for_epic(epic_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/software/1.0/epic/none/issue | `epic().get_issues_without_epic().expand(value: GetIssuesWithoutEpicRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `epic().get_issues_without_epic().expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| agile GET /rest/agile/1.0/issue/{} | `issue().get_issue(issue_id_or_key: impl Into<String>).expand(value: GetIssueRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<Issue>` | `issue().get_issue(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Issue>` |
| agile DELETE /rest/operations/1.0/linkedWorkspaces/bulk | `operations().delete_workspaces(workspace_ids: DeleteWorkspacesRequestWorkspaceIds) -> crate::core::Result<()>` | `operations().delete_workspaces(workspace_ids: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<()>` |
| agile DELETE /rest/security/1.0/linkedWorkspaces/bulk | `security_information().delete_linked_workspaces(workspace_ids: DeleteLinkedWorkspacesRequestWorkspaceIds) -> crate::core::Result<()>` | `security_information().delete_linked_workspaces(workspace_ids: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<()>` |
| agile GET /rest/software/1.0/sprint/{}/issue | `sprint().get_issues_for_sprint(sprint_id: i64).expand(value: GetIssuesForSprintRequestExpand).fields(value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>) -> crate::core::Result<SoftwareIssueResults>` | `sprint().get_issues_for_sprint(sprint_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SoftwareIssueResults>` |
| assets GET /objecttype/{}/attributes | `object_types().find_object_type_attributes(id: impl Into<String>).query(value: FindObjectTypeAttributesRequestQuery) -> crate::core::Result<Vec<ObjectTypeAttribute>>` | `object_types().find_object_type_attributes(id: impl Into<String>).query(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Vec<ObjectTypeAttribute>>` |
| assets_server GET /rest/assets/1.0/aql/objects | `aql().find_objects() -> crate::core::Result<ObjectListResult>` | `aql().find_objects() -> crate::core::Result<ObjectListResult>` |
| assets_server GET /rest/assets/1.0/object/archived | `objects().get_archived_objects() -> crate::core::Result<ArchivedObjectsPage>` | `objects().get_archived_objects() -> crate::core::Result<ArchivedObjectsPage>` |
| cloud GET /rest/api/3/data-policy/project | `app_data_policies().get_policies().ids(value: GetPoliciesRequestIds) -> crate::core::Result<ProjectDataPolicies>` | `app_data_policies().get_policies().ids(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<ProjectDataPolicies>` |
| cloud GET /rest/api/3/dashboard/search | `dashboards().get_dashboards_paginated().expand(value: GetDashboardsPaginatedRequestExpand).status(value: impl Into<GetDashboardsPaginatedRequestStatus>) -> crate::core::Result<Page<Dashboard>>` | `dashboards().get_dashboards_paginated().expand(value: impl IntoIterator<Item = impl Into<GetDashboardsPaginatedRequestExpandValue>>).status(value: impl Into<GetDashboardsPaginatedRequestStatus>) -> crate::core::Result<Page<Dashboard>>` |
| cloud POST /rest/api/3/filter | `filters().create_filter(filter: Filter).expand(value: CreateFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().create_filter(filter: Filter).expand(value: impl IntoIterator<Item = impl Into<CreateFilterRequestExpandValue>>) -> crate::core::Result<Filter>` |
| cloud DELETE /rest/api/3/filter/{}/favourite | `filters().delete_favourite_for_filter(id: i64).expand(value: DeleteFavouriteForFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().delete_favourite_for_filter(id: i64).expand(value: impl IntoIterator<Item = impl Into<DeleteFavouriteForFilterRequestExpandValue>>) -> crate::core::Result<Filter>` |
| cloud GET /rest/api/3/filter/favourite | `filters().get_favourite_filters().expand(value: GetFavouriteFiltersRequestExpand) -> crate::core::Result<Vec<Filter>>` | `filters().get_favourite_filters().expand(value: impl IntoIterator<Item = impl Into<GetFavouriteFiltersRequestExpandValue>>) -> crate::core::Result<Vec<Filter>>` |
| cloud GET /rest/api/3/filter/{} | `filters().get_filter(id: i64).expand(value: GetFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().get_filter(id: i64).expand(value: impl IntoIterator<Item = impl Into<GetFilterRequestExpandValue>>) -> crate::core::Result<Filter>` |
| cloud GET /rest/api/3/filter/search | `filters().get_filters_paginated().expand(value: GetFiltersPaginatedRequestExpand) -> crate::core::Result<Page<FilterDetails>>` | `filters().get_filters_paginated().expand(value: impl IntoIterator<Item = impl Into<GetFiltersPaginatedRequestExpandValue>>) -> crate::core::Result<Page<FilterDetails>>` |
| cloud GET /rest/api/3/filter/my | `filters().get_my_filters().expand(value: GetMyFiltersRequestExpand) -> crate::core::Result<Vec<Filter>>` | `filters().get_my_filters().expand(value: impl IntoIterator<Item = impl Into<GetMyFiltersRequestExpandValue>>) -> crate::core::Result<Vec<Filter>>` |
| cloud PUT /rest/api/3/filter/{}/favourite | `filters().set_favourite_for_filter(id: i64).expand(value: SetFavouriteForFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().set_favourite_for_filter(id: i64).expand(value: impl IntoIterator<Item = impl Into<SetFavouriteForFilterRequestExpandValue>>) -> crate::core::Result<Filter>` |
| cloud PUT /rest/api/3/filter/{} | `filters().update_filter(id: i64, body: Filter).expand(value: UpdateFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().update_filter(id: i64, body: Filter).expand(value: impl IntoIterator<Item = impl Into<UpdateFilterRequestExpandValue>>) -> crate::core::Result<Filter>` |
| cloud POST /rest/api/3/issue/{}/comment | `issue_comments().add_comment(issue_id_or_key: impl Into<String>, comment_input: CommentInput).expand(value: AddCommentRequestExpand) -> crate::core::Result<Comment>` | `issue_comments().add_comment(issue_id_or_key: impl Into<String>, comment_input: CommentInput).expand(value: impl IntoIterator<Item = impl Into<AddCommentRequestExpandValue>>) -> crate::core::Result<Comment>` |
| cloud GET /rest/api/3/issue/{}/comment/{} | `issue_comments().get_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: GetCommentRequestExpand) -> crate::core::Result<Comment>` | `issue_comments().get_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetCommentRequestExpandValue>>) -> crate::core::Result<Comment>` |
| cloud GET /rest/api/3/issue/{}/comment | `issue_comments().get_comments(issue_id_or_key: impl Into<String>).expand(value: GetCommentsRequestExpand) -> crate::core::Result<PageOfComments>` | `issue_comments().get_comments(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetCommentsRequestExpandValue>>) -> crate::core::Result<PageOfComments>` |
| cloud POST /rest/api/3/comment/list | `issue_comments().get_comments_by_ids(issue_comment_list_request: IssueCommentListRequest).expand(value: GetCommentsByIdsRequestExpand) -> crate::core::Result<Page<Comment>>` | `issue_comments().get_comments_by_ids(issue_comment_list_request: IssueCommentListRequest).expand(value: impl IntoIterator<Item = impl Into<GetCommentsByIdsRequestExpandValue>>) -> crate::core::Result<Page<Comment>>` |
| cloud PUT /rest/api/3/issue/{}/comment/{} | `issue_comments().update_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: CommentInput).expand(value: UpdateCommentRequestExpand) -> crate::core::Result<Comment>` | `issue_comments().update_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: CommentInput).expand(value: impl IntoIterator<Item = impl Into<UpdateCommentRequestExpandValue>>) -> crate::core::Result<Comment>` |
| cloud GET /rest/api/3/field/search | `issue_fields().get_fields_paginated().expand(value: GetFieldsPaginatedRequestExpand).query(value: impl Into<String>) -> crate::core::Result<Page<Field>>` | `issue_fields().get_fields_paginated().expand(value: impl IntoIterator<Item = impl Into<GetFieldsPaginatedRequestExpandValue>>).query(value: impl Into<String>) -> crate::core::Result<Page<Field>>` |
| cloud GET /rest/api/3/notificationscheme/{} | `issue_notification_schemes().get_notification_scheme(id: i64).expand(value: GetNotificationSchemeRequestExpand) -> crate::core::Result<NotificationScheme>` | `issue_notification_schemes().get_notification_scheme(id: i64).expand(value: impl IntoIterator<Item = impl Into<GetNotificationSchemeRequestExpandValue>>) -> crate::core::Result<NotificationScheme>` |
| cloud GET /rest/api/3/notificationscheme/project | `issue_notification_schemes().get_notification_scheme_to_project_mappings() -> crate::core::Result<Page<NotificationSchemeAndProjectMapping>>` | `issue_notification_schemes().get_notification_scheme_to_project_mappings() -> crate::core::Result<Page<NotificationSchemeAndProjectMapping>>` |
| cloud GET /rest/api/3/notificationscheme | `issue_notification_schemes().get_notification_schemes().expand(value: GetNotificationSchemesRequestExpand) -> crate::core::Result<Page<NotificationScheme>>` | `issue_notification_schemes().get_notification_schemes().expand(value: impl IntoIterator<Item = impl Into<GetNotificationSchemesRequestExpandValue>>) -> crate::core::Result<Page<NotificationScheme>>` |
| cloud GET /rest/api/3/priority/search | `issue_priorities().search_priorities().expand(value: impl Into<String>) -> crate::core::Result<Page<Priority>>` | `issue_priorities().search_priorities().expand(value: impl Into<String>) -> crate::core::Result<Page<Priority>>` |
| cloud GET /rest/api/3/search/jql | `issue_search().search_issues().expand(value: SearchIssuesRequestExpand).fields(value: impl IntoIterator<Item = impl Into<String>>).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchAndReconcileResults>` | `issue_search().search_issues().expand(value: impl IntoIterator<Item = impl Into<SearchIssuesRequestExpandValue>>).fields(value: impl IntoIterator<Item = impl Into<String>>).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchAndReconcileResults>` |
| cloud GET /rest/api/3/issuesecurityschemes/{}/members | `issue_security_level().get_issue_security_level_members(issue_security_scheme_id: i64).expand(value: GetIssueSecurityLevelMembersRequestExpand) -> crate::core::Result<Page<IssueSecurityLevelMember>>` | `issue_security_level().get_issue_security_level_members(issue_security_scheme_id: i64).expand(value: impl IntoIterator<Item = impl Into<GetIssueSecurityLevelMembersRequestExpandValue>>) -> crate::core::Result<Page<IssueSecurityLevelMember>>` |
| cloud GET /rest/api/3/issuetypescheme | `issue_type_schemes().get_all_issue_type_schemes().expand(value: GetAllIssueTypeSchemesRequestExpand) -> crate::core::Result<Page<IssueTypeScheme>>` | `issue_type_schemes().get_all_issue_type_schemes().expand(value: impl IntoIterator<Item = impl Into<GetAllIssueTypeSchemesRequestExpandValue>>) -> crate::core::Result<Page<IssueTypeScheme>>` |
| cloud GET /rest/api/3/issuetypescreenscheme | `issue_type_screen_schemes().get_issue_type_screen_schemes().expand(value: GetIssueTypeScreenSchemesRequestExpand) -> crate::core::Result<Page<IssueTypeScreenScheme>>` | `issue_type_screen_schemes().get_issue_type_screen_schemes().expand(value: impl IntoIterator<Item = impl Into<GetIssueTypeScreenSchemesRequestExpandValue>>) -> crate::core::Result<Page<IssueTypeScreenScheme>>` |
| cloud POST /rest/api/3/issue/{}/worklog | `issue_worklogs().add_worklog(issue_id_or_key: impl Into<String>, worklog_input: WorklogInput).expand(value: AddWorklogRequestExpand) -> crate::core::Result<Worklog>` | `issue_worklogs().add_worklog(issue_id_or_key: impl Into<String>, worklog_input: WorklogInput).expand(value: impl IntoIterator<Item = impl Into<AddWorklogRequestExpandValue>>) -> crate::core::Result<Worklog>` |
| cloud GET /rest/api/3/worklog/updated | `issue_worklogs().get_ids_of_worklogs_modified_since().expand(value: GetIdsOfWorklogsModifiedSinceRequestExpand) -> crate::core::Result<ChangedWorklogs>` | `issue_worklogs().get_ids_of_worklogs_modified_since().expand(value: impl IntoIterator<Item = impl Into<GetIdsOfWorklogsModifiedSinceRequestExpandValue>>) -> crate::core::Result<ChangedWorklogs>` |
| cloud GET /rest/api/3/issue/{}/worklog | `issue_worklogs().get_issue_worklog(issue_id_or_key: impl Into<String>).expand(value: GetIssueWorklogRequestExpand) -> crate::core::Result<PageOfWorklogs>` | `issue_worklogs().get_issue_worklog(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetIssueWorklogRequestExpandValue>>) -> crate::core::Result<PageOfWorklogs>` |
| cloud GET /rest/api/3/issue/{}/worklog/{} | `issue_worklogs().get_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: GetWorklogRequestExpand) -> crate::core::Result<Worklog>` | `issue_worklogs().get_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetWorklogRequestExpandValue>>) -> crate::core::Result<Worklog>` |
| cloud POST /rest/api/3/worklog/list | `issue_worklogs().get_worklogs_for_ids(worklog_ids_request: WorklogIdsRequest).expand(value: GetWorklogsForIdsRequestExpand) -> crate::core::Result<Vec<Worklog>>` | `issue_worklogs().get_worklogs_for_ids(worklog_ids_request: WorklogIdsRequest).expand(value: impl IntoIterator<Item = impl Into<GetWorklogsForIdsRequestExpandValue>>) -> crate::core::Result<Vec<Worklog>>` |
| cloud PUT /rest/api/3/issue/{}/worklog/{} | `issue_worklogs().update_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: WorklogInput).expand(value: UpdateWorklogRequestExpand) -> crate::core::Result<Worklog>` | `issue_worklogs().update_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: WorklogInput).expand(value: impl IntoIterator<Item = impl Into<UpdateWorklogRequestExpandValue>>) -> crate::core::Result<Worklog>` |
| cloud GET /rest/api/3/issue/{} | `issues().get_issue(issue_id_or_key: impl Into<String>).expand(value: GetIssueRequestExpand).fields(value: impl IntoIterator<Item = impl Into<String>>).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Issue>` | `issues().get_issue(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetIssueRequestExpandValue>>).fields(value: impl IntoIterator<Item = impl Into<String>>).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Issue>` |
| cloud GET /rest/api/3/issue/{}/transitions | `issues().get_transitions(issue_id_or_key: impl Into<String>).expand(value: GetTransitionsRequestExpand) -> crate::core::Result<Transitions>` | `issues().get_transitions(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetTransitionsRequestExpandValue>>) -> crate::core::Result<Transitions>` |
| cloud POST /rest/api/3/expression/evaluate | `jira_expressions().evaluate_expression(jira_expression_evaluate_request: JiraExpressionEvaluateRequest).expand(value: EvaluateExpressionRequestExpand) -> crate::core::Result<JExpEvaluateJiraExpressionResult>` | `jira_expressions().evaluate_expression(jira_expression_evaluate_request: JiraExpressionEvaluateRequest).expand(value: impl IntoIterator<Item = impl Into<EvaluateExpressionRequestExpandValue>>) -> crate::core::Result<JExpEvaluateJiraExpressionResult>` |
| cloud GET /rest/api/3/myself | `myself().get_current_user().expand(value: GetCurrentUserRequestExpand) -> crate::core::Result<User>` | `myself().get_current_user().expand(value: impl IntoIterator<Item = impl Into<GetCurrentUserRequestExpandValue>>) -> crate::core::Result<User>` |
| cloud POST /rest/api/3/permissionscheme/{}/permission | `permission_schemes().create_permission_grant(scheme_id: i64, permission_grant: PermissionGrant).expand(value: CreatePermissionGrantRequestExpand) -> crate::core::Result<PermissionGrant>` | `permission_schemes().create_permission_grant(scheme_id: i64, permission_grant: PermissionGrant).expand(value: impl IntoIterator<Item = impl Into<CreatePermissionGrantRequestExpandValue>>) -> crate::core::Result<PermissionGrant>` |
| cloud POST /rest/api/3/permissionscheme | `permission_schemes().create_permission_scheme(body: PermissionScheme).expand(value: CreatePermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `permission_schemes().create_permission_scheme(body: PermissionScheme).expand(value: impl IntoIterator<Item = impl Into<CreatePermissionSchemeRequestExpandValue>>) -> crate::core::Result<PermissionScheme>` |
| cloud GET /rest/api/3/permissionscheme | `permission_schemes().get_all_permission_schemes().expand(value: GetAllPermissionSchemesRequestExpand) -> crate::core::Result<PermissionSchemes>` | `permission_schemes().get_all_permission_schemes().expand(value: impl IntoIterator<Item = impl Into<GetAllPermissionSchemesRequestExpandValue>>) -> crate::core::Result<PermissionSchemes>` |
| cloud GET /rest/api/3/permissionscheme/{} | `permission_schemes().get_permission_scheme(scheme_id: i64).expand(value: GetPermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `permission_schemes().get_permission_scheme(scheme_id: i64).expand(value: impl IntoIterator<Item = impl Into<GetPermissionSchemeRequestExpandValue>>) -> crate::core::Result<PermissionScheme>` |
| cloud GET /rest/api/3/permissionscheme/{}/permission/{} | `permission_schemes().get_permission_scheme_grant(scheme_id: i64, permission_id: i64).expand(value: GetPermissionSchemeGrantRequestExpand) -> crate::core::Result<PermissionGrant>` | `permission_schemes().get_permission_scheme_grant(scheme_id: i64, permission_id: i64).expand(value: impl IntoIterator<Item = impl Into<GetPermissionSchemeGrantRequestExpandValue>>) -> crate::core::Result<PermissionGrant>` |
| cloud GET /rest/api/3/permissionscheme/{}/permission | `permission_schemes().get_permission_scheme_grants(scheme_id: i64).expand(value: GetPermissionSchemeGrantsRequestExpand) -> crate::core::Result<PermissionGrants>` | `permission_schemes().get_permission_scheme_grants(scheme_id: i64).expand(value: impl IntoIterator<Item = impl Into<GetPermissionSchemeGrantsRequestExpandValue>>) -> crate::core::Result<PermissionGrants>` |
| cloud PUT /rest/api/3/permissionscheme/{} | `permission_schemes().update_permission_scheme(scheme_id: i64, body: PermissionScheme).expand(value: UpdatePermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `permission_schemes().update_permission_scheme(scheme_id: i64, body: PermissionScheme).expand(value: impl IntoIterator<Item = impl Into<UpdatePermissionSchemeRequestExpandValue>>) -> crate::core::Result<PermissionScheme>` |
| cloud GET /rest/api/3/mypermissions | `permissions().get_my_permissions() -> crate::core::Result<Permissions>` | `permissions().get_my_permissions() -> crate::core::Result<Permissions>` |
| cloud PUT /rest/api/3/project/{}/permissionscheme | `project_permission_schemes().assign_permission_scheme(project_key_or_id: impl Into<String>, id: Id).expand(value: AssignPermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `project_permission_schemes().assign_permission_scheme(project_key_or_id: impl Into<String>, id: Id).expand(value: impl IntoIterator<Item = impl Into<AssignPermissionSchemeRequestExpandValue>>) -> crate::core::Result<PermissionScheme>` |
| cloud GET /rest/api/3/project/{}/permissionscheme | `project_permission_schemes().get_assigned_permission_scheme(project_key_or_id: impl Into<String>).expand(value: GetAssignedPermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `project_permission_schemes().get_assigned_permission_scheme(project_key_or_id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetAssignedPermissionSchemeRequestExpandValue>>) -> crate::core::Result<PermissionScheme>` |
| cloud GET /rest/api/3/project/{}/versions | `project_versions().get_project_versions(project_id_or_key: impl Into<String>).expand(value: GetProjectVersionsRequestExpand) -> crate::core::Result<Vec<Version>>` | `project_versions().get_project_versions(project_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetProjectVersionsRequestExpandValue>>) -> crate::core::Result<Vec<Version>>` |
| cloud GET /rest/api/3/project/{}/version | `project_versions().get_project_versions_paginated(project_id_or_key: impl Into<String>).expand(value: GetProjectVersionsPaginatedRequestExpand).query(value: impl Into<String>).status(value: GetProjectVersionsPaginatedRequestStatus) -> crate::core::Result<Page<Version>>` | `project_versions().get_project_versions_paginated(project_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetProjectVersionsPaginatedRequestExpandValue>>).query(value: impl Into<String>).status(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Page<Version>>` |
| cloud GET /rest/api/3/version/{} | `project_versions().get_version(id: impl Into<String>).expand(value: GetVersionRequestExpand) -> crate::core::Result<Version>` | `project_versions().get_version(id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetVersionRequestExpandValue>>) -> crate::core::Result<Version>` |
| cloud GET /rest/api/3/project/{}/notificationscheme | `projects().get_notification_scheme_for_project(project_key_or_id: impl Into<String>).expand(value: GetNotificationSchemeForProjectRequestExpand) -> crate::core::Result<NotificationScheme>` | `projects().get_notification_scheme_for_project(project_key_or_id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetNotificationSchemeForProjectRequestExpandValue>>) -> crate::core::Result<NotificationScheme>` |
| cloud GET /rest/api/3/project/{} | `projects().get_project(project_id_or_key: impl Into<String>).expand(value: GetProjectRequestExpand).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Project>` | `projects().get_project(project_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetProjectRequestExpandValue>>).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Project>` |
| cloud GET /rest/api/3/project/search | `projects().search_projects().expand(value: SearchProjectsRequestExpand).properties(value: impl IntoIterator<Item = StringList>).query(value: impl Into<String>).status(value: impl IntoIterator<Item = impl Into<SearchProjectsRequestStatus>>) -> crate::core::Result<Page<Project>>` | `projects().search_projects().expand(value: impl IntoIterator<Item = impl Into<SearchProjectsRequestExpandValue>>).properties(value: impl IntoIterator<Item = StringList>).query(value: impl Into<String>).status(value: impl IntoIterator<Item = impl Into<SearchProjectsRequestStatus>>) -> crate::core::Result<Page<Project>>` |
| cloud PUT /rest/api/3/project/{} | `projects().update_project(project_id_or_key: impl Into<String>, update_project_details: UpdateProjectDetails).expand(value: UpdateProjectRequestExpand) -> crate::core::Result<Project>` | `projects().update_project(project_id_or_key: impl Into<String>, update_project_details: UpdateProjectDetails).expand(value: impl IntoIterator<Item = impl Into<UpdateProjectRequestExpandValue>>) -> crate::core::Result<Project>` |
| cloud GET /rest/api/3/screenscheme | `screen_schemes().get_screen_schemes().expand(value: GetScreenSchemesRequestExpand) -> crate::core::Result<Page<ScreenScheme>>` | `screen_schemes().get_screen_schemes().expand(value: impl IntoIterator<Item = impl Into<GetScreenSchemesRequestExpandValue>>) -> crate::core::Result<Page<ScreenScheme>>` |
| cloud GET /rest/api/3/field/{}/screens | `screens().get_screens_for_field(field_id: impl Into<String>).expand(value: GetScreensForFieldRequestExpand) -> crate::core::Result<Page<ScreenWithTab>>` | `screens().get_screens_for_field(field_id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetScreensForFieldRequestExpandValue>>) -> crate::core::Result<Page<ScreenWithTab>>` |
| cloud GET /rest/api/3/uiModifications | `ui_modifications_apps().get_ui_modifications().expand(value: GetUiModificationsRequestExpand) -> crate::core::Result<Page<UiModificationDetails>>` | `ui_modifications_apps().get_ui_modifications().expand(value: impl IntoIterator<Item = impl Into<GetUiModificationsRequestExpandValue>>) -> crate::core::Result<Page<UiModificationDetails>>` |
| cloud GET /rest/api/3/user/assignable/multiProjectSearch | `user_search().find_bulk_assignable_users(project_keys: FindBulkAssignableUsersRequestProjectKeys).query(value: impl Into<String>) -> crate::core::Result<Vec<User>>` | `user_search().find_bulk_assignable_users(project_keys: impl IntoIterator<Item = impl Into<String>>).query(value: impl Into<String>) -> crate::core::Result<Vec<User>>` |
| cloud GET /rest/api/3/user | `users().get_user(account_id: impl Into<String>).expand(value: GetUserRequestExpand) -> crate::core::Result<User>` | `users().get_user(account_id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetUserRequestExpandValue>>) -> crate::core::Result<User>` |
| cloud GET /rest/api/3/workflow/rule/config | `workflow_transition_rules().get_workflow_transition_rule_configurations(types: impl IntoIterator<Item = impl Into<GetWorkflowTransitionRuleConfigurationsRequestTypes>>).expand(value: GetWorkflowTransitionRuleConfigurationsRequestExpand) -> crate::core::Result<Page<WorkflowTransitionRules>>` | `workflow_transition_rules().get_workflow_transition_rule_configurations(types: impl IntoIterator<Item = impl Into<GetWorkflowTransitionRuleConfigurationsRequestTypes>>).expand(value: impl IntoIterator<Item = impl Into<GetWorkflowTransitionRuleConfigurationsRequestExpandValue>>) -> crate::core::Result<Page<WorkflowTransitionRules>>` |
| cloud POST /rest/api/3/workflow/history/list | `workflows().list_workflow_history(workflow_history_list_request: WorkflowHistoryListRequest).expand(value: ListWorkflowHistoryRequestExpand) -> crate::core::Result<WorkflowHistoryListResponseDTO>` | `workflows().list_workflow_history(workflow_history_list_request: WorkflowHistoryListRequest).expand(value: impl IntoIterator<Item = impl Into<ListWorkflowHistoryRequestExpandValue>>) -> crate::core::Result<WorkflowHistoryListResponseDTO>` |
| cloud GET /rest/api/3/workflows/search | `workflows().search_workflows().expand(value: SearchWorkflowsRequestExpand) -> crate::core::Result<WorkflowSearchResponse>` | `workflows().search_workflows().expand(value: impl IntoIterator<Item = impl Into<SearchWorkflowsRequestExpandValue>>) -> crate::core::Result<WorkflowSearchResponse>` |
| server PUT /rest/api/2/application-properties/{} | `application_properties().set_property_via_restful_table(id: impl Into<String>, body: ApplicationPropertyValue) -> crate::core::Result<ApplicationProperty>` | `jira_settings().set_application_property(id: impl Into<String>, body: SimpleApplicationProperty) -> crate::core::Result<ApplicationProperty>` |
| server DELETE /rest/api/2/universal_avatar/type/{}/owner/{}/avatar/{} | `avatars().delete_avatar(id: i64, r#type: impl Into<String>, owning_object_id: impl Into<String>) -> crate::core::Result<()>` | `avatars().delete_avatar(r#type: impl Into<String>, owning_object_id: impl Into<String>, id: i64) -> crate::core::Result<()>` |
| server GET /rest/api/2/universal_avatar/type/{}/owner/{} | `avatars().get_avatars(r#type: impl Into<String>, owning_object_id: impl Into<String>) -> crate::core::Result<GetAvatars>` | `avatars().get_avatars(r#type: impl Into<String>, entity_id: impl Into<String>) -> crate::core::Result<GetAvatars>` |
| server POST /rest/agile/1.0/backlog/issue | `backlog().move_issues_to_backlog(issue_assign_request: IssueAssignRequest) -> crate::core::Result<()>` | `backlog().move_issues_to_backlog(issues: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<()>` |
| server DELETE /rest/agile/1.0/board/{}/properties/{} | `board().delete_board_property(property_key: impl Into<String>, board_id: i64) -> crate::core::Result<()>` | `board().delete_board_property(board_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server GET /rest/agile/1.0/board/{}/sprint | `board().get_all_sprints(board_id: i64).state(value: GetAllSprintsRequestState) -> crate::core::Result<Page<Sprint>>` | `board().get_all_sprints(board_id: i64).state(value: impl Into<GetAllSprintsRequestState>) -> crate::core::Result<Page<Sprint>>` |
| server GET /rest/agile/1.0/board/{}/configuration | `board().get_board_configuration(board_id: i64) -> crate::core::Result<BoardConfig>` | `board().get_configuration(board_id: i64) -> crate::core::Result<GetConfiguration>` |
| server GET /rest/agile/1.0/board/{}/properties/{} | `board().get_board_property(property_key: impl Into<String>, board_id: i64) -> crate::core::Result<EntityProperty>` | `board().get_board_property(board_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/agile/1.0/board/{}/properties | `board().get_board_property_keys(board_id: i64) -> crate::core::Result<EntityPropertiesKeys>` | `board().get_board_property_keys(board_id: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server GET /rest/agile/1.0/board/{}/backlog | `board().get_issues_for_backlog(board_id: i64).expand(value: impl Into<String>).fields(value: GetIssuesForBacklogRequestFields) -> crate::core::Result<SearchResults>` | `board().get_issues_for_backlog(board_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server GET /rest/agile/1.0/board/{}/issue | `board().get_issues_for_board(board_id: i64).expand(value: impl Into<String>).fields(value: GetIssuesForBoardRequestFields) -> crate::core::Result<SearchResults>` | `board().get_issues_for_board(board_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server GET /rest/agile/1.0/board/{}/epic/{}/issue | `board().get_issues_for_board_epic(epic_id: i64, board_id: i64).expand(value: GetIssuesForBoardEpicRequestExpand).fields(value: GetIssuesForBoardEpicRequestFields) -> crate::core::Result<SearchResults>` | `board().get_board_issues_for_epic(board_id: i64, epic_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server GET /rest/agile/1.0/board/{}/sprint/{}/issue | `board().get_issues_for_board_sprint(sprint_id: i64, board_id: i64).expand(value: GetIssuesForBoardSprintRequestExpand).fields(value: GetIssuesForBoardSprintRequestFields) -> crate::core::Result<SearchResults>` | `board().get_board_issues_for_sprint(board_id: i64, sprint_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server GET /rest/agile/1.0/board/{}/epic/none/issue | `board().get_issues_without_epic_for_board(board_id: i64).expand(value: GetIssuesWithoutEpicForBoardRequestExpand).fields(value: GetIssuesWithoutEpicForBoardRequestFields) -> crate::core::Result<SearchResults>` | `board().get_issues_without_epic_for_board(board_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server PUT /rest/agile/1.0/board/{}/properties/{} | `board().set_board_property(property_key: impl Into<String>, board_id: i64, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<Option<EntityPropertiesKeys>>` | `board().set_board_property(board_id: impl Into<String>, property_key: impl Into<String>, property_value: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<Option<PropertyKeys>>` |
| server DELETE /rest/api/2/dashboard/{}/items/{}/properties/{} | `dashboards().delete_dashboard_item_property(property_key: impl Into<String>, item_id: impl Into<String>, dashboard_id: impl Into<String>) -> crate::core::Result<()>` | `dashboards().delete_dashboard_item_property(dashboard_id: impl Into<String>, item_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server GET /rest/api/2/dashboard/{}/items/{}/properties/{} | `dashboards().get_dashboard_item_property(property_key: impl Into<String>, item_id: impl Into<String>, dashboard_id: impl Into<String>) -> crate::core::Result<EntityProperty>` | `dashboards().get_dashboard_item_property(dashboard_id: impl Into<String>, item_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/api/2/dashboard/{}/items/{}/properties | `dashboards().get_dashboard_item_property_keys(item_id: impl Into<String>, dashboard_id: impl Into<String>) -> crate::core::Result<EntityPropertiesKeys>` | `dashboards().get_dashboard_item_property_keys(dashboard_id: impl Into<String>, item_id: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server GET /rest/api/2/dashboard | `dashboards().list() -> crate::core::Result<Dashboards>` | `dashboards().get_all_dashboards() -> crate::core::Result<PageOfDashboards>` |
| server PUT /rest/api/2/dashboard/{}/items/{}/properties/{} | `dashboards().set_dashboard_item_property(property_key: impl Into<String>, item_id: impl Into<String>, dashboard_id: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` | `dashboards().set_dashboard_item_property(dashboard_id: impl Into<String>, item_id: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` |
| server GET /rest/agile/1.0/epic/{}/issue | `epic().get_issues_for_epic(epic_id_or_key: impl Into<String>).expand(value: GetIssuesForEpicRequestExpand).fields(value: GetIssuesForEpicRequestFields) -> crate::core::Result<SearchResults>` | `epic().get_issues_for_epic(epic_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server GET /rest/agile/1.0/epic/none/issue | `epic().get_issues_without_epic().expand(value: GetIssuesWithoutEpicRequestExpand).fields(value: GetIssuesWithoutEpicRequestFields) -> crate::core::Result<SearchResults>` | `epic().get_issues_without_epic().expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server POST /rest/agile/1.0/epic/{}/issue | `epic().move_issues_to_epic(epic_id_or_key: impl Into<String>, issue_assign_request: IssueAssignRequest) -> crate::core::Result<()>` | `epic().move_issues_to_epic(epic_id_or_key: impl Into<String>) -> crate::core::Result<()>` |
| server POST /rest/agile/1.0/epic/none/issue | `epic().remove_issues_from_epic(issue_assign_request: IssueAssignRequest) -> crate::core::Result<()>` | `epic().remove_issues_from_epic(issues: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<()>` |
| server POST /rest/api/2/filter/{}/permission | `filters().add_share_permission(id: impl Into<String>) -> crate::core::Result<Vec<FilterPermission>>` | `filter_sharing().add_share_permission(id: i64, share_permission_input: SharePermissionInput) -> crate::core::Result<Vec<SharePermission>>` |
| server POST /rest/api/2/filter | `filters().create_filter().expand(value: CreateFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().create_filter(filter: Filter).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Filter>` |
| server DELETE /rest/api/2/filter/{} | `filters().delete_filter(id: impl Into<String>) -> crate::core::Result<()>` | `filters().delete_filter(id: i64) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/filter/{}/permission/{} | `filters().delete_share_permission(id: impl Into<String>, permission_id: impl Into<String>) -> crate::core::Result<()>` | `filter_sharing().delete_share_permission(id: i64, permission_id: i64) -> crate::core::Result<()>` |
| server PUT /rest/api/2/filter/{} | `filters().edit_filter(id: impl Into<String>).expand(value: EditFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().update_filter(id: i64, body: Filter).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Filter>` |
| server GET /rest/api/2/filter/favourite | `filters().get_favourite_filters().expand(value: GetFavouriteFiltersRequestExpand) -> crate::core::Result<Vec<Filter>>` | `filters().get_favourite_filters().expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Vec<Filter>>` |
| server GET /rest/api/2/filter/{} | `filters().get_filter(id: impl Into<String>).expand(value: GetFilterRequestExpand) -> crate::core::Result<Filter>` | `filters().get_filter(id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Filter>` |
| server GET /rest/api/2/filter/{}/columns | `filters().get_filter_columns(id: impl Into<String>) -> crate::core::Result<Vec<ColumnLayout>>` | `filters().get_columns(id: i64) -> crate::core::Result<Vec<ColumnLayout>>` |
| server GET /rest/api/2/filter/{}/permission/{} | `filters().get_share_permission(permission_id: impl Into<String>, id: impl Into<String>) -> crate::core::Result<FilterPermission>` | `filter_sharing().get_share_permission(id: i64, permission_id: i64) -> crate::core::Result<SharePermission>` |
| server GET /rest/api/2/filter/{}/permission | `filters().get_share_permissions(id: impl Into<String>) -> crate::core::Result<Vec<FilterPermission>>` | `filter_sharing().get_share_permissions(id: i64) -> crate::core::Result<Vec<SharePermission>>` |
| server DELETE /rest/api/2/filter/{}/columns | `filters().reset_columns(id: impl Into<String>) -> crate::core::Result<()>` | `filters().reset_columns(id: i64) -> crate::core::Result<()>` |
| server PUT /rest/api/2/filter/{}/columns | `filters().set_columns(id: impl Into<String>) -> crate::core::Result<()>` | `filters().set_columns(id: i64, column_request_body: ColumnRequestBody) -> crate::core::Result<()>` |
| server PUT /rest/api/2/filter/defaultShareScope | `filters().set_default_share_scope() -> crate::core::Result<DefaultShareScope>` | `filter_sharing().set_default_share_scope(default_share_scope: DefaultShareScope) -> crate::core::Result<DefaultShareScope>` |
| server GET /rest/api/2/groupuserpicker | `group_and_user_picker().find_users_and_groups().query(value: impl Into<String>) -> crate::core::Result<UsersAndGroups>` | `group_and_user_picker().find_users_and_groups(query: impl Into<String>) -> crate::core::Result<FoundUsersAndGroups>` |
| server POST /rest/api/2/group/user | `groups().add_user_to_group(groupname: impl Into<String>) -> crate::core::Result<Group>` | `groups().add_user_to_group(groupname: impl Into<String>, update_user_to_group: UpdateUserToGroup) -> crate::core::Result<Group>` |
| server POST /rest/api/2/group | `groups().create_group() -> crate::core::Result<Group>` | `groups().create_group(add_group: AddGroup) -> crate::core::Result<Group>` |
| server GET /rest/api/2/groups/picker | `groups().find_groups().query(value: impl Into<String>) -> crate::core::Result<GroupSuggestions>` | `groups().find_groups().query(value: impl Into<String>) -> crate::core::Result<FoundGroups>` |
| server GET /rest/api/2/group/member | `groups().get_users_from_group(groupname: impl Into<String>) -> crate::core::Result<GetUsersFromGroup>` | `groups().get_users_from_group(groupname: impl Into<String>) -> crate::core::Result<GetUsersFromGroup>` |
| server GET /rest/api/2/attachment/meta | `issue_attachments().get_attachment_meta() -> crate::core::Result<AttachmentMeta>` | `issue_attachments().get_attachment_meta() -> crate::core::Result<AttachmentSettings>` |
| server DELETE /rest/api/2/comment/{}/properties/{} | `issue_comments().delete_comment_property(property_key: impl Into<String>, comment_id: impl Into<String>) -> crate::core::Result<()>` | `issue_comment_properties().delete_comment_property(comment_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server GET /rest/api/2/comment/{}/properties/{} | `issue_comments().get_comment_property(property_key: impl Into<String>, comment_id: impl Into<String>) -> crate::core::Result<EntityProperty>` | `issue_comment_properties().get_comment_property(comment_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/api/2/comment/{}/properties | `issue_comments().get_comment_property_keys(comment_id: impl Into<String>) -> crate::core::Result<EntityPropertiesKeys>` | `issue_comment_properties().get_comment_property_keys(comment_id: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server PUT /rest/api/2/comment/{}/properties/{} | `issue_comments().set_comment_property(property_key: impl Into<String>, comment_id: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` | `issue_comment_properties().set_comment_property(comment_id: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` |
| server POST /rest/api/2/field | `issue_fields().create_custom_field() -> crate::core::Result<Field>` | `issue_fields().create_custom_field(custom_field_definition: CustomFieldDefinitionJson) -> crate::core::Result<FieldDetails>` |
| server GET /rest/api/2/customFields/{}/options | `issue_fields().get_custom_field_options(custom_field_id: impl Into<String>).query(value: impl Into<String>) -> crate::core::Result<CustomFieldOptions>` | `issue_fields().get_custom_field_options(custom_field_id: impl Into<String>).query(value: impl Into<String>) -> crate::core::Result<CustomFieldOptions>` |
| server GET /rest/api/2/customFields | `issue_fields().get_custom_fields() -> crate::core::Result<Page<CustomField>>` | `issue_fields().get_custom_fields() -> crate::core::Result<Page<CustomField>>` |
| server GET /rest/api/2/field | `issue_fields().get_fields() -> crate::core::Result<Vec<Field>>` | `issue_fields().get_fields() -> crate::core::Result<Vec<FieldDetails>>` |
| server POST /rest/api/2/issueLinkType | `issue_link_types().create_issue_link_type(issue_link_type_json: IssueLinkTypeJson) -> crate::core::Result<IssueLinkTypeJson>` | `issue_link_types().create_issue_link_type(issue_link_type: IssueLinkType) -> crate::core::Result<IssueLinkType>` |
| server GET /rest/api/2/issueLinkType/{} | `issue_link_types().get_issue_link_type(issue_link_type_id: impl Into<String>) -> crate::core::Result<IssueLinkTypeJson>` | `issue_link_types().get_issue_link_type(issue_link_type_id: impl Into<String>) -> crate::core::Result<IssueLinkType>` |
| server PUT /rest/api/2/issueLinkType/{}/order | `issue_link_types().move_issue_link_type(issue_link_type_id: impl Into<String>, issue_link_type_order_update_request: IssueLinkTypeOrderUpdateRequest) -> crate::core::Result<IssueLinkTypeJson>` | `issue_link_types().move_issue_link_type(issue_link_type_id: impl Into<String>, issue_link_type_order_update_request: IssueLinkTypeOrderUpdateRequest) -> crate::core::Result<IssueLinkType>` |
| server PUT /rest/api/2/issueLinkType/{} | `issue_link_types().update_issue_link_type(issue_link_type_id: impl Into<String>, issue_link_type_json: IssueLinkTypeJson) -> crate::core::Result<IssueLinkTypeJson>` | `issue_link_types().update_issue_link_type(issue_link_type_id: impl Into<String>, issue_link_type: IssueLinkType) -> crate::core::Result<IssueLinkType>` |
| server POST /rest/api/2/issueLink | `issue_links().link_issues(link_issue_request_json: LinkIssueRequestJson) -> crate::core::Result<()>` | `issue_links().link_issues(link_issue_request: LinkIssueRequest) -> crate::core::Result<()>` |
| server GET /rest/api/2/notificationscheme/{} | `issue_notification_schemes().get_notification_scheme(id: i64).expand(value: impl Into<String>) -> crate::core::Result<NotificationScheme>` | `issue_notification_schemes().get_notification_scheme(id: i64).expand(value: impl IntoIterator<Item = impl Into<GetNotificationSchemeRequestExpandValue>>) -> crate::core::Result<NotificationScheme>` |
| server GET /rest/api/2/notificationscheme | `issue_notification_schemes().get_notification_schemes().expand(value: impl Into<String>) -> crate::core::Result<PagedResults>` | `issue_notification_schemes().get_notification_schemes().expand(value: impl IntoIterator<Item = impl Into<GetNotificationSchemesRequestExpandValue>>) -> crate::core::Result<PageOfNotificationSchemes>` |
| server GET /rest/api/2/priority | `issue_priorities().get_priorities() -> crate::core::Result<Vec<PriorityJson>>` | `issue_priorities().get_priorities() -> crate::core::Result<Vec<Priority>>` |
| server GET /rest/api/2/priority/page | `issue_priorities().get_priorities_paginated().query(value: impl Into<String>) -> crate::core::Result<Page<PriorityJson>>` | `issue_priorities().get_priorities_paginated().query(value: impl Into<String>) -> crate::core::Result<Page<Priority>>` |
| server GET /rest/api/2/priority/{} | `issue_priorities().get_priority(id: impl Into<String>) -> crate::core::Result<PriorityJson>` | `issue_priorities().get_priority(id: impl Into<String>) -> crate::core::Result<Priority>` |
| server GET /rest/api/2/search | `issue_search().search().expand(value: SearchRequest2Expand).fields(value: SearchRequest2Fields) -> crate::core::Result<SearchResults>` | `issue_search().search_issues().expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server POST /rest/api/2/search | `issue_search().search_using_search_request(search_request: SearchRequest) -> crate::core::Result<SearchResults>` | `issue_search().search_issues_post(search_and_reconcile_request: SearchRequest) -> crate::core::Result<SearchResults>` |
| server GET /rest/api/2/securitylevel/{} | `issue_security_level().get_issue_security_level(id: impl Into<String>) -> crate::core::Result<SecurityLevelJson>` | `issue_security_level().get_issue_security_level(id: impl Into<String>) -> crate::core::Result<SecurityLevel>` |
| server GET /rest/api/2/issuesecurityschemes/{} | `issue_security_schemes().get_issue_security_scheme(id: impl Into<String>) -> crate::core::Result<SecuritySchemeJson>` | `issue_security_schemes().get_issue_security_scheme(id: i64) -> crate::core::Result<SecurityScheme>` |
| server GET /rest/api/2/issuesecurityschemes | `issue_security_schemes().get_issue_security_schemes() -> crate::core::Result<SecuritySchemesJson>` | `issue_security_schemes().get_issue_security_schemes() -> crate::core::Result<SecuritySchemes>` |
| server POST /rest/api/2/issuetypescheme | `issue_type_schemes().create_issue_type_scheme(issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate) -> crate::core::Result<IssueTypeScheme>` | `issue_type_schemes().create_issue_type_scheme(issue_type_scheme_details: IssueTypeSchemeCreateUpdate) -> crate::core::Result<IssueTypeScheme>` |
| server DELETE /rest/api/2/issuetypescheme/{} | `issue_type_schemes().delete_issue_type_scheme(scheme_id: impl Into<String>) -> crate::core::Result<()>` | `issue_type_schemes().delete_issue_type_scheme(issue_type_scheme_id: i64) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/issuetypescheme/{}/associations/{} | `issue_type_schemes().remove_project_association(proj_id_or_key: impl Into<String>, scheme_id: impl Into<String>) -> crate::core::Result<()>` | `issue_type_schemes().remove_project_association(scheme_id: impl Into<String>, proj_id_or_key: impl Into<String>) -> crate::core::Result<()>` |
| server PUT /rest/api/2/issuetypescheme/{} | `issue_type_schemes().update_issue_type_scheme(scheme_id: impl Into<String>, issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate) -> crate::core::Result<IssueTypeScheme>` | `issue_type_schemes().update_issue_type_scheme(issue_type_scheme_id: i64, issue_type_scheme_update_details: IssueTypeSchemeCreateUpdate) -> crate::core::Result<IssueTypeScheme>` |
| server POST /rest/api/2/issuetype | `issue_types().create_issue_type(issue_type_create: IssueTypeCreate) -> crate::core::Result<IssueTypeJson>` | `issue_types().create_issue_type(issue_type_create: IssueTypeCreate) -> crate::core::Result<IssueTypeDetails>` |
| server DELETE /rest/api/2/issuetype/{}/properties/{} | `issue_types().delete_issue_type_property(property_key: impl Into<String>, issue_type_id: impl Into<String>) -> crate::core::Result<()>` | `issue_type_properties().delete_issue_type_property(issue_type_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server GET /rest/api/2/issuetype/{}/alternatives | `issue_types().get_alternative_issue_types(id: impl Into<String>) -> crate::core::Result<Vec<IssueTypeJson>>` | `issue_types().get_alternative_issue_types(id: impl Into<String>) -> crate::core::Result<Vec<IssueTypeDetails>>` |
| server GET /rest/api/2/issuetype | `issue_types().get_issue_all_types() -> crate::core::Result<Vec<IssueTypeJson>>` | `issue_types().get_issue_all_types() -> crate::core::Result<Vec<IssueTypeDetails>>` |
| server GET /rest/api/2/issuetype/{} | `issue_types().get_issue_type(id: impl Into<String>) -> crate::core::Result<IssueTypeJson>` | `issue_types().get_issue_type(id: impl Into<String>) -> crate::core::Result<IssueTypeDetails>` |
| server GET /rest/api/2/issuetype/{}/properties/{} | `issue_types().get_issue_type_property(property_key: impl Into<String>, issue_type_id: impl Into<String>) -> crate::core::Result<EntityProperty>` | `issue_type_properties().get_issue_type_property(issue_type_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/api/2/issuetype/{}/properties | `issue_types().get_issue_type_property_keys(issue_type_id: impl Into<String>) -> crate::core::Result<EntityPropertiesKeys>` | `issue_type_properties().get_issue_type_property_keys(issue_type_id: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server GET /rest/api/2/issuetype/page | `issue_types().get_paginated_issue_types().query(value: impl Into<String>) -> crate::core::Result<Page<IssueTypeJson>>` | `issue_types().get_paginated_issue_types().query(value: impl Into<String>) -> crate::core::Result<Page<IssueTypeDetails>>` |
| server PUT /rest/api/2/issuetype/{}/properties/{} | `issue_types().set_issue_type_property(property_key: impl Into<String>, issue_type_id: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` | `issue_type_properties().set_issue_type_property(issue_type_id: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` |
| server PUT /rest/api/2/issuetype/{} | `issue_types().update_issue_type(id: impl Into<String>, issue_type_update: IssueTypeUpdate) -> crate::core::Result<IssueTypeJson>` | `issue_types().update_issue_type(id: impl Into<String>, issue_type_update: IssueTypeUpdate) -> crate::core::Result<IssueTypeDetails>` |
| server GET /rest/api/2/worklog/deleted | `issue_worklogs().get_ids_of_worklogs_deleted_since() -> crate::core::Result<WorklogChangedSince>` | `issue_worklogs().get_ids_of_worklogs_deleted_since() -> crate::core::Result<ChangedWorklogs>` |
| server GET /rest/api/2/worklog/updated | `issue_worklogs().get_ids_of_worklogs_modified_since() -> crate::core::Result<WorklogChangedSince>` | `issue_worklogs().get_ids_of_worklogs_modified_since() -> crate::core::Result<ChangedWorklogs>` |
| server POST /rest/api/2/issue/{}/comment | `issues().add_comment(issue_id_or_key: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<CommentJson>` | `issue_comments().add_comment(issue_id_or_key: impl Into<String>, comment_input: CommentInput).expand(value: impl IntoIterator<Item = impl Into<AddCommentRequestExpandValue>>) -> crate::core::Result<Comment>` |
| server POST /rest/api/2/issue/{}/watchers | `issues().add_watcher(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issue_watchers().add_watcher(issue_id_or_key: impl Into<String>, body: impl Into<String>) -> crate::core::Result<()>` |
| server POST /rest/api/2/issue/{}/worklog | `issues().add_worklog(issue_id_or_key: impl Into<String>) -> crate::core::Result<Worklog>` | `issue_worklogs().add_worklog(issue_id_or_key: impl Into<String>, worklog_input: WorklogInput) -> crate::core::Result<Worklog>` |
| server PUT /rest/api/2/issue/{}/assignee | `issues().assign(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issues().assign_issue(issue_id_or_key: impl Into<String>, user: User) -> crate::core::Result<()>` |
| server POST /rest/api/2/issue | `issues().create_issue() -> crate::core::Result<IssueCreateResponse>` | `issues().create_issue(issue_update_details: IssueUpdateDetails) -> crate::core::Result<CreatedIssue>` |
| server POST /rest/api/2/issue/bulk | `issues().create_issues() -> crate::core::Result<IssuesCreateResponse>` | `issues().create_issues(issues_update: IssuesUpdate) -> crate::core::Result<CreatedIssues>` |
| server POST /rest/api/2/issue/{}/remotelink | `issues().create_or_update_remote_issue_link(issue_id_or_key: impl Into<String>) -> crate::core::Result<RemoteIssueLink>` | `issue_remote_links().create_or_update_remote_issue_link(issue_id_or_key: impl Into<String>, remote_issue_link_request: RemoteIssueLinkRequest) -> crate::core::Result<RemoteIssueLink>` |
| server DELETE /rest/api/2/issue/{} | `issues().delete_issue(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issues().delete_issue(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/issue/{}/properties/{} | `issues().delete_issue_property(property_key: impl Into<String>, issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issue_properties().delete_issue_property(issue_id_or_key: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/issue/{}/remotelink/{} | `issues().delete_remote_issue_link_by_id(link_id: impl Into<String>, issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issue_remote_links().delete_remote_issue_link_by_id(issue_id_or_key: impl Into<String>, link_id: impl Into<String>) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/issue/{}/worklog/{} | `issues().delete_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>) -> crate::core::Result<()>` | `issue_worklogs().delete_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>) -> crate::core::Result<()>` |
| server POST /rest/api/2/issue/{}/transitions | `issues().do_transition(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issues().do_transition(issue_id_or_key: impl Into<String>, issue_update_details: IssueUpdateDetails) -> crate::core::Result<()>` |
| server PUT /rest/api/2/issue/{} | `issues().edit_issue(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issues().edit_issue(issue_id_or_key: impl Into<String>, issue_update_details: IssueUpdateDetails) -> crate::core::Result<()>` |
| server PUT /rest/agile/1.0/issue/{}/estimation | `issues().estimate_issue_for_board(issue_id_or_key: impl Into<String>, field_edit: FieldEdit) -> crate::core::Result<FieldValue>` | `issue().estimate_issue_for_board(issue_id_or_key: impl Into<String>) -> crate::core::Result<FieldValue>` |
| server GET /rest/agile/1.0/issue/{} | `issues().get_agile_issue(issue_id_or_key: impl Into<String>).expand(value: GetAgileIssueRequestExpand).fields(value: GetAgileIssueRequestFields) -> crate::core::Result<Issue>` | `issue().get_issue(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Issue>` |
| server GET /rest/api/2/issue/{}/comment/{} | `issues().get_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<CommentJson>` | `issue_comments().get_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetCommentRequestExpandValue>>) -> crate::core::Result<Comment>` |
| server GET /rest/api/2/issue/{}/comment | `issues().get_comments(issue_id_or_key: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<CommentsWithPaginationJson>` | `issue_comments().get_comments(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetCommentsRequestExpandValue>>) -> crate::core::Result<PageOfComments>` |
| server GET /rest/api/2/issue/createmeta/{}/issuetypes/{} | `issues().get_create_issue_meta_fields(issue_type_id: impl Into<String>, project_id_or_key: impl Into<String>) -> crate::core::Result<Page<FieldMeta>>` | `issues().get_create_issue_meta_issue_type_id(project_id_or_key: impl Into<String>, issue_type_id: impl Into<String>) -> crate::core::Result<Page<FieldMeta>>` |
| server GET /rest/api/2/issue/createmeta/{}/issuetypes | `issues().get_create_issue_meta_project_issue_types(project_id_or_key: impl Into<String>) -> crate::core::Result<Page<CreateMetaIssueType>>` | `issues().get_create_issue_meta_issue_types(project_id_or_key: impl Into<String>) -> crate::core::Result<Page<CreateMetaIssueType>>` |
| server GET /rest/api/2/issue/{}/editmeta | `issues().get_edit_issue_meta(issue_id_or_key: impl Into<String>) -> crate::core::Result<EditMeta>` | `issues().get_edit_issue_meta(issue_id_or_key: impl Into<String>) -> crate::core::Result<IssueUpdateMetadata>` |
| server GET /rest/api/2/issue/{} | `issues().get_issue(issue_id_or_key: impl Into<String>).expand(value: impl Into<String>).fields(value: impl Into<String>).properties(value: impl Into<String>) -> crate::core::Result<Issue>` | `issues().get_issue(issue_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<IssuesGetIssueRequestExpandValue>>).fields(value: impl IntoIterator<Item = impl Into<String>>).properties(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Issue>` |
| server GET /rest/api/2/issue/picker | `issues().get_issue_picker_resource().query(value: impl Into<String>) -> crate::core::Result<IssuePickerResult>` | `issue_search().get_issue_picker_resource().query(value: impl Into<String>) -> crate::core::Result<IssuePickerSuggestions>` |
| server GET /rest/api/2/issue/{}/properties/{} | `issues().get_issue_property(property_key: impl Into<String>, issue_id_or_key: impl Into<String>) -> crate::core::Result<EntityProperty>` | `issue_properties().get_issue_property(issue_id_or_key: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/api/2/issue/{}/properties | `issues().get_issue_property_keys(issue_id_or_key: impl Into<String>) -> crate::core::Result<EntityPropertiesKeys>` | `issue_properties().get_issue_property_keys(issue_id_or_key: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server GET /rest/api/2/issue/{}/worklog | `issues().get_issue_worklog(issue_id_or_key: impl Into<String>) -> crate::core::Result<WorklogWithPagination>` | `issue_worklogs().get_issue_worklog(issue_id_or_key: impl Into<String>) -> crate::core::Result<PageOfWorklogs>` |
| server GET /rest/api/2/issue/{}/remotelink/{} | `issues().get_remote_issue_link_by_id(link_id: impl Into<String>, issue_id_or_key: impl Into<String>) -> crate::core::Result<RemoteIssueLink>` | `issue_remote_links().get_remote_issue_link_by_id(issue_id_or_key: impl Into<String>, link_id: impl Into<String>) -> crate::core::Result<RemoteIssueLink>` |
| server GET /rest/api/2/issue/{}/transitions | `issues().get_transitions(issue_id_or_key: impl Into<String>) -> crate::core::Result<TransitionsMeta>` | `issues().get_transitions(issue_id_or_key: impl Into<String>) -> crate::core::Result<Transitions>` |
| server GET /rest/api/2/issue/{}/votes | `issues().get_votes(issue_id_or_key: impl Into<String>) -> crate::core::Result<Vote>` | `issue_votes().get_votes(issue_id_or_key: impl Into<String>) -> crate::core::Result<Votes>` |
| server POST /rest/api/2/issue/{}/notify | `issues().notify(issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issues().notify(issue_id_or_key: impl Into<String>, notification: Notification) -> crate::core::Result<()>` |
| server PUT /rest/api/2/issue/{}/properties/{} | `issues().set_issue_property(property_key: impl Into<String>, issue_id_or_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` | `issue_properties().set_issue_property(issue_id_or_key: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` |
| server PUT /rest/api/2/issue/{}/comment/{} | `issues().update_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<CommentJson>` | `issue_comments().update_comment(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: CommentInput).expand(value: impl IntoIterator<Item = impl Into<UpdateCommentRequestExpandValue>>) -> crate::core::Result<Comment>` |
| server PUT /rest/api/2/issue/{}/remotelink/{} | `issues().update_remote_issue_link(link_id: impl Into<String>, issue_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `issue_remote_links().update_remote_issue_link(issue_id_or_key: impl Into<String>, link_id: impl Into<String>, remote_issue_link_request: RemoteIssueLinkRequest) -> crate::core::Result<()>` |
| server PUT /rest/api/2/issue/{}/worklog/{} | `issues().update_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>) -> crate::core::Result<Worklog>` | `issue_worklogs().update_worklog(issue_id_or_key: impl Into<String>, id: impl Into<String>, body: WorklogInput) -> crate::core::Result<Worklog>` |
| server PUT /rest/api/2/settings/columns | `jira_settings().set_issue_navigator_default_columns_form() -> crate::core::Result<()>` | `issue_navigator_settings().set_issue_navigator_default_columns(column_request_body: ColumnRequestBody) -> crate::core::Result<()>` |
| server GET /rest/api/2/jql/autocompletedata | `jql().get_auto_complete() -> crate::core::Result<AutoCompleteResponse>` | `jql().get_auto_complete() -> crate::core::Result<JQLReferenceData>` |
| server GET /rest/api/2/jql/autocompletedata/suggestions | `jql().get_field_auto_complete_for_query_string() -> crate::core::Result<AutoCompleteResultWrapper>` | `jql().get_field_auto_complete_for_query_string() -> crate::core::Result<AutoCompleteSuggestions>` |
| server GET /rest/api/2/mypreferences | `my_preferences().get_preference() -> crate::core::Result<String>` | `myself().get_preference(key: impl Into<String>) -> crate::core::Result<String>` |
| server DELETE /rest/api/2/mypreferences | `my_preferences().remove_preference() -> crate::core::Result<()>` | `myself().remove_preference(key: impl Into<String>) -> crate::core::Result<()>` |
| server PUT /rest/api/2/mypreferences | `my_preferences().set_preference() -> crate::core::Result<()>` | `myself().set_preference(key: impl Into<String>, body: impl Into<String>) -> crate::core::Result<()>` |
| server POST /rest/api/2/permissionscheme/{}/permission | `permission_schemes().create_permission_grant(scheme_id: i64).expand(value: CreatePermissionGrantRequestExpand) -> crate::core::Result<PermissionGrant>` | `permission_schemes().create_permission_grant(scheme_id: i64, permission_grant: PermissionGrant).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionGrant>` |
| server POST /rest/api/2/permissionscheme | `permission_schemes().create_permission_scheme().expand(value: CreatePermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `permission_schemes().create_permission_scheme(body: PermissionScheme).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionScheme>` |
| server DELETE /rest/api/2/permissionscheme/{}/permission/{} | `permission_schemes().delete_permission_scheme_entity(permission_id: i64, scheme_id: i64) -> crate::core::Result<()>` | `permission_schemes().delete_permission_scheme_entity(scheme_id: i64, permission_id: i64) -> crate::core::Result<()>` |
| server GET /rest/api/2/permissionscheme/{} | `permission_schemes().get_permission_scheme(scheme_id: i64).expand(value: GetPermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `permission_schemes().get_permission_scheme(scheme_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionScheme>` |
| server GET /rest/api/2/permissionscheme/{}/permission/{} | `permission_schemes().get_permission_scheme_grant(permission_id: i64, scheme_id: i64).expand(value: GetPermissionSchemeGrantRequestExpand) -> crate::core::Result<PermissionGrant>` | `permission_schemes().get_permission_scheme_grant(scheme_id: i64, permission_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionGrant>` |
| server GET /rest/api/2/permissionscheme/{}/permission | `permission_schemes().get_permission_scheme_grants(scheme_id: i64).expand(value: GetPermissionSchemeGrantsRequestExpand) -> crate::core::Result<PermissionGrants>` | `permission_schemes().get_permission_scheme_grants(scheme_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionGrants>` |
| server GET /rest/api/2/permissionscheme | `permission_schemes().get_permission_schemes().expand(value: GetPermissionSchemesRequestExpand) -> crate::core::Result<PermissionSchemes>` | `permission_schemes().get_all_permission_schemes().expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionSchemes>` |
| server PUT /rest/api/2/permissionscheme/{} | `permission_schemes().update_permission_scheme(scheme_id: i64).expand(value: UpdatePermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `permission_schemes().update_permission_scheme(scheme_id: i64, body: PermissionScheme).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionScheme>` |
| server GET /rest/api/2/permissions | `permissions().get_all_permissions() -> crate::core::Result<PermissionsJson>` | `permissions().get_all_permissions() -> crate::core::Result<Permissions>` |
| server GET /rest/api/2/mypermissions | `permissions().get_permissions() -> crate::core::Result<PermissionsJson>` | `permissions().get_my_permissions() -> crate::core::Result<Permissions>` |
| server POST /rest/api/2/projectCategory | `project_categories().create_project_category(project_category: ProjectCategory) -> crate::core::Result<ProjectCategoryJson>` | `project_categories().create_project_category(project_category: ProjectCategory) -> crate::core::Result<ProjectCategory>` |
| server GET /rest/api/2/projectCategory | `project_categories().get_all_project_categories() -> crate::core::Result<Vec<ProjectCategoryJson>>` | `project_categories().get_all_project_categories() -> crate::core::Result<Vec<ProjectCategory>>` |
| server GET /rest/api/2/projectCategory/{} | `project_categories().get_project_category_by_id(id: i64) -> crate::core::Result<ProjectCategoryJson>` | `project_categories().get_project_category_by_id(id: i64) -> crate::core::Result<ProjectCategory>` |
| server PUT /rest/api/2/projectCategory/{} | `project_categories().update_project_category(id: i64, body: ProjectCategory) -> crate::core::Result<ProjectCategoryJson>` | `project_categories().update_project_category(id: i64, body: ProjectCategory) -> crate::core::Result<ProjectCategory>` |
| server POST /rest/api/2/component | `project_components().create_component() -> crate::core::Result<Component>` | `project_components().create_component(project_component: ProjectComponent) -> crate::core::Result<ProjectComponent>` |
| server GET /rest/api/2/component/{} | `project_components().get_component(id: impl Into<String>) -> crate::core::Result<Component>` | `project_components().get_component(id: impl Into<String>) -> crate::core::Result<ProjectComponent>` |
| server GET /rest/api/2/component/{}/relatedIssueCounts | `project_components().get_component_related_issues(id: impl Into<String>) -> crate::core::Result<ComponentIssueCounts>` | `project_components().get_component_related_issues(id: impl Into<String>) -> crate::core::Result<ComponentIssuesCount>` |
| server GET /rest/api/2/component/page | `project_components().get_paginated_components().query(value: impl Into<String>) -> crate::core::Result<PagedResults>` | `project_components().get_paginated_components().query(value: impl Into<String>) -> crate::core::Result<PagedResults>` |
| server PUT /rest/api/2/component/{} | `project_components().update_component(id: impl Into<String>) -> crate::core::Result<Component>` | `project_components().update_component(id: impl Into<String>, body: ProjectComponent) -> crate::core::Result<ProjectComponent>` |
| server POST /rest/api/2/role/{}/actors | `project_roles().add_project_role_actors_to_role(id: i64) -> crate::core::Result<ProjectRoleActors>` | `project_role_actors().add_project_role_actors_to_role(id: i64, actor_input: ActorInput) -> crate::core::Result<ProjectRoleActors>` |
| server PUT /rest/api/2/role/{} | `project_roles().fully_update_project_role(id: i64) -> crate::core::Result<ProjectRole>` | `project_roles().fully_update_project_role(id: i64, create_update_role_request: CreateUpdateRoleRequest) -> crate::core::Result<ProjectRole>` |
| server POST /rest/api/2/role/{} | `project_roles().partial_update_project_role(id: i64) -> crate::core::Result<ProjectRole>` | `project_roles().partial_update_project_role(id: i64, create_update_role_request: CreateUpdateRoleRequest) -> crate::core::Result<ProjectRole>` |
| server GET /rest/api/2/version/{} | `project_versions().get_version(id: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<Version>` | `project_versions().get_version(id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Version>` |
| server GET /rest/api/2/version/{}/unresolvedIssueCount | `project_versions().get_version_unresolved_issues(id: impl Into<String>) -> crate::core::Result<VersionUnresolvedIssueCounts>` | `project_versions().get_version_unresolved_issues(id: impl Into<String>) -> crate::core::Result<VersionUnresolvedIssuesCount>` |
| server PUT /rest/api/2/version/{}/mergeto/{} | `project_versions().merge(move_issues_to: impl Into<String>, id: impl Into<String>) -> crate::core::Result<()>` | `project_versions().merge_versions(id: impl Into<String>, move_issues_to: impl Into<String>) -> crate::core::Result<()>` |
| server PUT /rest/api/2/project/{}/permissionscheme | `projects().assign_permission_scheme(project_key_or_id: impl Into<String>, id: Id).expand(value: AssignPermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `project_permission_schemes().assign_permission_scheme(project_key_or_id: impl Into<String>, id: Id).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionScheme>` |
| server POST /rest/api/2/project | `projects().create_project(project_input: ProjectInput) -> crate::core::Result<ProjectIdentity>` | `projects().create_project(project_input: ProjectInput) -> crate::core::Result<ProjectIdentifiers>` |
| server DELETE /rest/api/2/project/{}/properties/{} | `projects().delete_project_property(property_key: impl Into<String>, project_id_or_key: impl Into<String>) -> crate::core::Result<()>` | `project_properties().delete_project_property(project_id_or_key: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server GET /rest/api/2/project | `projects().get_all_projects().expand(value: impl Into<String>) -> crate::core::Result<Vec<Project>>` | `projects().search_projects().expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Vec<Project>>` |
| server GET /rest/api/2/project/{}/statuses | `projects().get_all_statuses(project_id_or_key: impl Into<String>) -> crate::core::Result<Vec<IssueTypeWithStatusJson>>` | `projects().get_all_statuses(project_id_or_key: impl Into<String>) -> crate::core::Result<Vec<IssueTypeWithStatus>>` |
| server GET /rest/api/2/project/{}/permissionscheme | `projects().get_assigned_permission_scheme(project_key_or_id: impl Into<String>).expand(value: GetAssignedPermissionSchemeRequestExpand) -> crate::core::Result<PermissionScheme>` | `project_permission_schemes().get_assigned_permission_scheme(project_key_or_id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PermissionScheme>` |
| server GET /rest/api/2/project/{} | `projects().get_project(project_id_or_key: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<Project>` | `projects().get_project(project_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Project>` |
| server GET /rest/api/2/project/{}/components | `projects().get_project_components(project_id_or_key: impl Into<String>) -> crate::core::Result<Vec<Component>>` | `project_components().get_project_components(project_id_or_key: impl Into<String>) -> crate::core::Result<Vec<ProjectComponent>>` |
| server GET /rest/api/2/project/{}/issuesecuritylevelscheme | `projects().get_project_issue_security_scheme(project_key_or_id: impl Into<String>) -> crate::core::Result<SecuritySchemeJson>` | `project_permission_schemes().get_project_issue_security_scheme(project_key_or_id: impl Into<String>) -> crate::core::Result<SecurityScheme>` |
| server GET /rest/api/2/project/{}/notificationscheme | `projects().get_project_notification_scheme(project_key_or_id: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<NotificationScheme>` | `projects().get_notification_scheme_for_project(project_key_or_id: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<GetNotificationSchemeForProjectRequestExpandValue>>) -> crate::core::Result<NotificationScheme>` |
| server GET /rest/api/2/project/{}/properties/{} | `projects().get_project_property(property_key: impl Into<String>, project_id_or_key: impl Into<String>) -> crate::core::Result<EntityProperty>` | `project_properties().get_project_property(project_id_or_key: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/api/2/project/{}/properties | `projects().get_project_property_keys(project_id_or_key: impl Into<String>) -> crate::core::Result<EntityPropertiesKeys>` | `project_properties().get_project_property_keys(project_id_or_key: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server GET /rest/api/2/project/{}/versions | `projects().get_project_versions(project_id_or_key: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<Vec<Version>>` | `project_versions().get_project_versions(project_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Vec<Version>>` |
| server GET /rest/api/2/project/{}/version | `projects().get_project_versions_paginated(project_id_or_key: impl Into<String>).expand(value: impl Into<String>) -> crate::core::Result<PagedResults>` | `project_versions().get_project_versions_paginated(project_id_or_key: impl Into<String>).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<PageOfVersions>` |
| server GET /rest/api/2/project/{}/securitylevel | `projects().get_security_levels_for_project(project_key_or_id: impl Into<String>) -> crate::core::Result<SecurityListLevelJson>` | `project_permission_schemes().get_security_levels_for_project(project_key_or_id: impl Into<String>) -> crate::core::Result<ProjectIssueSecurityLevels>` |
| server PUT /rest/api/2/project/{}/properties/{} | `projects().set_project_property(property_key: impl Into<String>, project_id_or_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` | `project_properties().set_project_property(project_id_or_key: impl Into<String>, property_key: impl Into<String>, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/project/{}/priorityscheme/{} | `projects().unassign_priority_scheme(scheme_id: i64, project_key_or_id: impl Into<String>) -> crate::core::Result<PriorityScheme>` | `projects().unassign_priority_scheme(project_key_or_id: impl Into<String>, scheme_id: i64) -> crate::core::Result<PriorityScheme>` |
| server PUT /rest/api/2/project/{} | `projects().update_project(project_id_or_key: impl Into<String>, project_update: ProjectUpdate).expand(value: impl Into<String>) -> crate::core::Result<Project>` | `projects().update_project(project_id_or_key: impl Into<String>, update_project_details: UpdateProjectDetails).expand(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Project>` |
| server POST /rest/api/2/screens/{}/tabs/{}/fields | `screens().add_field(tab_id: i64, screen_id: i64) -> crate::core::Result<ScreenableField>` | `screen_tab_fields().add_screen_tab_field(screen_id: i64, tab_id: i64, add_field: AddField) -> crate::core::Result<ScreenableField>` |
| server POST /rest/api/2/screens/{}/tabs | `screens().add_tab(screen_id: i64) -> crate::core::Result<ScreenableTab>` | `screen_tabs().add_screen_tab(screen_id: i64, screenable_tab: ScreenableTab) -> crate::core::Result<ScreenableTab>` |
| server DELETE /rest/api/2/screens/{}/tabs/{} | `screens().delete_tab(tab_id: i64, screen_id: i64) -> crate::core::Result<()>` | `screen_tabs().delete_screen_tab(screen_id: i64, tab_id: i64) -> crate::core::Result<()>` |
| server GET /rest/api/2/screens/{}/tabs/{}/fields | `screens().get_all_fields(tab_id: i64, screen_id: i64) -> crate::core::Result<Vec<ScreenableField>>` | `screen_tab_fields().get_all_screen_tab_fields(screen_id: i64, tab_id: i64) -> crate::core::Result<Vec<ScreenableField>>` |
| server GET /rest/api/2/screens | `screens().get_all_screens().expand(value: impl Into<String>) -> crate::core::Result<Vec<Screen>>` | `screens().get_screens().expand(value: impl Into<String>) -> crate::core::Result<Vec<Screen>>` |
| server POST /rest/api/2/screens/{}/tabs/{}/fields/{}/move | `screens().move_field(tab_id: i64, screen_id: i64, id: impl Into<String>) -> crate::core::Result<()>` | `screen_tab_fields().move_screen_tab_field(screen_id: i64, tab_id: i64, id: impl Into<String>, move_field: MoveField) -> crate::core::Result<()>` |
| server POST /rest/api/2/screens/{}/tabs/{}/move/{} | `screens().move_tab(tab_id: i64, screen_id: i64, pos: i64) -> crate::core::Result<()>` | `screen_tabs().move_screen_tab(screen_id: i64, tab_id: i64, pos: i64) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/screens/{}/tabs/{}/fields/{} | `screens().remove_field(tab_id: i64, screen_id: i64, id: impl Into<String>) -> crate::core::Result<()>` | `screen_tab_fields().remove_screen_tab_field(screen_id: i64, tab_id: i64, id: impl Into<String>) -> crate::core::Result<()>` |
| server PUT /rest/api/2/screens/{}/tabs/{} | `screens().rename_tab(tab_id: i64, screen_id: i64) -> crate::core::Result<ScreenableTab>` | `screen_tabs().rename_screen_tab(screen_id: i64, tab_id: i64, screenable_tab: ScreenableTab) -> crate::core::Result<ScreenableTab>` |
| server PUT /rest/api/2/screens/{}/tabs/{}/fields/{}/updateShowWhenEmptyIndicator/{} | `screens().update_show_when_empty_indicator(tab_id: i64, screen_id: i64, new_value: bool, id: impl Into<String>) -> crate::core::Result<()>` | `screens().update_show_when_empty_indicator(screen_id: i64, tab_id: i64, id: impl Into<String>, new_value: bool) -> crate::core::Result<()>` |
| server GET /rest/api/2/serverInfo | `server_info().get_server_info() -> crate::core::Result<ServerInfo>` | `server_info().get_server_info() -> crate::core::Result<ServerInformation>` |
| server POST /rest/agile/1.0/sprint | `sprint().create_sprint(sprint_create: SprintCreate) -> crate::core::Result<Sprint>` | `sprint().create_sprint(name: impl Into<String>) -> crate::core::Result<Sprint>` |
| server DELETE /rest/agile/1.0/sprint/{}/properties/{} | `sprint().delete_sprint_property(property_key: impl Into<String>, sprint_id: i64) -> crate::core::Result<()>` | `sprint().delete_property(sprint_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<()>` |
| server GET /rest/agile/1.0/sprint/{}/issue | `sprint().get_issues_for_sprint(sprint_id: i64).expand(value: GetIssuesForSprintRequestExpand).fields(value: GetIssuesForSprintRequestFields) -> crate::core::Result<SearchResults>` | `sprint().get_issues_for_sprint(sprint_id: i64).expand(value: impl IntoIterator<Item = impl Into<String>>).fields(value: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<SearchResults>` |
| server GET /rest/agile/1.0/sprint/{}/properties/{} | `sprint().get_sprint_property(property_key: impl Into<String>, sprint_id: i64) -> crate::core::Result<EntityProperty>` | `sprint().get_property(sprint_id: impl Into<String>, property_key: impl Into<String>) -> crate::core::Result<EntityProperty>` |
| server GET /rest/agile/1.0/sprint/{}/properties | `sprint().get_sprint_property_keys(sprint_id: i64) -> crate::core::Result<EntityPropertiesKeys>` | `sprint().get_properties_keys(sprint_id: impl Into<String>) -> crate::core::Result<PropertyKeys>` |
| server POST /rest/agile/1.0/sprint/{}/issue | `sprint().move_issues_to_sprint(sprint_id: i64, issue_assign_request: IssueAssignRequest) -> crate::core::Result<()>` | `sprint().move_issues_to_sprint_and_rank(sprint_id: i64, issue_rank_request: IssueAssignRequest) -> crate::core::Result<()>` |
| server POST /rest/agile/1.0/sprint/{} | `sprint().partially_update_sprint(sprint_id: i64, sprint: Sprint) -> crate::core::Result<Sprint>` | `sprint().partially_update_sprint(sprint_id: i64).state(value: impl Into<String>) -> crate::core::Result<Sprint>` |
| server PUT /rest/agile/1.0/sprint/{}/properties/{} | `sprint().set_sprint_property(property_key: impl Into<String>, sprint_id: i64, body: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` | `sprint().set_property(sprint_id: impl Into<String>, property_key: impl Into<String>, property_value: std::collections::HashMap<String, serde_json::Value>) -> crate::core::Result<()>` |
| server POST /rest/agile/1.0/sprint/{}/swap | `sprint().swap_sprint(sprint_id: i64, sprint_swap: SprintSwap) -> crate::core::Result<()>` | `sprint().swap_sprint(sprint_id: i64) -> crate::core::Result<()>` |
| server PUT /rest/agile/1.0/sprint/{} | `sprint().update_sprint(sprint_id: i64, sprint: Sprint) -> crate::core::Result<Sprint>` | `sprint().update_sprint(sprint_id: i64, name: impl Into<String>, state: impl Into<String>) -> crate::core::Result<Sprint>` |
| server POST /rest/api/2/user | `users().create_user(user_write: UserWrite) -> crate::core::Result<UserWrite>` | `users().create_user(new_user_details: UserWrite) -> crate::core::Result<UserWrite>` |
| server GET /rest/api/2/user/assignable/search | `users().find_assignable_users() -> crate::core::Result<User>` | `user_search().find_assignable_users() -> crate::core::Result<Vec<User>>` |
| server GET /rest/api/2/user/assignable/multiProjectSearch | `users().find_bulk_assignable_users() -> crate::core::Result<User>` | `user_search().find_bulk_assignable_users(project_keys: impl IntoIterator<Item = impl Into<String>>) -> crate::core::Result<Vec<User>>` |
| server GET /rest/api/2/user/search | `users().find_users() -> crate::core::Result<User>` | `user_search().find_users() -> crate::core::Result<Vec<User>>` |
| server GET /rest/api/2/user/picker | `users().find_users_for_picker().query(value: impl Into<String>) -> crate::core::Result<UserPickerResults>` | `user_search().find_users_for_picker(query: impl Into<String>) -> crate::core::Result<FoundUsers>` |
| server GET /rest/api/2/user/viewissue/search | `users().find_users_with_browse_permission() -> crate::core::Result<User>` | `user_search().find_users_with_browse_permission() -> crate::core::Result<Vec<User>>` |
| server GET /rest/api/2/user/properties | `users().get_user_property_keys() -> crate::core::Result<EntityPropertiesKeys>` | `user_properties().get_user_property_keys() -> crate::core::Result<PropertyKeys>` |
| server PUT /rest/api/2/user/columns | `users().set_columns_url_encoded() -> crate::core::Result<()>` | `users().set_user_columns(user_column_request_body: SetUserColumnsRequestUserColumnRequestBody) -> crate::core::Result<()>` |
| server DELETE /rest/api/2/workflowscheme/{}/draft/issuetype/{} | `workflow_schemes().delete_draft_issue_type(issue_type: impl Into<String>, id: i64) -> crate::core::Result<WorkflowScheme>` | `workflow_scheme_drafts().delete_workflow_scheme_draft_issue_type(id: i64, issue_type: impl Into<String>) -> crate::core::Result<WorkflowScheme>` |
| server DELETE /rest/api/2/workflowscheme/{}/issuetype/{} | `workflow_schemes().delete_workflow_scheme_issue_type(issue_type: impl Into<String>, id: i64) -> crate::core::Result<WorkflowScheme>` | `workflow_schemes().delete_workflow_scheme_issue_type(id: i64, issue_type: impl Into<String>) -> crate::core::Result<WorkflowScheme>` |
| server GET /rest/api/2/workflowscheme/{}/draft/issuetype/{} | `workflow_schemes().get_draft_issue_type(issue_type: impl Into<String>, id: i64) -> crate::core::Result<IssueTypeMapping>` | `workflow_scheme_drafts().get_workflow_scheme_draft_issue_type(id: i64, issue_type: impl Into<String>) -> crate::core::Result<IssueTypeWorkflowMapping>` |
| server GET /rest/api/2/workflowscheme/{}/issuetype/{} | `workflow_schemes().get_workflow_scheme_issue_type(issue_type: impl Into<String>, id: i64) -> crate::core::Result<IssueTypeMapping>` | `workflow_schemes().get_workflow_scheme_issue_type(id: i64, issue_type: impl Into<String>) -> crate::core::Result<IssueTypeWorkflowMapping>` |
| server PUT /rest/api/2/workflowscheme/{}/draft/issuetype/{} | `workflow_schemes().set_draft_issue_type(issue_type: impl Into<String>, id: i64, body: IssueTypeMapping) -> crate::core::Result<WorkflowScheme>` | `workflow_scheme_drafts().set_workflow_scheme_draft_issue_type(id: i64, issue_type: impl Into<String>, body: IssueTypeWorkflowMapping) -> crate::core::Result<WorkflowScheme>` |
| server PUT /rest/api/2/workflowscheme/{}/issuetype/{} | `workflow_schemes().set_issue_type(issue_type: impl Into<String>, id: i64, body: IssueTypeMapping) -> crate::core::Result<WorkflowScheme>` | `workflow_schemes().set_workflow_scheme_issue_type(id: i64, issue_type: impl Into<String>, body: IssueTypeWorkflowMapping) -> crate::core::Result<WorkflowScheme>` |
| server PUT /rest/api/2/workflowscheme/{}/default | `workflow_schemes().update_default(id: i64, default: DefaultModel) -> crate::core::Result<WorkflowScheme>` | `workflow_schemes().update_default_workflow(id: i64, default_workflow: DefaultModel) -> crate::core::Result<WorkflowScheme>` |
| server PUT /rest/api/2/workflowscheme/{}/draft/default | `workflow_schemes().update_draft_default(id: i64, default: DefaultModel) -> crate::core::Result<WorkflowScheme>` | `workflow_scheme_drafts().update_draft_default_workflow(id: i64, default_workflow: DefaultModel) -> crate::core::Result<WorkflowScheme>` |
| server GET /rest/api/2/statuscategory | `workflow_status_categories().get_status_categories() -> crate::core::Result<Vec<StatusCategoryJson>>` | `workflow_status_categories().get_status_categories() -> crate::core::Result<Vec<StatusCategory>>` |
| server GET /rest/api/2/statuscategory/{} | `workflow_status_categories().get_status_category(id_or_key: impl Into<String>) -> crate::core::Result<StatusCategoryJson>` | `workflow_status_categories().get_status_category(id_or_key: impl Into<String>) -> crate::core::Result<StatusCategory>` |
| server GET /rest/api/2/status/page | `workflow_statuses().get_paginated_statuses().query(value: impl Into<String>) -> crate::core::Result<Page<StatusJson>>` | `workflow_statuses().get_paginated_statuses().query(value: impl Into<String>) -> crate::core::Result<Page<StatusDetails>>` |
| server GET /rest/api/2/status/{} | `workflow_statuses().get_status(id_or_name: impl Into<String>) -> crate::core::Result<StatusJson>` | `workflow_statuses().get_status(id_or_name: impl Into<String>) -> crate::core::Result<StatusDetails>` |
| server GET /rest/api/2/status | `workflow_statuses().get_statuses() -> crate::core::Result<Vec<StatusJson>>` | `workflow_statuses().get_statuses() -> crate::core::Result<Vec<StatusDetails>>` |
| service_desk GET /rest/servicedeskapi/request/{}/attachment | `request().get_attachments_for_request(issue_id_or_key: impl Into<String>, start: i64, limit: i64) -> crate::core::Result<Page<Attachment>>` | `request().get_attachments_for_request(issue_id_or_key: impl Into<String>) -> crate::core::Result<Page<Attachment>>` |

</details>

<details>
<summary>Model types</summary>

| 0.1.1 | 0.2.0 |
|---|---|
| `jira::server::ApplicationPropertyValue` | `jira::server::SimpleApplicationProperty` |
| `jira::server::AttachmentMeta` | `jira::server::AttachmentSettings` |
| `jira::server::AutoCompleteResponseVisibleFieldNames` | `jira::server::JQLReferenceDataVisibleFieldNames` |
| `jira::server::AutoCompleteResponseVisibleFunctionNames` | `jira::server::JQLReferenceDataVisibleFunctionNames` |
| `jira::server::AutoCompleteResponse` | `jira::server::JQLReferenceData` |
| `jira::server::AutoCompleteResultWrapperResults` | `jira::server::AutoCompleteSuggestionsResults` |
| `jira::server::AutoCompleteResultWrapper` | `jira::server::AutoCompleteSuggestions` |
| `jira::server::BoardConfig` | `jira::server::GetConfiguration` |
| `jira::server::CommentJson` | `jira::server::Comment` |
| `jira::server::CommentsWithPaginationJson` | `jira::server::PageOfComments` |
| `jira::server::ComponentAssigneeType` | `jira::server::ProjectComponentAssigneeType` |
| `jira::server::ComponentIssueCounts` | `jira::server::ComponentIssuesCount` |
| `jira::server::Component` | `jira::server::ProjectComponent` |
| `jira::server::Dashboards` | `jira::server::PageOfDashboards` |
| `jira::server::EditMeta` | `jira::server::IssueUpdateMetadata` |
| `jira::server::EntityPropertiesKeys` | `jira::server::PropertyKeys` |
| `jira::server::EntityPropertyKey` | `jira::server::PropertyKey` |
| `jira::server::Field` | `jira::server::FieldDetails` |
| `jira::server::FilterPermission` | `jira::server::SharePermission` |
| `jira::server::GroupJson` | `jira::server::GroupName` |
| `jira::server::GroupSuggestions` | `jira::server::FoundGroups` |
| `jira::server::IssueCreateResponse` | `jira::server::CreatedIssue` |
| `jira::server::IssueLinkTypeJson` | `jira::server::IssueLinkType` |
| `jira::server::IssuePickerResult` | `jira::server::IssuePickerSuggestions` |
| `jira::server::IssueTypeJson` | `jira::server::IssueTypeDetails` |
| `jira::server::IssueTypeMapping` | `jira::server::IssueTypeWorkflowMapping` |
| `jira::server::IssueTypeWithStatusJson` | `jira::server::IssueTypeWithStatus` |
| `jira::server::IssueUpdate` | `jira::server::IssueUpdateDetails` |
| `jira::server::IssuesCreateResponse` | `jira::server::CreatedIssues` |
| `jira::server::LinkIssueRequestJson` | `jira::server::LinkIssueRequest` |
| `jira::server::NotificationJson` | `jira::server::Notification` |
| `jira::server::PermissionsJson` | `jira::server::Permissions` |
| `jira::server::PriorityJson` | `jira::server::Priority` |
| `jira::server::ProjectCategoryJson` | `jira::server::ProjectCategory` |
| `jira::server::ProjectIdentity` | `jira::server::ProjectIdentifiers` |
| `jira::server::ProjectUpdateAssigneeType` | `jira::server::UpdateProjectDetailsAssigneeType` |
| `jira::server::ProjectUpdate` | `jira::server::UpdateProjectDetails` |
| `jira::server::RankingConfig` | `jira::server::GetConfigurationRanking` |
| `jira::server::RemoteIssueLinkCreateOrUpdateRequest` | `jira::server::RemoteIssueLinkRequest` |
| `jira::server::SecurityLevelJson` | `jira::server::SecurityLevel` |
| `jira::server::SecurityListLevelJson` | `jira::server::ProjectIssueSecurityLevels` |
| `jira::server::SecuritySchemeJson` | `jira::server::SecurityScheme` |
| `jira::server::SecuritySchemesJson` | `jira::server::SecuritySchemes` |
| `jira::server::ServerInfo` | `jira::server::ServerInformation` |
| `jira::server::StatusCategoryJson` | `jira::server::StatusCategory` |
| `jira::server::StatusJson` | `jira::server::StatusDetails` |
| `jira::server::Subquery` | `jira::server::GetConfigurationSubQuery` |
| `jira::server::TransitionsMeta` | `jira::server::Transitions` |
| `jira::server::UserJson` | `jira::server::UserDetails` |
| `jira::server::UserPickerResultsUsers` | `jira::server::FoundUsersUsers` |
| `jira::server::UserPickerResults` | `jira::server::FoundUsers` |
| `jira::server::UsersAndGroups` | `jira::server::FoundUsersAndGroups` |
| `jira::server::VersionUnresolvedIssueCounts` | `jira::server::VersionUnresolvedIssuesCount` |
| `jira::server::VisibilityJsonType` | `jira::server::VisibilityType` |
| `jira::server::VisibilityJson` | `jira::server::Visibility` |
| `jira::server::Vote` | `jira::server::Votes` |
| `jira::server::WorklogChangedSince` | `jira::server::ChangedWorklogs` |
| `jira::server::WorklogWithPagination` | `jira::server::PageOfWorklogs` |

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
| `server::GetAgileIssueRequest` | `server::IssueGetIssueRequest` |
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
| `server::GetIssuesForBoardSprintRequest` | `server::GetBoardIssuesForSprintRequest` |
| `server::GetPermissionSchemesRequest` | `server::GetAllPermissionSchemesRequest` |
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

<details>
<summary>Parameter types removed or renamed</summary>

| 0.1.1 | 0.2.0 |
|---|---|
| `agile::DeleteLinkedWorkspacesRequestWorkspaceIds` | `impl IntoIterator<Item = impl Into<String>>` positional argument |
| `agile::DeleteWorkspacesRequestWorkspaceIds` | `impl IntoIterator<Item = impl Into<String>>` positional argument |
| `agile::GetAllBoardsRequestExpand` | `agile::GetAllBoardsRequestExpandValue` list |
| `agile::GetBoardIssuesForEpicRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetBoardIssuesForSprintRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssueRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssuesForBacklogRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssuesForBoardRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssuesForEpicRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssuesForSprintRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssuesWithoutEpicForBoardRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `agile::GetIssuesWithoutEpicRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `assets::FindObjectTypeAttributesRequestQuery` | `value: impl IntoIterator<Item = impl Into<String>>` via `.query(...)` |
| `cloud::AddCommentRequestExpand` | `cloud::AddCommentRequestExpandValue` list |
| `cloud::AddWorklogRequestExpand` | `cloud::AddWorklogRequestExpandValue` list |
| `cloud::AssignPermissionSchemeRequestExpand` | `cloud::AssignPermissionSchemeRequestExpandValue` list |
| `cloud::CreateFilterRequestExpand` | `cloud::CreateFilterRequestExpandValue` list |
| `cloud::CreatePermissionGrantRequestExpand` | `cloud::CreatePermissionGrantRequestExpandValue` list |
| `cloud::CreatePermissionSchemeRequestExpand` | `cloud::CreatePermissionSchemeRequestExpandValue` list |
| `cloud::DeleteFavouriteForFilterRequestExpand` | `cloud::DeleteFavouriteForFilterRequestExpandValue` list |
| `cloud::EvaluateExpressionRequestExpand` | `cloud::EvaluateExpressionRequestExpandValue` list |
| `cloud::FindBulkAssignableUsersRequestProjectKeys` | `impl IntoIterator<Item = impl Into<String>>` positional argument |
| `cloud::GetAllIssueTypeSchemesRequestExpand` | `cloud::GetAllIssueTypeSchemesRequestExpandValue` list |
| `cloud::GetAllPermissionSchemesRequestExpand` | `cloud::GetAllPermissionSchemesRequestExpandValue` list |
| `cloud::GetAssignedPermissionSchemeRequestExpand` | `cloud::GetAssignedPermissionSchemeRequestExpandValue` list |
| `cloud::GetCommentRequestExpand` | `cloud::GetCommentRequestExpandValue` list |
| `cloud::GetCommentsByIdsRequestExpand` | `cloud::GetCommentsByIdsRequestExpandValue` list |
| `cloud::GetCommentsRequestExpand` | `cloud::GetCommentsRequestExpandValue` list |
| `cloud::GetCurrentUserRequestExpand` | `cloud::GetCurrentUserRequestExpandValue` list |
| `cloud::GetDashboardsPaginatedRequestExpand` | `cloud::GetDashboardsPaginatedRequestExpandValue` list |
| `cloud::GetFavouriteFiltersRequestExpand` | `cloud::GetFavouriteFiltersRequestExpandValue` list |
| `cloud::GetFieldsPaginatedRequestExpand` | `cloud::GetFieldsPaginatedRequestExpandValue` list |
| `cloud::GetFilterRequestExpand` | `cloud::GetFilterRequestExpandValue` list |
| `cloud::GetFiltersPaginatedRequestExpand` | `cloud::GetFiltersPaginatedRequestExpandValue` list |
| `cloud::GetIdsOfWorklogsModifiedSinceRequestExpand` | `cloud::GetIdsOfWorklogsModifiedSinceRequestExpandValue` list |
| `cloud::GetIssueRequestExpand` | `cloud::GetIssueRequestExpandValue` list |
| `cloud::GetIssueSecurityLevelMembersRequestExpand` | `cloud::GetIssueSecurityLevelMembersRequestExpandValue` list |
| `cloud::GetIssueTypeScreenSchemesRequestExpand` | `cloud::GetIssueTypeScreenSchemesRequestExpandValue` list |
| `cloud::GetIssueWorklogRequestExpand` | `cloud::GetIssueWorklogRequestExpandValue` list |
| `cloud::GetMyFiltersRequestExpand` | `cloud::GetMyFiltersRequestExpandValue` list |
| `cloud::GetMyPermissionsRequestPermissions` | `value: impl IntoIterator<Item = impl Into<String>>` via `.permissions(...)` |
| `cloud::GetNotificationSchemeForProjectRequestExpand` | `cloud::GetNotificationSchemeForProjectRequestExpandValue` list |
| `cloud::GetNotificationSchemeRequestExpand` | `cloud::GetNotificationSchemeRequestExpandValue` list |
| `cloud::GetNotificationSchemesRequestExpand` | `cloud::GetNotificationSchemesRequestExpandValue` list |
| `cloud::GetPermissionSchemeGrantRequestExpand` | `cloud::GetPermissionSchemeGrantRequestExpandValue` list |
| `cloud::GetPermissionSchemeGrantsRequestExpand` | `cloud::GetPermissionSchemeGrantsRequestExpandValue` list |
| `cloud::GetPermissionSchemeRequestExpand` | `cloud::GetPermissionSchemeRequestExpandValue` list |
| `cloud::GetPoliciesRequestIds` | `value: impl IntoIterator<Item = impl Into<String>>` via `.ids(...)` |
| `cloud::GetProjectRequestExpand` | `cloud::GetProjectRequestExpandValue` list |
| `cloud::GetProjectVersionsPaginatedRequestExpand` | `cloud::GetProjectVersionsPaginatedRequestExpandValue` list |
| `cloud::GetProjectVersionsPaginatedRequestStatus` | `value: impl IntoIterator<Item = impl Into<String>>` via `.status(...)` |
| `cloud::GetProjectVersionsRequestExpand` | `cloud::GetProjectVersionsRequestExpandValue` list |
| `cloud::GetScreenSchemesRequestExpand` | `cloud::GetScreenSchemesRequestExpandValue` list |
| `cloud::GetScreensForFieldRequestExpand` | `cloud::GetScreensForFieldRequestExpandValue` list |
| `cloud::GetTransitionsRequestExpand` | `cloud::GetTransitionsRequestExpandValue` list |
| `cloud::GetUiModificationsRequestExpand` | `cloud::GetUiModificationsRequestExpandValue` list |
| `cloud::GetUserRequestExpand` | `cloud::GetUserRequestExpandValue` list |
| `cloud::GetVersionRequestExpand` | `cloud::GetVersionRequestExpandValue` list |
| `cloud::GetWorkflowTransitionRuleConfigurationsRequestExpand` | `cloud::GetWorkflowTransitionRuleConfigurationsRequestExpandValue` list |
| `cloud::GetWorklogRequestExpand` | `cloud::GetWorklogRequestExpandValue` list |
| `cloud::GetWorklogsForIdsRequestExpand` | `cloud::GetWorklogsForIdsRequestExpandValue` list |
| `cloud::ListWorkflowHistoryRequestExpand` | `cloud::ListWorkflowHistoryRequestExpandValue` list |
| `cloud::SearchIssuesRequestExpand` | `cloud::SearchIssuesRequestExpandValue` list |
| `cloud::SearchProjectsRequestExpand` | `cloud::SearchProjectsRequestExpandValue` list |
| `cloud::SearchProjectsRequestTypeKey` | `value: impl IntoIterator<Item = impl Into<String>>` via `.type_key(...)` |
| `cloud::SearchWorkflowsRequestExpand` | `cloud::SearchWorkflowsRequestExpandValue` list |
| `cloud::SetFavouriteForFilterRequestExpand` | `cloud::SetFavouriteForFilterRequestExpandValue` list |
| `cloud::UpdateCommentRequestExpand` | `cloud::UpdateCommentRequestExpandValue` list |
| `cloud::UpdateFilterRequestExpand` | `cloud::UpdateFilterRequestExpandValue` list |
| `cloud::UpdatePermissionSchemeRequestExpand` | `cloud::UpdatePermissionSchemeRequestExpandValue` list |
| `cloud::UpdateProjectRequestExpand` | `cloud::UpdateProjectRequestExpandValue` list |
| `cloud::UpdateWorklogRequestExpand` | `cloud::UpdateWorklogRequestExpandValue` list |
| `server::AssignPermissionSchemeRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::CreateFilterRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::CreatePermissionGrantRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::CreatePermissionSchemeRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::EditFilterRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::FindBulkAssignableUsersRequestProjectKeys` | `impl IntoIterator<Item = impl Into<String>>` positional argument |
| `server::GetAgileIssueRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetAgileIssueRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetAllBoardsRequestType` | `impl IntoIterator<Item = impl Into<String>>` via `.r#type(...)` |
| `server::GetAssignedPermissionSchemeRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetFavouriteFiltersRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetFilterRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesForBacklogRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesForBoardEpicRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesForBoardEpicRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesForBoardRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesForBoardSprintRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesForBoardSprintRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesForEpicRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesForEpicRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesForSprintRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesForSprintRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesWithoutEpicForBoardRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesWithoutEpicForBoardRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetIssuesWithoutEpicRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetIssuesWithoutEpicRequestFields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::GetPermissionSchemeGrantRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetPermissionSchemeGrantsRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetPermissionSchemeRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::GetPermissionSchemesRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::SearchRequest2Expand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |
| `server::SearchRequest2Fields` | `value: impl IntoIterator<Item = impl Into<String>>` via `.fields(...)` |
| `server::UpdatePermissionSchemeRequestExpand` | `value: impl IntoIterator<Item = impl Into<String>>` via `.expand(...)` |

</details>

### Fixed

- Cloud field searches and their streams can read pages containing system fields without `schema`.
- Data Center user searches return `Vec<User>` instead of one `User`, so list responses deserialize correctly.
- Paging parameters previously declared as strings are numbers on Data Center, three Cloud requests, and Assets Data Center.
- The `ChangedWorklogs` stream ends when the response sets `lastPage`.
- Jira Software Cloud issue-list `fields` now serializes field names; `get_projects_full` has `stream()` over its one answer, and the Service Management attachment listing exposes its actual paging controls.
- Data Center project search calls the full project endpoint while `get_projects_picker()` calls the picker endpoint.

Data Center `issue_worklogs().get_issue_worklog()` returns every worklog in one response: on Data Center 11.3.11, `startAt` and `maxResults` are ignored, so its `stream()` makes one request and loses nothing. Data Center `find_assignable_users`, `find_bulk_assignable_users`, and `find_users_with_browse_permission` also ignore `startAt`; they have no `stream()`, and only the first `maxResults` users can be read.

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

[0.3.0]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.3.0
[0.2.0]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.2.0
[0.1.1]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.1.1
[0.1.0]: https://github.com/MrRefactoring/jira.rs/releases/tag/v0.1.0
