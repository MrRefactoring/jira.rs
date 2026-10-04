# Cloud and Server/Data Center parity

One row per operation, comparing the Cloud clients (`CloudClient`, and `AgileClient` for Jira Software) with the Server/Data Center client (`ServerClient`). Operations are paired by HTTP method and path, with `/rest/api/3` read as `/rest/api/2`.

This is the state at 0.1.1, before the parity work; the last column is what each row becomes. Cloud names are canonical.

## Status

| Status | Meaning |
|---|---|
| identical | Same accessor, method, parameters, response type name and `.stream()` on both. |
| renamed | Same operation under a different method name. |
| service differs | Same operation behind a different service accessor. |
| signature differs | Positional or builder parameters differ in name, order or type. |
| type differs | The response type is named differently. |
| stream missing | The request is paginated and has no `.stream()`. |
| platform-only | The operation exists on one platform only and stays there. |

## Summary

444 Server/DC operations, 573 Cloud operations (476 platform, 97 Jira Software); 295 are paired.

| Status | Operations |
|---|---|
| platform-only (Cloud) | 278 |
| signature differs | 150 |
| platform-only (Server/DC) | 149 |
| type differs | 122 |
| service differs | 111 |
| renamed | 57 |
| identical | 37 |
| stream missing | 24 |
| stream missing on Cloud | 23 |
| cloud-only stream missing | 17 |
| server-only stream missing | 7 |

A paired row can carry several statuses.

## Paired operations

| Cloud | Server/DC | Endpoint | Status | Target |
|---|---|---|---|---|
| `AgileClient::backlog().move_issues_to_backlog` | `backlog().move_issues_to_backlog` | `POST /rest/agile/1.0/backlog/issue` | signature differs | align parameter names, order and types |
| `AgileClient::board().create_board` | `board().create_board` | `POST /rest/agile/1.0/board` | identical | — |
| `AgileClient::board().delete_board` | `board().delete_board` | `DELETE /rest/agile/1.0/board/{…}` | identical | — |
| `AgileClient::board().delete_board_property` | `board().delete_board_property` | `DELETE /rest/agile/1.0/board/{…}/properties/{…}` | signature differs | align parameter names, order and types |
| `AgileClient::board().get_all_boards` | `board().get_all_boards` | `GET /rest/agile/1.0/board` | identical | — |
| `AgileClient::board().get_all_sprints` | `board().get_all_sprints` | `GET /rest/agile/1.0/board/{…}/sprint` | signature differs, type differs, stream missing on Cloud | align parameter names, order and types; align response type name; add Cloud `.stream()` |
| `AgileClient::board().get_all_versions` | `board().get_all_versions` | `GET /rest/agile/1.0/board/{…}/version` | type differs, stream missing on Cloud | align response type name; add Cloud `.stream()` |
| `AgileClient::board().get_board` | `board().get_board` | `GET /rest/agile/1.0/board/{…}` | identical | — |
| `AgileClient::board().get_board_issues_for_epic` | `board().get_issues_for_board_epic` | `GET /rest/agile/1.0/board/{…}/epic/{…}/issue` | renamed, signature differs, type differs, stream missing, stream missing on Cloud | rename to `get_board_issues_for_epic`; align parameter names, order and types; answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::board().get_board_issues_for_sprint` | `board().get_issues_for_board_sprint` | `GET /rest/agile/1.0/board/{…}/sprint/{…}/issue` | renamed, signature differs, type differs, stream missing, stream missing on Cloud | rename to `get_board_issues_for_sprint`; align parameter names, order and types; answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::board().get_board_property` | `board().get_board_property` | `GET /rest/agile/1.0/board/{…}/properties/{…}` | signature differs | align parameter names, order and types |
| `AgileClient::board().get_board_property_keys` | `board().get_board_property_keys` | `GET /rest/agile/1.0/board/{…}/properties` | signature differs, type differs | align parameter names, order and types; answer as `PropertyKeys` naming |
| `AgileClient::board().get_configuration` | `board().get_board_configuration` | `GET /rest/agile/1.0/board/{…}/configuration` | renamed, type differs | rename to `get_configuration`; align response type name |
| `AgileClient::board().get_epics` | `board().get_epics` | `GET /rest/agile/1.0/board/{…}/epic` | type differs, stream missing on Cloud | align response type name; add Cloud `.stream()` |
| `AgileClient::board().get_issues_for_backlog` | `board().get_issues_for_backlog` | `GET /rest/agile/1.0/board/{…}/backlog` | signature differs, type differs, stream missing, stream missing on Cloud | align parameter names, order and types; answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::board().get_issues_for_board` | `board().get_issues_for_board` | `GET /rest/agile/1.0/board/{…}/issue` | signature differs, type differs, stream missing, stream missing on Cloud | align parameter names, order and types; answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::board().get_issues_without_epic_for_board` | `board().get_issues_without_epic_for_board` | `GET /rest/agile/1.0/board/{…}/epic/none/issue` | type differs, stream missing, stream missing on Cloud | answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::board().get_projects` | `board().get_projects` | `GET /rest/agile/1.0/board/{…}/project` | type differs, stream missing on Cloud | align response type name; add Cloud `.stream()` |
| `AgileClient::board().set_board_property` | `board().set_board_property` | `PUT /rest/agile/1.0/board/{…}/properties/{…}` | signature differs, type differs | align parameter names, order and types; align response type name |
| `AgileClient::epic().get_epic` | `epic().get_epic` | `GET /rest/agile/1.0/epic/{…}` | identical | — |
| `AgileClient::epic().get_issues_for_epic` | `epic().get_issues_for_epic` | `GET /rest/agile/1.0/epic/{…}/issue` | type differs, stream missing, stream missing on Cloud | answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::epic().get_issues_without_epic` | `epic().get_issues_without_epic` | `GET /rest/agile/1.0/epic/none/issue` | type differs, stream missing, stream missing on Cloud | answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::epic().move_issues_to_epic` | `epic().move_issues_to_epic` | `POST /rest/agile/1.0/epic/{…}/issue` | signature differs | align parameter names, order and types |
| `AgileClient::epic().partially_update_epic` | `epic().partially_update_epic` | `POST /rest/agile/1.0/epic/{…}` | identical | — |
| `AgileClient::epic().rank_epics` | `epic().rank_epics` | `PUT /rest/agile/1.0/epic/{…}/rank` | identical | — |
| `AgileClient::epic().remove_issues_from_epic` | `epic().remove_issues_from_epic` | `POST /rest/agile/1.0/epic/none/issue` | signature differs | align parameter names, order and types |
| `AgileClient::issue().estimate_issue_for_board` | `issues().estimate_issue_for_board` | `PUT /rest/agile/1.0/issue/{…}/estimation` | service differs, signature differs, type differs | move to `issue()`; align parameter names, order and types; answer as `EstimateIssueForBoard` naming |
| `AgileClient::issue().get_issue` | `issues().get_agile_issue` | `GET /rest/agile/1.0/issue/{…}` | renamed, service differs, signature differs | rename to `get_issue`; move to `issue()`; align parameter names, order and types |
| `AgileClient::issue().get_issue_estimation_for_board` | `issues().get_issue_estimation_for_board` | `GET /rest/agile/1.0/issue/{…}/estimation` | service differs, type differs | move to `issue()`; align response type name |
| `AgileClient::issue().rank_issues` | `issues().rank_issues` | `PUT /rest/agile/1.0/issue/rank` | service differs, type differs | move to `issue()`; align response type name |
| `AgileClient::sprint().create_sprint` | `sprint().create_sprint` | `POST /rest/agile/1.0/sprint` | signature differs | align parameter names, order and types |
| `AgileClient::sprint().delete_property` | `sprint().delete_sprint_property` | `DELETE /rest/agile/1.0/sprint/{…}/properties/{…}` | renamed, signature differs | rename to `delete_property`; align parameter names, order and types |
| `AgileClient::sprint().delete_sprint` | `sprint().delete_sprint` | `DELETE /rest/agile/1.0/sprint/{…}` | identical | — |
| `AgileClient::sprint().get_issues_for_sprint` | `sprint().get_issues_for_sprint` | `GET /rest/agile/1.0/sprint/{…}/issue` | type differs, stream missing, stream missing on Cloud | answer as `SoftwareIssueResults` naming; add `.stream()`; add Cloud `.stream()` |
| `AgileClient::sprint().get_properties_keys` | `sprint().get_sprint_property_keys` | `GET /rest/agile/1.0/sprint/{…}/properties` | renamed, signature differs, type differs | rename to `get_properties_keys`; align parameter names, order and types; answer as `PropertyKeys` naming |
| `AgileClient::sprint().get_property` | `sprint().get_sprint_property` | `GET /rest/agile/1.0/sprint/{…}/properties/{…}` | renamed, signature differs | rename to `get_property`; align parameter names, order and types |
| `AgileClient::sprint().get_sprint` | `sprint().get_sprint` | `GET /rest/agile/1.0/sprint/{…}` | identical | — |
| `AgileClient::sprint().move_issues_to_sprint_and_rank` | `sprint().move_issues_to_sprint` | `POST /rest/agile/1.0/sprint/{…}/issue` | renamed, signature differs | rename to `move_issues_to_sprint_and_rank`; align parameter names, order and types |
| `AgileClient::sprint().partially_update_sprint` | `sprint().partially_update_sprint` | `POST /rest/agile/1.0/sprint/{…}` | signature differs | align parameter names, order and types |
| `AgileClient::sprint().set_property` | `sprint().set_sprint_property` | `PUT /rest/agile/1.0/sprint/{…}/properties/{…}` | renamed, signature differs | rename to `set_property`; align parameter names, order and types |
| `AgileClient::sprint().swap_sprint` | `sprint().swap_sprint` | `POST /rest/agile/1.0/sprint/{…}/swap` | signature differs | align parameter names, order and types |
| `AgileClient::sprint().update_sprint` | `sprint().update_sprint` | `PUT /rest/agile/1.0/sprint/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::application_roles().get_all_application_roles` | `application_roles().get_all` | `GET /rest/api/2/applicationrole` | renamed | rename to `get_all_application_roles` |
| `CloudClient::application_roles().get_application_role` | `application_roles().get_application_role` | `GET /rest/api/2/applicationrole/{…}` | identical | — |
| `CloudClient::avatars().delete_avatar` | `avatars().delete_avatar` | `DELETE /rest/api/2/universal_avatar/type/{…}/owner/{…}/avatar/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::avatars().get_all_system_avatars` | `avatars().get_all_system_avatars` | `GET /rest/api/2/avatar/{…}/system` | signature differs, type differs | align parameter names, order and types; answer as `SystemAvatars` naming |
| `CloudClient::avatars().get_avatars` | `avatars().get_avatars` | `GET /rest/api/2/universal_avatar/type/{…}/owner/{…}` | signature differs, type differs | align parameter names, order and types; answer as `Avatars` naming |
| `CloudClient::dashboards().delete_dashboard_item_property` | `dashboards().delete_dashboard_item_property` | `DELETE /rest/api/2/dashboard/{…}/items/{…}/properties/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::dashboards().get_all_dashboards` | `dashboards().list` | `GET /rest/api/2/dashboard` | renamed, signature differs, type differs, stream missing, stream missing on Cloud | rename to `get_all_dashboards`; align parameter names, order and types; answer as `PageOfDashboards` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::dashboards().get_dashboard` | `dashboards().get_dashboard` | `GET /rest/api/2/dashboard/{…}` | identical | — |
| `CloudClient::dashboards().get_dashboard_item_property` | `dashboards().get_dashboard_item_property` | `GET /rest/api/2/dashboard/{…}/items/{…}/properties/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::dashboards().get_dashboard_item_property_keys` | `dashboards().get_dashboard_item_property_keys` | `GET /rest/api/2/dashboard/{…}/items/{…}/properties` | signature differs, type differs | align parameter names, order and types; answer as `PropertyKeys` naming |
| `CloudClient::dashboards().set_dashboard_item_property` | `dashboards().set_dashboard_item_property` | `PUT /rest/api/2/dashboard/{…}/items/{…}/properties/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::filter_sharing().add_share_permission` | `filters().add_share_permission` | `POST /rest/api/2/filter/{…}/permission` | service differs, signature differs, type differs | move to `filter_sharing()`; align parameter names, order and types; answer as `Vec<SharePermission>` naming |
| `CloudClient::filter_sharing().delete_share_permission` | `filters().delete_share_permission` | `DELETE /rest/api/2/filter/{…}/permission/{…}` | service differs, signature differs | move to `filter_sharing()`; align parameter names, order and types |
| `CloudClient::filter_sharing().get_default_share_scope` | `filters().get_default_share_scope` | `GET /rest/api/2/filter/defaultShareScope` | service differs | move to `filter_sharing()` |
| `CloudClient::filter_sharing().get_share_permission` | `filters().get_share_permission` | `GET /rest/api/2/filter/{…}/permission/{…}` | service differs, signature differs, type differs | move to `filter_sharing()`; align parameter names, order and types; answer as `SharePermission` naming |
| `CloudClient::filter_sharing().get_share_permissions` | `filters().get_share_permissions` | `GET /rest/api/2/filter/{…}/permission` | service differs, signature differs, type differs | move to `filter_sharing()`; align parameter names, order and types; answer as `Vec<SharePermission>` naming |
| `CloudClient::filter_sharing().set_default_share_scope` | `filters().set_default_share_scope` | `PUT /rest/api/2/filter/defaultShareScope` | service differs, signature differs | move to `filter_sharing()`; align parameter names, order and types |
| `CloudClient::filters().create_filter` | `filters().create_filter` | `POST /rest/api/2/filter` | signature differs | align parameter names, order and types |
| `CloudClient::filters().delete_filter` | `filters().delete_filter` | `DELETE /rest/api/2/filter/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::filters().get_columns` | `filters().get_filter_columns` | `GET /rest/api/2/filter/{…}/columns` | renamed, signature differs, type differs | rename to `get_columns`; align parameter names, order and types; answer as `Vec<ColumnItem>` naming |
| `CloudClient::filters().get_favourite_filters` | `filters().get_favourite_filters` | `GET /rest/api/2/filter/favourite` | identical | — |
| `CloudClient::filters().get_filter` | `filters().get_filter` | `GET /rest/api/2/filter/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::filters().reset_columns` | `filters().reset_columns` | `DELETE /rest/api/2/filter/{…}/columns` | signature differs | align parameter names, order and types |
| `CloudClient::filters().set_columns` | `filters().set_columns` | `PUT /rest/api/2/filter/{…}/columns` | signature differs | align parameter names, order and types |
| `CloudClient::filters().update_filter` | `filters().edit_filter` | `PUT /rest/api/2/filter/{…}` | renamed, signature differs | rename to `update_filter`; align parameter names, order and types |
| `CloudClient::group_and_user_picker().find_users_and_groups` | `group_and_user_picker().find_users_and_groups` | `GET /rest/api/2/groupuserpicker` | signature differs, type differs | align parameter names, order and types; answer as `FoundUsersAndGroups` naming |
| `CloudClient::groups().add_user_to_group` | `groups().add_user_to_group` | `POST /rest/api/2/group/user` | signature differs | align parameter names, order and types |
| `CloudClient::groups().create_group` | `groups().create_group` | `POST /rest/api/2/group` | signature differs | align parameter names, order and types |
| `CloudClient::groups().find_groups` | `groups().find_groups` | `GET /rest/api/2/groups/picker` | signature differs, type differs | align parameter names, order and types; answer as `FoundGroups` naming |
| `CloudClient::groups().get_users_from_group` | `groups().get_users_from_group` | `GET /rest/api/2/group/member` | signature differs, type differs, stream missing | align parameter names, order and types; answer as `Page<UserDetails>` naming; add `.stream()` |
| `CloudClient::groups().remove_group` | `groups().remove_group` | `DELETE /rest/api/2/group` | signature differs | align parameter names, order and types |
| `CloudClient::groups().remove_user_from_group` | `groups().remove_user_from_group` | `DELETE /rest/api/2/group/user` | signature differs | align parameter names, order and types |
| `CloudClient::issue_attachments().add_attachment` | `issues().add_attachment` | `POST /rest/api/2/issue/{…}/attachments` | service differs, type differs | move to `issue_attachments()`; answer as `Vec<Attachment>` naming |
| `CloudClient::issue_attachments().get_attachment` | `issue_attachments().get_attachment` | `GET /rest/api/2/attachment/{…}` | type differs | answer as `AttachmentMetadata` naming |
| `CloudClient::issue_attachments().get_attachment_meta` | `issue_attachments().get_attachment_meta` | `GET /rest/api/2/attachment/meta` | type differs | answer as `AttachmentSettings` naming |
| `CloudClient::issue_attachments().remove_attachment` | `issue_attachments().remove_attachment` | `DELETE /rest/api/2/attachment/{…}` | identical | — |
| `CloudClient::issue_comment_properties().delete_comment_property` | `issue_comments().delete_comment_property` | `DELETE /rest/api/2/comment/{…}/properties/{…}` | service differs, signature differs | move to `issue_comment_properties()`; align parameter names, order and types |
| `CloudClient::issue_comment_properties().get_comment_property` | `issue_comments().get_comment_property` | `GET /rest/api/2/comment/{…}/properties/{…}` | service differs, signature differs | move to `issue_comment_properties()`; align parameter names, order and types |
| `CloudClient::issue_comment_properties().get_comment_property_keys` | `issue_comments().get_comment_property_keys` | `GET /rest/api/2/comment/{…}/properties` | service differs, type differs | move to `issue_comment_properties()`; answer as `PropertyKeys` naming |
| `CloudClient::issue_comment_properties().set_comment_property` | `issue_comments().set_comment_property` | `PUT /rest/api/2/comment/{…}/properties/{…}` | service differs, signature differs | move to `issue_comment_properties()`; align parameter names, order and types |
| `CloudClient::issue_comments().add_comment` | `issues().add_comment` | `POST /rest/api/2/issue/{…}/comment` | service differs, signature differs, type differs | move to `issue_comments()`; align parameter names, order and types; answer as `Comment` naming |
| `CloudClient::issue_comments().delete_comment` | `issues().delete_comment` | `DELETE /rest/api/2/issue/{…}/comment/{…}` | service differs | move to `issue_comments()` |
| `CloudClient::issue_comments().get_comment` | `issues().get_comment` | `GET /rest/api/2/issue/{…}/comment/{…}` | service differs, signature differs, type differs | move to `issue_comments()`; align parameter names, order and types; answer as `Comment` naming |
| `CloudClient::issue_comments().get_comments` | `issues().get_comments` | `GET /rest/api/2/issue/{…}/comment` | service differs, signature differs, type differs, stream missing, stream missing on Cloud | move to `issue_comments()`; align parameter names, order and types; answer as `PageOfComments` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::issue_comments().update_comment` | `issues().update_comment` | `PUT /rest/api/2/issue/{…}/comment/{…}` | service differs, signature differs, type differs | move to `issue_comments()`; align parameter names, order and types; answer as `Comment` naming |
| `CloudClient::issue_custom_field_options().get_custom_field_option` | `issue_custom_field_options().get_custom_field_option` | `GET /rest/api/2/customFieldOption/{…}` | identical | — |
| `CloudClient::issue_fields().create_custom_field` | `issue_fields().create_custom_field` | `POST /rest/api/2/field` | signature differs, type differs | align parameter names, order and types; answer as `FieldDetails` naming |
| `CloudClient::issue_fields().get_fields` | `issue_fields().get_fields` | `GET /rest/api/2/field` | type differs | answer as `Vec<FieldDetails>` naming |
| `CloudClient::issue_link_types().create_issue_link_type` | `issue_link_types().create_issue_link_type` | `POST /rest/api/2/issueLinkType` | signature differs, type differs | align parameter names, order and types; answer as `IssueLinkType` naming |
| `CloudClient::issue_link_types().delete_issue_link_type` | `issue_link_types().delete_issue_link_type` | `DELETE /rest/api/2/issueLinkType/{…}` | identical | — |
| `CloudClient::issue_link_types().get_issue_link_type` | `issue_link_types().get_issue_link_type` | `GET /rest/api/2/issueLinkType/{…}` | type differs | answer as `IssueLinkType` naming |
| `CloudClient::issue_link_types().get_issue_link_types` | `issue_link_types().get_issue_link_types` | `GET /rest/api/2/issueLinkType` | identical | — |
| `CloudClient::issue_link_types().update_issue_link_type` | `issue_link_types().update_issue_link_type` | `PUT /rest/api/2/issueLinkType/{…}` | signature differs, type differs | align parameter names, order and types; answer as `IssueLinkType` naming |
| `CloudClient::issue_links().delete_issue_link` | `issue_links().delete_issue_link` | `DELETE /rest/api/2/issueLink/{…}` | identical | — |
| `CloudClient::issue_links().get_issue_link` | `issue_links().get_issue_link` | `GET /rest/api/2/issueLink/{…}` | identical | — |
| `CloudClient::issue_links().link_issues` | `issue_links().link_issues` | `POST /rest/api/2/issueLink` | signature differs | align parameter names, order and types |
| `CloudClient::issue_navigator_settings().get_issue_navigator_default_columns` | `jira_settings().get_issue_navigator_default_columns` | `GET /rest/api/2/settings/columns` | service differs, type differs | move to `issue_navigator_settings()`; answer as `Vec<ColumnItem>` naming |
| `CloudClient::issue_navigator_settings().set_issue_navigator_default_columns` | `jira_settings().set_issue_navigator_default_columns_form` | `PUT /rest/api/2/settings/columns` | renamed, service differs, signature differs | rename to `set_issue_navigator_default_columns`; move to `issue_navigator_settings()`; align parameter names, order and types |
| `CloudClient::issue_notification_schemes().get_notification_scheme` | `issue_notification_schemes().get_notification_scheme` | `GET /rest/api/2/notificationscheme/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::issue_notification_schemes().get_notification_schemes` | `issue_notification_schemes().get_notification_schemes` | `GET /rest/api/2/notificationscheme` | signature differs, type differs, stream missing, stream missing on Cloud | align parameter names, order and types; answer as `Page<NotificationScheme>` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::issue_priorities().get_priority` | `issue_priorities().get_priority` | `GET /rest/api/2/priority/{…}` | type differs | answer as `Priority` naming |
| `CloudClient::issue_properties().delete_issue_property` | `issues().delete_issue_property` | `DELETE /rest/api/2/issue/{…}/properties/{…}` | service differs, signature differs | move to `issue_properties()`; align parameter names, order and types |
| `CloudClient::issue_properties().get_issue_property` | `issues().get_issue_property` | `GET /rest/api/2/issue/{…}/properties/{…}` | service differs, signature differs | move to `issue_properties()`; align parameter names, order and types |
| `CloudClient::issue_properties().get_issue_property_keys` | `issues().get_issue_property_keys` | `GET /rest/api/2/issue/{…}/properties` | service differs, type differs | move to `issue_properties()`; answer as `PropertyKeys` naming |
| `CloudClient::issue_properties().set_issue_property` | `issues().set_issue_property` | `PUT /rest/api/2/issue/{…}/properties/{…}` | service differs, signature differs | move to `issue_properties()`; align parameter names, order and types |
| `CloudClient::issue_remote_links().create_or_update_remote_issue_link` | `issues().create_or_update_remote_issue_link` | `POST /rest/api/2/issue/{…}/remotelink` | service differs, signature differs, type differs | move to `issue_remote_links()`; align parameter names, order and types; answer as `RemoteIssueLinkIdentifies` naming |
| `CloudClient::issue_remote_links().delete_remote_issue_link_by_global_id` | `issues().delete_remote_issue_link_by_global_id` | `DELETE /rest/api/2/issue/{…}/remotelink` | service differs | move to `issue_remote_links()` |
| `CloudClient::issue_remote_links().delete_remote_issue_link_by_id` | `issues().delete_remote_issue_link_by_id` | `DELETE /rest/api/2/issue/{…}/remotelink/{…}` | service differs, signature differs | move to `issue_remote_links()`; align parameter names, order and types |
| `CloudClient::issue_remote_links().get_remote_issue_link_by_id` | `issues().get_remote_issue_link_by_id` | `GET /rest/api/2/issue/{…}/remotelink/{…}` | service differs, signature differs | move to `issue_remote_links()`; align parameter names, order and types |
| `CloudClient::issue_remote_links().get_remote_issue_links` | `issues().get_remote_issue_links` | `GET /rest/api/2/issue/{…}/remotelink` | service differs, type differs | move to `issue_remote_links()`; align response type name |
| `CloudClient::issue_remote_links().update_remote_issue_link` | `issues().update_remote_issue_link` | `PUT /rest/api/2/issue/{…}/remotelink/{…}` | service differs, signature differs | move to `issue_remote_links()`; align parameter names, order and types |
| `CloudClient::issue_resolutions().get_resolution` | `issue_resolutions().get_resolution` | `GET /rest/api/2/resolution/{…}` | type differs | answer as `Resolution` naming |
| `CloudClient::issue_search().get_issue_picker_resource` | `issues().get_issue_picker_resource` | `GET /rest/api/2/issue/picker` | service differs, signature differs, type differs | move to `issue_search()`; align parameter names, order and types; answer as `IssuePickerSuggestions` naming |
| `CloudClient::issue_search().search_issues` | `issue_search().search` | `GET /rest/api/2/search` | renamed, signature differs, type differs, stream missing, stream missing on Cloud | rename to `search_issues`; align parameter names, order and types; answer as `SearchAndReconcileResults` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::issue_security_level().get_issue_security_level` | `issue_security_level().get_issue_security_level` | `GET /rest/api/2/securitylevel/{…}` | type differs | answer as `SecurityLevel` naming |
| `CloudClient::issue_security_schemes().get_issue_security_scheme` | `issue_security_schemes().get_issue_security_scheme` | `GET /rest/api/2/issuesecurityschemes/{…}` | signature differs, type differs | align parameter names, order and types; answer as `SecurityScheme` naming |
| `CloudClient::issue_security_schemes().get_issue_security_schemes` | `issue_security_schemes().get_issue_security_schemes` | `GET /rest/api/2/issuesecurityschemes` | type differs | answer as `SecuritySchemes` naming |
| `CloudClient::issue_type_properties().delete_issue_type_property` | `issue_types().delete_issue_type_property` | `DELETE /rest/api/2/issuetype/{…}/properties/{…}` | service differs, signature differs | move to `issue_type_properties()`; align parameter names, order and types |
| `CloudClient::issue_type_properties().get_issue_type_property` | `issue_types().get_issue_type_property` | `GET /rest/api/2/issuetype/{…}/properties/{…}` | service differs, signature differs | move to `issue_type_properties()`; align parameter names, order and types |
| `CloudClient::issue_type_properties().get_issue_type_property_keys` | `issue_types().get_issue_type_property_keys` | `GET /rest/api/2/issuetype/{…}/properties` | service differs, type differs | move to `issue_type_properties()`; answer as `PropertyKeys` naming |
| `CloudClient::issue_type_properties().set_issue_type_property` | `issue_types().set_issue_type_property` | `PUT /rest/api/2/issuetype/{…}/properties/{…}` | service differs, signature differs | move to `issue_type_properties()`; align parameter names, order and types |
| `CloudClient::issue_type_schemes().create_issue_type_scheme` | `issue_type_schemes().create_issue_type_scheme` | `POST /rest/api/2/issuetypescheme` | signature differs, type differs | align parameter names, order and types; answer as `IssueTypeSchemeID` naming |
| `CloudClient::issue_type_schemes().delete_issue_type_scheme` | `issue_type_schemes().delete_issue_type_scheme` | `DELETE /rest/api/2/issuetypescheme/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::issue_type_schemes().get_all_issue_type_schemes` | `issue_type_schemes().get_all_issue_type_schemes` | `GET /rest/api/2/issuetypescheme` | type differs, stream missing | answer as `Page<IssueTypeScheme>` naming; add `.stream()` |
| `CloudClient::issue_type_schemes().update_issue_type_scheme` | `issue_type_schemes().update_issue_type_scheme` | `PUT /rest/api/2/issuetypescheme/{…}` | signature differs, type differs | align parameter names, order and types; align response type name |
| `CloudClient::issue_types().create_issue_type` | `issue_types().create_issue_type` | `POST /rest/api/2/issuetype` | type differs | answer as `IssueTypeDetails` naming |
| `CloudClient::issue_types().delete_issue_type` | `issue_types().delete_issue_type` | `DELETE /rest/api/2/issuetype/{…}` | identical | — |
| `CloudClient::issue_types().get_alternative_issue_types` | `issue_types().get_alternative_issue_types` | `GET /rest/api/2/issuetype/{…}/alternatives` | type differs | answer as `Vec<IssueTypeDetails>` naming |
| `CloudClient::issue_types().get_issue_all_types` | `issue_types().get_issue_all_types` | `GET /rest/api/2/issuetype` | type differs | answer as `Vec<IssueTypeDetails>` naming |
| `CloudClient::issue_types().get_issue_type` | `issue_types().get_issue_type` | `GET /rest/api/2/issuetype/{…}` | type differs | answer as `IssueTypeDetails` naming |
| `CloudClient::issue_types().update_issue_type` | `issue_types().update_issue_type` | `PUT /rest/api/2/issuetype/{…}` | type differs | answer as `IssueTypeDetails` naming |
| `CloudClient::issue_votes().add_vote` | `issues().add_vote` | `POST /rest/api/2/issue/{…}/votes` | service differs | move to `issue_votes()` |
| `CloudClient::issue_votes().get_votes` | `issues().get_votes` | `GET /rest/api/2/issue/{…}/votes` | service differs, type differs | move to `issue_votes()`; answer as `Votes` naming |
| `CloudClient::issue_votes().remove_vote` | `issues().remove_vote` | `DELETE /rest/api/2/issue/{…}/votes` | service differs | move to `issue_votes()` |
| `CloudClient::issue_watchers().add_watcher` | `issues().add_watcher` | `POST /rest/api/2/issue/{…}/watchers` | service differs, signature differs | move to `issue_watchers()`; align parameter names, order and types |
| `CloudClient::issue_watchers().get_issue_watchers` | `issues().get_issue_watchers` | `GET /rest/api/2/issue/{…}/watchers` | service differs | move to `issue_watchers()` |
| `CloudClient::issue_watchers().remove_watcher` | `issues().remove_watcher` | `DELETE /rest/api/2/issue/{…}/watchers` | service differs, signature differs | move to `issue_watchers()`; align parameter names, order and types |
| `CloudClient::issue_worklogs().add_worklog` | `issues().add_worklog` | `POST /rest/api/2/issue/{…}/worklog` | service differs, signature differs | move to `issue_worklogs()`; align parameter names, order and types |
| `CloudClient::issue_worklogs().delete_worklog` | `issues().delete_worklog` | `DELETE /rest/api/2/issue/{…}/worklog/{…}` | service differs, signature differs | move to `issue_worklogs()`; align parameter names, order and types |
| `CloudClient::issue_worklogs().get_ids_of_worklogs_deleted_since` | `issue_worklogs().get_ids_of_worklogs_deleted_since` | `GET /rest/api/2/worklog/deleted` | type differs | answer as `ChangedWorklogs` naming |
| `CloudClient::issue_worklogs().get_ids_of_worklogs_modified_since` | `issue_worklogs().get_ids_of_worklogs_modified_since` | `GET /rest/api/2/worklog/updated` | type differs | answer as `ChangedWorklogs` naming |
| `CloudClient::issue_worklogs().get_issue_worklog` | `issues().get_issue_worklog` | `GET /rest/api/2/issue/{…}/worklog` | service differs, type differs, stream missing, stream missing on Cloud | move to `issue_worklogs()`; answer as `PageOfWorklogs` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::issue_worklogs().get_worklog` | `issues().get_worklog` | `GET /rest/api/2/issue/{…}/worklog/{…}` | service differs | move to `issue_worklogs()` |
| `CloudClient::issue_worklogs().get_worklogs_for_ids` | `issue_worklogs().get_worklogs_for_ids` | `POST /rest/api/2/worklog/list` | identical | — |
| `CloudClient::issue_worklogs().update_worklog` | `issues().update_worklog` | `PUT /rest/api/2/issue/{…}/worklog/{…}` | service differs, signature differs | move to `issue_worklogs()`; align parameter names, order and types |
| `CloudClient::issues().assign_issue` | `issues().assign` | `PUT /rest/api/2/issue/{…}/assignee` | renamed, signature differs | rename to `assign_issue`; align parameter names, order and types |
| `CloudClient::issues().create_issue` | `issues().create_issue` | `POST /rest/api/2/issue` | signature differs, type differs | align parameter names, order and types; answer as `CreatedIssue` naming |
| `CloudClient::issues().create_issues` | `issues().create_issues` | `POST /rest/api/2/issue/bulk` | signature differs, type differs | align parameter names, order and types; answer as `CreatedIssues` naming |
| `CloudClient::issues().delete_issue` | `issues().delete_issue` | `DELETE /rest/api/2/issue/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::issues().do_transition` | `issues().do_transition` | `POST /rest/api/2/issue/{…}/transitions` | signature differs | align parameter names, order and types |
| `CloudClient::issues().edit_issue` | `issues().edit_issue` | `PUT /rest/api/2/issue/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::issues().get_create_issue_meta_issue_type_id` | `issues().get_create_issue_meta_fields` | `GET /rest/api/2/issue/createmeta/{…}/issuetypes/{…}` | renamed, signature differs, type differs, stream missing, stream missing on Cloud | rename to `get_create_issue_meta_issue_type_id`; align parameter names, order and types; answer as `PageOfCreateMetaIssueTypeWithField` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::issues().get_create_issue_meta_issue_types` | `issues().get_create_issue_meta_project_issue_types` | `GET /rest/api/2/issue/createmeta/{…}/issuetypes` | renamed, signature differs, type differs, stream missing, stream missing on Cloud | rename to `get_create_issue_meta_issue_types`; align parameter names, order and types; answer as `PageOfCreateMetaIssueTypes` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::issues().get_edit_issue_meta` | `issues().get_edit_issue_meta` | `GET /rest/api/2/issue/{…}/editmeta` | type differs | answer as `IssueUpdateMetadata` naming |
| `CloudClient::issues().get_issue` | `issues().get_issue` | `GET /rest/api/2/issue/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::issues().get_transitions` | `issues().get_transitions` | `GET /rest/api/2/issue/{…}/transitions` | type differs | answer as `Transitions` naming |
| `CloudClient::issues().notify` | `issues().notify` | `POST /rest/api/2/issue/{…}/notify` | signature differs | align parameter names, order and types |
| `CloudClient::jira_settings().get_advanced_settings` | `application_properties().get_advanced_settings` | `GET /rest/api/2/application-properties/advanced-settings` | service differs | move to `jira_settings()` |
| `CloudClient::jira_settings().get_application_property` | `application_properties().get_application_properties` | `GET /rest/api/2/application-properties` | renamed, service differs | rename to `get_application_property`; move to `jira_settings()` |
| `CloudClient::jira_settings().get_configuration` | `configuration().get_configuration` | `GET /rest/api/2/configuration` | service differs | move to `jira_settings()` |
| `CloudClient::jira_settings().set_application_property` | `application_properties().set_property_via_restful_table` | `PUT /rest/api/2/application-properties/{…}` | renamed, service differs, signature differs | rename to `set_application_property`; move to `jira_settings()`; align parameter names, order and types |
| `CloudClient::jql().get_auto_complete` | `jql().get_auto_complete` | `GET /rest/api/2/jql/autocompletedata` | type differs | answer as `JQLReferenceData` naming |
| `CloudClient::jql().get_field_auto_complete_for_query_string` | `jql().get_field_auto_complete_for_query_string` | `GET /rest/api/2/jql/autocompletedata/suggestions` | type differs | answer as `AutoCompleteSuggestions` naming |
| `CloudClient::myself().get_current_user` | `myself().get_current_user` | `GET /rest/api/2/myself` | identical | — |
| `CloudClient::myself().get_preference` | `my_preferences().get_preference` | `GET /rest/api/2/mypreferences` | service differs, signature differs | move to `myself()`; align parameter names, order and types |
| `CloudClient::myself().remove_preference` | `my_preferences().remove_preference` | `DELETE /rest/api/2/mypreferences` | service differs, signature differs | move to `myself()`; align parameter names, order and types |
| `CloudClient::myself().set_preference` | `my_preferences().set_preference` | `PUT /rest/api/2/mypreferences` | service differs, signature differs | move to `myself()`; align parameter names, order and types |
| `CloudClient::permission_schemes().create_permission_grant` | `permission_schemes().create_permission_grant` | `POST /rest/api/2/permissionscheme/{…}/permission` | signature differs | align parameter names, order and types |
| `CloudClient::permission_schemes().create_permission_scheme` | `permission_schemes().create_permission_scheme` | `POST /rest/api/2/permissionscheme` | signature differs | align parameter names, order and types |
| `CloudClient::permission_schemes().delete_permission_scheme` | `permission_schemes().delete_permission_scheme` | `DELETE /rest/api/2/permissionscheme/{…}` | identical | — |
| `CloudClient::permission_schemes().delete_permission_scheme_entity` | `permission_schemes().delete_permission_scheme_entity` | `DELETE /rest/api/2/permissionscheme/{…}/permission/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::permission_schemes().get_all_permission_schemes` | `permission_schemes().get_permission_schemes` | `GET /rest/api/2/permissionscheme` | renamed, signature differs | rename to `get_all_permission_schemes`; align parameter names, order and types |
| `CloudClient::permission_schemes().get_permission_scheme` | `permission_schemes().get_permission_scheme` | `GET /rest/api/2/permissionscheme/{…}` | identical | — |
| `CloudClient::permission_schemes().get_permission_scheme_grant` | `permission_schemes().get_permission_scheme_grant` | `GET /rest/api/2/permissionscheme/{…}/permission/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::permission_schemes().get_permission_scheme_grants` | `permission_schemes().get_permission_scheme_grants` | `GET /rest/api/2/permissionscheme/{…}/permission` | identical | — |
| `CloudClient::permission_schemes().update_permission_scheme` | `permission_schemes().update_permission_scheme` | `PUT /rest/api/2/permissionscheme/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::permissions().get_all_permissions` | `permissions().get_all_permissions` | `GET /rest/api/2/permissions` | type differs | answer as `Permissions` naming |
| `CloudClient::permissions().get_my_permissions` | `permissions().get_permissions` | `GET /rest/api/2/mypermissions` | renamed, type differs | rename to `get_my_permissions`; answer as `Permissions` naming |
| `CloudClient::project_avatars().delete_project_avatar` | `projects().delete_project_avatar` | `DELETE /rest/api/2/project/{…}/avatar/{…}` | service differs | move to `project_avatars()` |
| `CloudClient::project_avatars().get_all_project_avatars` | `projects().get_all_project_avatars` | `GET /rest/api/2/project/{…}/avatars` | service differs, type differs | move to `project_avatars()`; answer as `ProjectAvatars` naming |
| `CloudClient::project_avatars().update_project_avatar` | `projects().update_project_avatar` | `PUT /rest/api/2/project/{…}/avatar` | service differs | move to `project_avatars()` |
| `CloudClient::project_categories().create_project_category` | `project_categories().create_project_category` | `POST /rest/api/2/projectCategory` | type differs | answer as `ProjectCategory` naming |
| `CloudClient::project_categories().get_all_project_categories` | `project_categories().get_all_project_categories` | `GET /rest/api/2/projectCategory` | type differs | answer as `Vec<ProjectCategory>` naming |
| `CloudClient::project_categories().get_project_category_by_id` | `project_categories().get_project_category_by_id` | `GET /rest/api/2/projectCategory/{…}` | type differs | answer as `ProjectCategory` naming |
| `CloudClient::project_categories().remove_project_category` | `project_categories().remove_project_category` | `DELETE /rest/api/2/projectCategory/{…}` | identical | — |
| `CloudClient::project_categories().update_project_category` | `project_categories().update_project_category` | `PUT /rest/api/2/projectCategory/{…}` | type differs | answer as `UpdatedProjectCategory` naming |
| `CloudClient::project_components().create_component` | `project_components().create_component` | `POST /rest/api/2/component` | signature differs, type differs | align parameter names, order and types; answer as `ProjectComponent` naming |
| `CloudClient::project_components().delete_component` | `project_components().delete_component` | `DELETE /rest/api/2/component/{…}` | identical | — |
| `CloudClient::project_components().get_component` | `project_components().get_component` | `GET /rest/api/2/component/{…}` | type differs | answer as `ProjectComponent` naming |
| `CloudClient::project_components().get_component_related_issues` | `project_components().get_component_related_issues` | `GET /rest/api/2/component/{…}/relatedIssueCounts` | type differs | answer as `ComponentIssuesCount` naming |
| `CloudClient::project_components().get_project_components` | `projects().get_project_components` | `GET /rest/api/2/project/{…}/components` | service differs, type differs | move to `project_components()`; answer as `Vec<ProjectComponent>` naming |
| `CloudClient::project_components().update_component` | `project_components().update_component` | `PUT /rest/api/2/component/{…}` | signature differs, type differs | align parameter names, order and types; answer as `ProjectComponent` naming |
| `CloudClient::project_key_and_name_validation().validate_project_key` | `project_key_and_name_validation().validate_project_key` | `GET /rest/api/2/projectvalidate/key` | identical | — |
| `CloudClient::project_permission_schemes().assign_permission_scheme` | `projects().assign_permission_scheme` | `PUT /rest/api/2/project/{…}/permissionscheme` | service differs | move to `project_permission_schemes()` |
| `CloudClient::project_permission_schemes().get_assigned_permission_scheme` | `projects().get_assigned_permission_scheme` | `GET /rest/api/2/project/{…}/permissionscheme` | service differs | move to `project_permission_schemes()` |
| `CloudClient::project_permission_schemes().get_project_issue_security_scheme` | `projects().get_project_issue_security_scheme` | `GET /rest/api/2/project/{…}/issuesecuritylevelscheme` | service differs, type differs | move to `project_permission_schemes()`; answer as `SecurityScheme` naming |
| `CloudClient::project_permission_schemes().get_security_levels_for_project` | `projects().get_security_levels_for_project` | `GET /rest/api/2/project/{…}/securitylevel` | service differs, type differs | move to `project_permission_schemes()`; answer as `ProjectIssueSecurityLevels` naming |
| `CloudClient::project_properties().delete_project_property` | `projects().delete_project_property` | `DELETE /rest/api/2/project/{…}/properties/{…}` | service differs, signature differs | move to `project_properties()`; align parameter names, order and types |
| `CloudClient::project_properties().get_project_property` | `projects().get_project_property` | `GET /rest/api/2/project/{…}/properties/{…}` | service differs, signature differs | move to `project_properties()`; align parameter names, order and types |
| `CloudClient::project_properties().get_project_property_keys` | `projects().get_project_property_keys` | `GET /rest/api/2/project/{…}/properties` | service differs, type differs | move to `project_properties()`; answer as `PropertyKeys` naming |
| `CloudClient::project_properties().set_project_property` | `projects().set_project_property` | `PUT /rest/api/2/project/{…}/properties/{…}` | service differs, signature differs | move to `project_properties()`; align parameter names, order and types |
| `CloudClient::project_role_actors().add_actor_users` | `projects().add_actor_users` | `POST /rest/api/2/project/{…}/role/{…}` | service differs | move to `project_role_actors()` |
| `CloudClient::project_role_actors().add_project_role_actors_to_role` | `project_roles().add_project_role_actors_to_role` | `POST /rest/api/2/role/{…}/actors` | service differs, signature differs, type differs | move to `project_role_actors()`; align parameter names, order and types; answer as `ProjectRole` naming |
| `CloudClient::project_role_actors().delete_actor` | `projects().delete_actor` | `DELETE /rest/api/2/project/{…}/role/{…}` | service differs | move to `project_role_actors()` |
| `CloudClient::project_role_actors().delete_project_role_actors_from_role` | `project_roles().delete_project_role_actors_from_role` | `DELETE /rest/api/2/role/{…}/actors` | service differs, type differs | move to `project_role_actors()`; answer as `ProjectRole` naming |
| `CloudClient::project_role_actors().get_project_role_actors_for_role` | `project_roles().get_project_role_actors_for_role` | `GET /rest/api/2/role/{…}/actors` | service differs, type differs | move to `project_role_actors()`; answer as `ProjectRole` naming |
| `CloudClient::project_role_actors().set_actors` | `projects().set_actors` | `PUT /rest/api/2/project/{…}/role/{…}` | service differs | move to `project_role_actors()` |
| `CloudClient::project_roles().create_project_role` | `project_roles().create_project_role` | `POST /rest/api/2/role` | identical | — |
| `CloudClient::project_roles().delete_project_role` | `project_roles().delete_project_role` | `DELETE /rest/api/2/role/{…}` | identical | — |
| `CloudClient::project_roles().fully_update_project_role` | `project_roles().fully_update_project_role` | `PUT /rest/api/2/role/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::project_roles().get_all_project_roles` | `project_roles().get_all_project_roles` | `GET /rest/api/2/role` | identical | — |
| `CloudClient::project_roles().get_project_role` | `projects().get_project_role` | `GET /rest/api/2/project/{…}/role/{…}` | service differs | move to `project_roles()` |
| `CloudClient::project_roles().get_project_role_by_id` | `project_roles().get_project_roles_by_id` | `GET /rest/api/2/role/{…}` | renamed | rename to `get_project_role_by_id` |
| `CloudClient::project_roles().get_project_roles` | `projects().get_project_roles` | `GET /rest/api/2/project/{…}/role` | service differs | move to `project_roles()` |
| `CloudClient::project_roles().partial_update_project_role` | `project_roles().partial_update_project_role` | `POST /rest/api/2/role/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::project_types().get_accessible_project_type_by_key` | `projects().get_accessible_project_type_by_key` | `GET /rest/api/2/project/type/{…}/accessible` | service differs, signature differs | move to `project_types()`; align parameter names, order and types |
| `CloudClient::project_types().get_all_project_types` | `projects().get_all_project_types` | `GET /rest/api/2/project/type` | service differs | move to `project_types()` |
| `CloudClient::project_types().get_project_type_by_key` | `projects().get_project_type_by_key` | `GET /rest/api/2/project/type/{…}` | service differs, signature differs | move to `project_types()`; align parameter names, order and types |
| `CloudClient::project_versions().create_version` | `project_versions().create_version` | `POST /rest/api/2/version` | identical | — |
| `CloudClient::project_versions().delete_and_replace_version` | `project_versions().delete_version_and_swap` | `POST /rest/api/2/version/{…}/removeAndSwap` | renamed | rename to `delete_and_replace_version` |
| `CloudClient::project_versions().get_project_versions` | `projects().get_project_versions` | `GET /rest/api/2/project/{…}/versions` | service differs, signature differs | move to `project_versions()`; align parameter names, order and types |
| `CloudClient::project_versions().get_project_versions_paginated` | `projects().get_project_versions_paginated` | `GET /rest/api/2/project/{…}/version` | service differs, signature differs, type differs, stream missing | move to `project_versions()`; align parameter names, order and types; answer as `Page<Version>` naming; add `.stream()` |
| `CloudClient::project_versions().get_version` | `project_versions().get_version` | `GET /rest/api/2/version/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::project_versions().get_version_related_issues` | `project_versions().get_version_related_issues` | `GET /rest/api/2/version/{…}/relatedIssueCounts` | identical | — |
| `CloudClient::project_versions().get_version_unresolved_issues` | `project_versions().get_version_unresolved_issues` | `GET /rest/api/2/version/{…}/unresolvedIssueCount` | type differs | answer as `VersionUnresolvedIssuesCount` naming |
| `CloudClient::project_versions().merge_versions` | `project_versions().merge` | `PUT /rest/api/2/version/{…}/mergeto/{…}` | renamed, signature differs | rename to `merge_versions`; align parameter names, order and types |
| `CloudClient::project_versions().move_version` | `project_versions().move_version` | `POST /rest/api/2/version/{…}/move` | identical | — |
| `CloudClient::project_versions().update_version` | `project_versions().update_version` | `PUT /rest/api/2/version/{…}` | type differs | answer as `Version` naming |
| `CloudClient::projects().create_project` | `projects().create_project` | `POST /rest/api/2/project` | signature differs, type differs | align parameter names, order and types; answer as `ProjectIdentifiers` naming |
| `CloudClient::projects().delete_project` | `projects().delete_project` | `DELETE /rest/api/2/project/{…}` | identical | — |
| `CloudClient::projects().get_all_statuses` | `projects().get_all_statuses` | `GET /rest/api/2/project/{…}/statuses` | type differs | answer as `Vec<IssueTypeWithStatus>` naming |
| `CloudClient::projects().get_notification_scheme_for_project` | `projects().get_project_notification_scheme` | `GET /rest/api/2/project/{…}/notificationscheme` | renamed, signature differs | rename to `get_notification_scheme_for_project`; align parameter names, order and types |
| `CloudClient::projects().get_project` | `projects().get_project` | `GET /rest/api/2/project/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::projects().search_projects` | `projects().search_for_projects` | `GET /rest/api/2/projects/picker` | renamed, type differs, stream missing | rename to `search_projects`; answer as `Page<Project>` naming; add `.stream()` |
| `CloudClient::projects().update_project` | `projects().update_project` | `PUT /rest/api/2/project/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::screen_tab_fields().add_screen_tab_field` | `screens().add_field` | `POST /rest/api/2/screens/{…}/tabs/{…}/fields` | renamed, service differs, signature differs | rename to `add_screen_tab_field`; move to `screen_tab_fields()`; align parameter names, order and types |
| `CloudClient::screen_tab_fields().get_all_screen_tab_fields` | `screens().get_all_fields` | `GET /rest/api/2/screens/{…}/tabs/{…}/fields` | renamed, service differs, signature differs | rename to `get_all_screen_tab_fields`; move to `screen_tab_fields()`; align parameter names, order and types |
| `CloudClient::screen_tab_fields().move_screen_tab_field` | `screens().move_field` | `POST /rest/api/2/screens/{…}/tabs/{…}/fields/{…}/move` | renamed, service differs, signature differs | rename to `move_screen_tab_field`; move to `screen_tab_fields()`; align parameter names, order and types |
| `CloudClient::screen_tab_fields().remove_screen_tab_field` | `screens().remove_field` | `DELETE /rest/api/2/screens/{…}/tabs/{…}/fields/{…}` | renamed, service differs, signature differs | rename to `remove_screen_tab_field`; move to `screen_tab_fields()`; align parameter names, order and types |
| `CloudClient::screen_tabs().add_screen_tab` | `screens().add_tab` | `POST /rest/api/2/screens/{…}/tabs` | renamed, service differs, signature differs | rename to `add_screen_tab`; move to `screen_tabs()`; align parameter names, order and types |
| `CloudClient::screen_tabs().delete_screen_tab` | `screens().delete_tab` | `DELETE /rest/api/2/screens/{…}/tabs/{…}` | renamed, service differs, signature differs | rename to `delete_screen_tab`; move to `screen_tabs()`; align parameter names, order and types |
| `CloudClient::screen_tabs().get_all_screen_tabs` | `screens().get_all_tabs` | `GET /rest/api/2/screens/{…}/tabs` | renamed, service differs | rename to `get_all_screen_tabs`; move to `screen_tabs()` |
| `CloudClient::screen_tabs().move_screen_tab` | `screens().move_tab` | `POST /rest/api/2/screens/{…}/tabs/{…}/move/{…}` | renamed, service differs, signature differs | rename to `move_screen_tab`; move to `screen_tabs()`; align parameter names, order and types |
| `CloudClient::screen_tabs().rename_screen_tab` | `screens().rename_tab` | `PUT /rest/api/2/screens/{…}/tabs/{…}` | renamed, service differs, signature differs | rename to `rename_screen_tab`; move to `screen_tabs()`; align parameter names, order and types |
| `CloudClient::screens().add_field_to_default_screen` | `screens().add_field_to_default_screen` | `POST /rest/api/2/screens/addToDefault/{…}` | identical | — |
| `CloudClient::screens().get_available_screen_fields` | `screens().get_fields_to_add` | `GET /rest/api/2/screens/{…}/availableFields` | renamed | rename to `get_available_screen_fields` |
| `CloudClient::screens().get_screens` | `screens().get_all_screens` | `GET /rest/api/2/screens` | renamed, signature differs, type differs, stream missing | rename to `get_screens`; align parameter names, order and types; answer as `Page<Screen>` naming; add `.stream()` |
| `CloudClient::server_info().get_server_info` | `server_info().get_server_info` | `GET /rest/api/2/serverInfo` | type differs | answer as `ServerInformation` naming |
| `CloudClient::user_properties().delete_user_property` | `users().delete_user_property` | `DELETE /rest/api/2/user/properties/{…}` | service differs | move to `user_properties()` |
| `CloudClient::user_properties().get_user_property` | `users().get_user_property` | `GET /rest/api/2/user/properties/{…}` | service differs | move to `user_properties()` |
| `CloudClient::user_properties().get_user_property_keys` | `users().get_user_property_keys` | `GET /rest/api/2/user/properties` | service differs, type differs | move to `user_properties()`; answer as `PropertyKeys` naming |
| `CloudClient::user_properties().set_user_property` | `users().set_user_property` | `PUT /rest/api/2/user/properties/{…}` | service differs | move to `user_properties()` |
| `CloudClient::user_search().find_assignable_users` | `users().find_assignable_users` | `GET /rest/api/2/user/assignable/search` | service differs, type differs, stream missing, stream missing on Cloud | move to `user_search()`; answer as `Vec<User>` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::user_search().find_bulk_assignable_users` | `users().find_bulk_assignable_users` | `GET /rest/api/2/user/assignable/multiProjectSearch` | service differs, signature differs, type differs, stream missing, stream missing on Cloud | move to `user_search()`; align parameter names, order and types; answer as `Vec<User>` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::user_search().find_users` | `users().find_users` | `GET /rest/api/2/user/search` | service differs, type differs, stream missing, stream missing on Cloud | move to `user_search()`; answer as `Vec<User>` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::user_search().find_users_for_picker` | `users().find_users_for_picker` | `GET /rest/api/2/user/picker` | service differs, signature differs, type differs | move to `user_search()`; align parameter names, order and types; answer as `FoundUsers` naming |
| `CloudClient::user_search().find_users_with_browse_permission` | `users().find_users_with_browse_permission` | `GET /rest/api/2/user/viewissue/search` | service differs, type differs, stream missing, stream missing on Cloud | move to `user_search()`; answer as `Vec<User>` naming; add `.stream()`; add Cloud `.stream()` |
| `CloudClient::users().create_user` | `users().create_user` | `POST /rest/api/2/user` | signature differs, type differs | align parameter names, order and types; answer as `User` naming |
| `CloudClient::users().get_user` | `users().get_user` | `GET /rest/api/2/user` | signature differs | align parameter names, order and types |
| `CloudClient::users().get_user_default_columns` | `users().default_columns` | `GET /rest/api/2/user/columns` | renamed, type differs | rename to `get_user_default_columns`; answer as `Vec<ColumnItem>` naming |
| `CloudClient::users().remove_user` | `users().remove_user` | `DELETE /rest/api/2/user` | signature differs | align parameter names, order and types |
| `CloudClient::users().reset_user_columns` | `users().reset_user_columns` | `DELETE /rest/api/2/user/columns` | identical | — |
| `CloudClient::users().set_user_columns` | `users().set_columns_url_encoded` | `PUT /rest/api/2/user/columns` | renamed, signature differs | rename to `set_user_columns`; align parameter names, order and types |
| `CloudClient::workflow_scheme_drafts().create_workflow_scheme_draft_from_parent` | `workflow_schemes().create_draft_for_parent` | `POST /rest/api/2/workflowscheme/{…}/createdraft` | renamed, service differs | rename to `create_workflow_scheme_draft_from_parent`; move to `workflow_scheme_drafts()` |
| `CloudClient::workflow_scheme_drafts().delete_draft_default_workflow` | `workflow_schemes().delete_draft_default` | `DELETE /rest/api/2/workflowscheme/{…}/draft/default` | renamed, service differs | rename to `delete_draft_default_workflow`; move to `workflow_scheme_drafts()` |
| `CloudClient::workflow_scheme_drafts().delete_draft_workflow_mapping` | `workflow_schemes().delete_draft_workflow_mapping` | `DELETE /rest/api/2/workflowscheme/{…}/draft/workflow` | service differs, signature differs, type differs | move to `workflow_scheme_drafts()`; align parameter names, order and types; align response type name |
| `CloudClient::workflow_scheme_drafts().delete_workflow_scheme_draft` | `workflow_schemes().delete_draft_by_id` | `DELETE /rest/api/2/workflowscheme/{…}/draft` | renamed, service differs | rename to `delete_workflow_scheme_draft`; move to `workflow_scheme_drafts()` |
| `CloudClient::workflow_scheme_drafts().delete_workflow_scheme_draft_issue_type` | `workflow_schemes().delete_draft_issue_type` | `DELETE /rest/api/2/workflowscheme/{…}/draft/issuetype/{…}` | renamed, service differs, signature differs | rename to `delete_workflow_scheme_draft_issue_type`; move to `workflow_scheme_drafts()`; align parameter names, order and types |
| `CloudClient::workflow_scheme_drafts().get_draft_default_workflow` | `workflow_schemes().get_draft_default` | `GET /rest/api/2/workflowscheme/{…}/draft/default` | renamed, service differs, type differs | rename to `get_draft_default_workflow`; move to `workflow_scheme_drafts()`; answer as `DefaultWorkflow` naming |
| `CloudClient::workflow_scheme_drafts().get_draft_workflow` | `workflow_schemes().get_draft_workflow` | `GET /rest/api/2/workflowscheme/{…}/draft/workflow` | service differs, type differs | move to `workflow_scheme_drafts()`; answer as `IssueTypesWorkflowMapping` naming |
| `CloudClient::workflow_scheme_drafts().get_workflow_scheme_draft` | `workflow_schemes().get_draft_by_id` | `GET /rest/api/2/workflowscheme/{…}/draft` | renamed, service differs | rename to `get_workflow_scheme_draft`; move to `workflow_scheme_drafts()` |
| `CloudClient::workflow_scheme_drafts().get_workflow_scheme_draft_issue_type` | `workflow_schemes().get_draft_issue_type` | `GET /rest/api/2/workflowscheme/{…}/draft/issuetype/{…}` | renamed, service differs, signature differs, type differs | rename to `get_workflow_scheme_draft_issue_type`; move to `workflow_scheme_drafts()`; align parameter names, order and types; answer as `IssueTypeWorkflowMapping` naming |
| `CloudClient::workflow_scheme_drafts().set_workflow_scheme_draft_issue_type` | `workflow_schemes().set_draft_issue_type` | `PUT /rest/api/2/workflowscheme/{…}/draft/issuetype/{…}` | renamed, service differs, signature differs | rename to `set_workflow_scheme_draft_issue_type`; move to `workflow_scheme_drafts()`; align parameter names, order and types |
| `CloudClient::workflow_scheme_drafts().update_draft_default_workflow` | `workflow_schemes().update_draft_default` | `PUT /rest/api/2/workflowscheme/{…}/draft/default` | renamed, service differs, signature differs | rename to `update_draft_default_workflow`; move to `workflow_scheme_drafts()`; align parameter names, order and types |
| `CloudClient::workflow_scheme_drafts().update_draft_workflow_mapping` | `workflow_schemes().update_draft_workflow_mapping` | `PUT /rest/api/2/workflowscheme/{…}/draft/workflow` | service differs, signature differs | move to `workflow_scheme_drafts()`; align parameter names, order and types |
| `CloudClient::workflow_scheme_drafts().update_workflow_scheme_draft` | `workflow_schemes().update_draft` | `PUT /rest/api/2/workflowscheme/{…}/draft` | renamed, service differs | rename to `update_workflow_scheme_draft`; move to `workflow_scheme_drafts()` |
| `CloudClient::workflow_schemes().create_workflow_scheme` | `workflow_schemes().create_scheme` | `POST /rest/api/2/workflowscheme` | renamed | rename to `create_workflow_scheme` |
| `CloudClient::workflow_schemes().delete_default_workflow` | `workflow_schemes().delete_default` | `DELETE /rest/api/2/workflowscheme/{…}/default` | renamed | rename to `delete_default_workflow` |
| `CloudClient::workflow_schemes().delete_workflow_mapping` | `workflow_schemes().delete_workflow_mapping` | `DELETE /rest/api/2/workflowscheme/{…}/workflow` | signature differs, type differs | align parameter names, order and types; align response type name |
| `CloudClient::workflow_schemes().delete_workflow_scheme` | `workflow_schemes().delete_scheme` | `DELETE /rest/api/2/workflowscheme/{…}` | renamed | rename to `delete_workflow_scheme` |
| `CloudClient::workflow_schemes().delete_workflow_scheme_issue_type` | `workflow_schemes().delete_workflow_scheme_issue_type` | `DELETE /rest/api/2/workflowscheme/{…}/issuetype/{…}` | signature differs | align parameter names, order and types |
| `CloudClient::workflow_schemes().get_default_workflow` | `workflow_schemes().get_default` | `GET /rest/api/2/workflowscheme/{…}/default` | renamed, type differs | rename to `get_default_workflow`; answer as `DefaultWorkflow` naming |
| `CloudClient::workflow_schemes().get_workflow` | `workflow_schemes().get_workflow` | `GET /rest/api/2/workflowscheme/{…}/workflow` | type differs | answer as `IssueTypesWorkflowMapping` naming |
| `CloudClient::workflow_schemes().get_workflow_scheme` | `workflow_schemes().get_by_id` | `GET /rest/api/2/workflowscheme/{…}` | renamed | rename to `get_workflow_scheme` |
| `CloudClient::workflow_schemes().get_workflow_scheme_issue_type` | `workflow_schemes().get_workflow_scheme_issue_type` | `GET /rest/api/2/workflowscheme/{…}/issuetype/{…}` | signature differs, type differs | align parameter names, order and types; answer as `IssueTypeWorkflowMapping` naming |
| `CloudClient::workflow_schemes().set_workflow_scheme_issue_type` | `workflow_schemes().set_issue_type` | `PUT /rest/api/2/workflowscheme/{…}/issuetype/{…}` | renamed, signature differs | rename to `set_workflow_scheme_issue_type`; align parameter names, order and types |
| `CloudClient::workflow_schemes().update_default_workflow` | `workflow_schemes().update_default` | `PUT /rest/api/2/workflowscheme/{…}/default` | renamed, signature differs | rename to `update_default_workflow`; align parameter names, order and types |
| `CloudClient::workflow_schemes().update_workflow_mapping` | `workflow_schemes().update_workflow_mapping` | `PUT /rest/api/2/workflowscheme/{…}/workflow` | signature differs | align parameter names, order and types |
| `CloudClient::workflow_schemes().update_workflow_scheme` | `workflow_schemes().update_workflow_scheme` | `PUT /rest/api/2/workflowscheme/{…}` | identical | — |
| `CloudClient::workflow_status_categories().get_status_categories` | `workflow_status_categories().get_status_categories` | `GET /rest/api/2/statuscategory` | type differs | answer as `Vec<StatusCategory>` naming |
| `CloudClient::workflow_status_categories().get_status_category` | `workflow_status_categories().get_status_category` | `GET /rest/api/2/statuscategory/{…}` | type differs | answer as `StatusCategory` naming |
| `CloudClient::workflow_statuses().get_status` | `workflow_statuses().get_status` | `GET /rest/api/2/status/{…}` | type differs | answer as `StatusDetails` naming |
| `CloudClient::workflow_statuses().get_statuses` | `workflow_statuses().get_statuses` | `GET /rest/api/2/status` | type differs | answer as `Vec<StatusDetails>` naming |

## Server/Data Center only

| Server/DC | Endpoint | Target |
|---|---|---|
| `application_roles().put_bulk` | `PUT /rest/api/2/applicationrole` | stays Server/DC-only |
| `application_roles().update_application_role` | `PUT /rest/api/2/applicationrole/{…}` | stays Server/DC-only |
| `avatars().create_avatar_from_temporary` | `POST /rest/api/2/universal_avatar/type/{…}/owner/{…}/avatar` | stays Server/DC-only |
| `avatars().store_temporary_avatar_using_multi_part` | `POST /rest/api/2/universal_avatar/type/{…}/owner/{…}/temp` | stays Server/DC-only |
| `board().get_refined_velocity` | `GET /rest/agile/1.0/board/{…}/settings/refined-velocity` | stays Server/DC-only |
| `board().set_refined_velocity` | `PUT /rest/agile/1.0/board/{…}/settings/refined-velocity` | stays Server/DC-only |
| `cluster().acknowledge_errors` | `POST /rest/api/2/cluster/zdu/retryUpgrade` | stays Server/DC-only |
| `cluster().approve_upgrade` | `POST /rest/api/2/cluster/zdu/approve` | stays Server/DC-only |
| `cluster().cancel_upgrade` | `POST /rest/api/2/cluster/zdu/cancel` | stays Server/DC-only |
| `cluster().change_node_state_to_offline` | `PUT /rest/api/2/cluster/node/{…}/offline` | stays Server/DC-only |
| `cluster().delete_node` | `DELETE /rest/api/2/cluster/node/{…}` | stays Server/DC-only |
| `cluster().get_all_nodes` | `GET /rest/api/2/cluster/nodes` | stays Server/DC-only |
| `cluster().get_state` | `GET /rest/api/2/cluster/zdu/state` | stays Server/DC-only |
| `cluster().set_ready_to_upgrade` | `POST /rest/api/2/cluster/zdu/start` | stays Server/DC-only |
| `email_templates().apply_email_templates` | `POST /rest/api/2/email-templates/apply` | stays Server/DC-only |
| `email_templates().download_email_templates` | `GET /rest/api/2/email-templates` | stays Server/DC-only |
| `email_templates().get_email_types` | `GET /rest/api/2/email-templates/types` | stays Server/DC-only |
| `email_templates().revert_email_templates_to_default` | `POST /rest/api/2/email-templates/revert` | stays Server/DC-only |
| `email_templates().upload_email_templates` | `POST /rest/api/2/email-templates` | stays Server/DC-only |
| `indexing().create_index_snapshot` | `POST /rest/api/2/index-snapshot` | stays Server/DC-only |
| `indexing().get_index_summary` | `GET /rest/api/2/index/summary` | stays Server/DC-only |
| `indexing().get_progress_bulk` | `GET /rest/api/2/reindex/request/bulk` | stays Server/DC-only |
| `indexing().get_reindex_info` | `GET /rest/api/2/reindex` | stays Server/DC-only |
| `indexing().get_reindex_progress` | `GET /rest/api/2/reindex/progress` | stays Server/DC-only |
| `indexing().get_reindex_request_progress` | `GET /rest/api/2/reindex/request/{…}` | stays Server/DC-only |
| `indexing().is_index_snapshot_running` | `GET /rest/api/2/index-snapshot/isRunning` | stays Server/DC-only |
| `indexing().list_index_snapshot` | `GET /rest/api/2/index-snapshot` | stays Server/DC-only |
| `indexing().process_requests` | `POST /rest/api/2/reindex/request` | stays Server/DC-only |
| `indexing().reindex` | `POST /rest/api/2/reindex` | stays Server/DC-only |
| `indexing().reindex_issues` | `POST /rest/api/2/reindex/issue` | stays Server/DC-only |
| `issue_attachments().expand_for_humans` | `GET /rest/api/2/attachment/{…}/expand/human` | stays Server/DC-only |
| `issue_attachments().expand_for_machines` | `GET /rest/api/2/attachment/{…}/expand/raw` | stays Server/DC-only |
| `issue_fields().bulk_delete_custom_fields` | `DELETE /rest/api/2/customFields` | stays Server/DC-only |
| `issue_fields().get_custom_field_options` | `GET /rest/api/2/customFields/{…}/options` | stays Server/DC-only; add `.stream()` |
| `issue_fields().get_custom_fields` | `GET /rest/api/2/customFields` | stays Server/DC-only; add `.stream()` |
| `issue_link_types().move_issue_link_type` | `PUT /rest/api/2/issueLinkType/{…}/order` | stays Server/DC-only |
| `issue_link_types().reset_order` | `PUT /rest/api/2/issueLinkType/order` | stays Server/DC-only |
| `issue_priorities().get_priorities` | `GET /rest/api/2/priority` | stays Server/DC-only |
| `issue_priorities().get_priorities_paginated` | `GET /rest/api/2/priority/page` | stays Server/DC-only |
| `issue_resolutions().get_paginated_resolutions` | `GET /rest/api/2/resolution/page` | stays Server/DC-only |
| `issue_resolutions().get_resolutions` | `GET /rest/api/2/resolution` | stays Server/DC-only |
| `issue_search().get_error` | `GET /rest/api/2/search/error/lookup` | stays Server/DC-only |
| `issue_search().search_using_search_request` | `POST /rest/api/2/search` | stays Server/DC-only; add `.stream()` |
| `issue_type_schemes().add_project_associations_to_scheme` | `POST /rest/api/2/issuetypescheme/{…}/associations` | stays Server/DC-only |
| `issue_type_schemes().get_associated_projects` | `GET /rest/api/2/issuetypescheme/{…}/associations` | stays Server/DC-only |
| `issue_type_schemes().get_issue_type_scheme` | `GET /rest/api/2/issuetypescheme/{…}` | stays Server/DC-only |
| `issue_type_schemes().remove_all_project_associations` | `DELETE /rest/api/2/issuetypescheme/{…}/associations` | stays Server/DC-only |
| `issue_type_schemes().remove_project_association` | `DELETE /rest/api/2/issuetypescheme/{…}/associations/{…}` | stays Server/DC-only |
| `issue_type_schemes().set_project_associations_for_scheme` | `PUT /rest/api/2/issuetypescheme/{…}/associations` | stays Server/DC-only |
| `issue_types().create_issue_type_avatar_from_temporary` | `POST /rest/api/2/issuetype/{…}/avatar` | stays Server/DC-only |
| `issue_types().get_paginated_issue_types` | `GET /rest/api/2/issuetype/page` | stays Server/DC-only |
| `issue_types().store_temporary_issue_type_avatar_using_multi_part` | `POST /rest/api/2/issuetype/{…}/avatar/temporary` | stays Server/DC-only |
| `issues().archive_issue` | `PUT /rest/api/2/issue/{…}/archive` | stays Server/DC-only |
| `issues().archive_issues` | `POST /rest/api/2/issue/archive` | stays Server/DC-only |
| `issues().can_move_sub_task` | `GET /rest/api/2/issue/{…}/subtask/move` | stays Server/DC-only |
| `issues().create_reciprocal_remote_issue_link` | `POST /rest/api/2/issue/remotelink/reciprocal` | stays Server/DC-only |
| `issues().get_pinned_comments` | `GET /rest/api/2/issue/{…}/pinned-comments` | stays Server/DC-only |
| `issues().get_sub_tasks` | `GET /rest/api/2/issue/{…}/subtask` | stays Server/DC-only |
| `issues().move_sub_tasks` | `POST /rest/api/2/issue/{…}/subtask/move` | stays Server/DC-only |
| `issues().restore_issue` | `PUT /rest/api/2/issue/{…}/restore` | stays Server/DC-only |
| `issues().set_pin_comment` | `PUT /rest/api/2/issue/{…}/comment/{…}/pin` | stays Server/DC-only |
| `jira_settings().set_base_url` | `PUT /rest/api/2/settings/baseUrl` | stays Server/DC-only |
| `license_validator().validate` | `POST /rest/api/2/licenseValidator` | stays Server/DC-only |
| `monitoring().are_metrics_exposed` | `GET /rest/api/2/monitoring/jmx/areMetricsExposed` | stays Server/DC-only |
| `monitoring().get_available_metrics` | `GET /rest/api/2/monitoring/jmx/getAvailableMetrics` | stays Server/DC-only |
| `monitoring().is_app_monitoring_enabled` | `GET /rest/api/2/monitoring/app` | stays Server/DC-only |
| `monitoring().is_ipd_monitoring_enabled` | `GET /rest/api/2/monitoring/ipd` | stays Server/DC-only |
| `monitoring().set_app_monitoring_enabled` | `POST /rest/api/2/monitoring/app` | stays Server/DC-only |
| `monitoring().set_ipd_monitoring_enabled` | `POST /rest/api/2/monitoring/ipd` | stays Server/DC-only |
| `monitoring().start` | `POST /rest/api/2/monitoring/jmx/startExposing` | stays Server/DC-only |
| `monitoring().stop` | `POST /rest/api/2/monitoring/jmx/stopExposing` | stays Server/DC-only |
| `myself().change_my_password` | `PUT /rest/api/2/myself/password` | stays Server/DC-only |
| `myself().update_current_user` | `PUT /rest/api/2/myself` | stays Server/DC-only |
| `password().get_password_policy` | `GET /rest/api/2/password/policy` | stays Server/DC-only |
| `password().policy_check_create_user` | `POST /rest/api/2/password/policy/createUser` | stays Server/DC-only |
| `password().policy_check_update_user` | `POST /rest/api/2/password/policy/updateUser` | stays Server/DC-only |
| `permission_schemes().get_scheme_attribute` | `GET /rest/api/2/permissionscheme/{…}/attribute/{…}` | stays Server/DC-only |
| `permission_schemes().set_scheme_attribute` | `PUT /rest/api/2/permissionscheme/{…}/attribute/{…}` | stays Server/DC-only |
| `priority_schemes().create_priority_scheme` | `POST /rest/api/2/priorityschemes` | stays Server/DC-only |
| `priority_schemes().delete_priority_scheme` | `DELETE /rest/api/2/priorityschemes/{…}` | stays Server/DC-only |
| `priority_schemes().get_priority_scheme` | `GET /rest/api/2/priorityschemes/{…}` | stays Server/DC-only |
| `priority_schemes().get_priority_schemes` | `GET /rest/api/2/priorityschemes` | stays Server/DC-only; add `.stream()` |
| `priority_schemes().update_priority_scheme` | `PUT /rest/api/2/priorityschemes/{…}` | stays Server/DC-only |
| `project_components().get_paginated_components` | `GET /rest/api/2/component/page` | stays Server/DC-only; add `.stream()` |
| `project_versions().create_or_update_remote_version_link` | `POST /rest/api/2/version/{…}/remotelink` | stays Server/DC-only |
| `project_versions().create_or_update_remote_version_link_by_global_id` | `POST /rest/api/2/version/{…}/remotelink/{…}` | stays Server/DC-only |
| `project_versions().delete_remote_version_link` | `DELETE /rest/api/2/version/{…}/remotelink/{…}` | stays Server/DC-only |
| `project_versions().delete_remote_version_links_by_version_id` | `DELETE /rest/api/2/version/{…}/remotelink` | stays Server/DC-only |
| `project_versions().get_paginated_versions` | `GET /rest/api/2/version` | stays Server/DC-only |
| `project_versions().get_remote_version_link` | `GET /rest/api/2/version/{…}/remotelink/{…}` | stays Server/DC-only |
| `project_versions().get_remote_version_links` | `GET /rest/api/2/version/remotelink` | stays Server/DC-only |
| `project_versions().get_remote_version_links_by_version_id` | `GET /rest/api/2/version/{…}/remotelink` | stays Server/DC-only |
| `projects().archive_project` | `PUT /rest/api/2/project/{…}/archive` | stays Server/DC-only |
| `projects().assign_priority_scheme` | `PUT /rest/api/2/project/{…}/priorityscheme` | stays Server/DC-only |
| `projects().create_project_avatar_from_temporary` | `POST /rest/api/2/project/{…}/avatar` | stays Server/DC-only |
| `projects().get_all_projects` | `GET /rest/api/2/project` | stays Server/DC-only |
| `projects().get_assigned_priority_scheme` | `GET /rest/api/2/project/{…}/priorityscheme` | stays Server/DC-only |
| `projects().get_workflow_scheme_for_project` | `GET /rest/api/2/project/{…}/workflowscheme` | stays Server/DC-only |
| `projects().restore_project` | `PUT /rest/api/2/project/{…}/restore` | stays Server/DC-only |
| `projects().store_temporary_project_avatar_using_multi_part` | `POST /rest/api/2/project/{…}/avatar/temporary` | stays Server/DC-only |
| `projects().unassign_priority_scheme` | `DELETE /rest/api/2/project/{…}/priorityscheme/{…}` | stays Server/DC-only |
| `projects().update_project_type` | `PUT /rest/api/2/project/{…}/type/{…}` | stays Server/DC-only |
| `read_only_mode().get_read_only_mode` | `GET /rest/api/2/readonly-mode` | stays Server/DC-only |
| `read_only_mode().update_read_only_mode` | `PUT /rest/api/2/readonly-mode` | stays Server/DC-only |
| `screens().update_show_when_empty_indicator` | `PUT /rest/api/2/screens/{…}/tabs/{…}/fields/{…}/updateShowWhenEmptyIndicator/{…}` | stays Server/DC-only |
| `search_limits().get_max_aggregation_buckets` | `GET /rest/api/2/searchLimits/maxAggregationBuckets` | stays Server/DC-only |
| `search_limits().get_max_result_window` | `GET /rest/api/2/searchLimits/maxResultWindow` | stays Server/DC-only |
| `session().current_user` | `GET /rest/auth/1/session` | stays Server/DC-only |
| `session().login` | `POST /rest/auth/1/session` | stays Server/DC-only |
| `session().logout` | `DELETE /rest/auth/1/session` | stays Server/DC-only |
| `sprint().unmap_all_sprints` | `PUT /rest/agile/1.0/sprint/unmap-all` | stays Server/DC-only |
| `sprint().unmap_sprints` | `PUT /rest/agile/1.0/sprint/unmap` | stays Server/DC-only |
| `terminology().get_all_terminology_entries` | `GET /rest/api/2/terminology/entries` | stays Server/DC-only |
| `terminology().get_terminology_entry` | `GET /rest/api/2/terminology/entries/{…}` | stays Server/DC-only |
| `terminology().set_terminology_entries` | `POST /rest/api/2/terminology/entries` | stays Server/DC-only |
| `upgrade().get_upgrade_result` | `GET /rest/api/2/upgrade` | stays Server/DC-only |
| `upgrade().run_upgrades_now` | `POST /rest/api/2/upgrade` | stays Server/DC-only |
| `users().add_user_to_application` | `POST /rest/api/2/user/application` | stays Server/DC-only |
| `users().change_user_password` | `PUT /rest/api/2/user/password` | stays Server/DC-only |
| `users().create_user_avatar_from_temporary` | `POST /rest/api/2/user/avatar` | stays Server/DC-only |
| `users().delete_session` | `DELETE /rest/api/2/user/session/{…}` | stays Server/DC-only |
| `users().delete_user_avatar` | `DELETE /rest/api/2/user/avatar/{…}` | stays Server/DC-only |
| `users().get_a11y_personal_settings` | `GET /rest/api/2/user/a11y/personal-settings` | stays Server/DC-only |
| `users().get_all_user_avatars` | `GET /rest/api/2/user/avatars` | stays Server/DC-only |
| `users().get_duplicated_users_count` | `GET /rest/api/2/user/duplicated/count` | stays Server/DC-only |
| `users().get_duplicated_users_mapping` | `GET /rest/api/2/user/duplicated/list` | stays Server/DC-only |
| `users().get_user_anonymization_progress` | `GET /rest/api/2/user/anonymization/progress` | stays Server/DC-only |
| `users().get_user_list` | `GET /rest/api/2/user/list` | stays Server/DC-only; add `.stream()` |
| `users().remove_user_from_application` | `DELETE /rest/api/2/user/application` | stays Server/DC-only |
| `users().schedule_user_anonymization` | `POST /rest/api/2/user/anonymization` | stays Server/DC-only |
| `users().schedule_user_anonymization_rerun` | `POST /rest/api/2/user/anonymization/rerun` | stays Server/DC-only |
| `users().store_temporary_user_avatar_using_multi_part` | `POST /rest/api/2/user/avatar/temporary` | stays Server/DC-only |
| `users().unlock_anonymization` | `DELETE /rest/api/2/user/anonymization/unlock` | stays Server/DC-only |
| `users().update_user` | `PUT /rest/api/2/user` | stays Server/DC-only |
| `users().update_user_avatar` | `PUT /rest/api/2/user/avatar` | stays Server/DC-only |
| `users().validate_user_anonymization` | `GET /rest/api/2/user/anonymization` | stays Server/DC-only |
| `users().validate_user_anonymization_rerun` | `GET /rest/api/2/user/anonymization/rerun` | stays Server/DC-only |
| `webhooks().create_webhook` | `POST /rest/jira-webhook/1.0/webhooks` | stays Server/DC-only |
| `webhooks().delete_webhook` | `DELETE /rest/jira-webhook/1.0/webhooks/{…}` | stays Server/DC-only |
| `webhooks().get_latest_webhook_invocation` | `GET /rest/jira-webhook/1.0/webhooks/{…}/latest` | stays Server/DC-only |
| `webhooks().get_webhook` | `GET /rest/jira-webhook/1.0/webhooks/{…}` | stays Server/DC-only |
| `webhooks().get_webhook_statistics` | `GET /rest/jira-webhook/1.0/webhooks/{…}/statistics` | stays Server/DC-only |
| `webhooks().get_webhook_statistics_summary` | `GET /rest/jira-webhook/1.0/webhooks/{…}/statistics/summary` | stays Server/DC-only |
| `webhooks().get_webhook_transitions` | `GET /rest/jira-webhook/1.0/webhooks/{…}/transitions` | stays Server/DC-only |
| `webhooks().get_webhooks` | `GET /rest/jira-webhook/1.0/webhooks` | stays Server/DC-only; add `.stream()` |
| `webhooks().update_webhook` | `PUT /rest/jira-webhook/1.0/webhooks/{…}` | stays Server/DC-only |
| `websudo().release` | `DELETE /rest/auth/1/websudo` | stays Server/DC-only |
| `workflow_statuses().get_paginated_statuses` | `GET /rest/api/2/status/page` | stays Server/DC-only |
| `workflows().get_all_workflows` | `GET /rest/api/2/workflow` | stays Server/DC-only |

## Cloud only

| Cloud | Endpoint | Target |
|---|---|---|
| `AgileClient::backlog().move_issues_to_backlog_for_board` | `POST /rest/agile/1.0/backlog/{…}/issue` | stays Cloud-only |
| `AgileClient::board().get_all_quick_filters` | `GET /rest/agile/1.0/board/{…}/quickfilter` | stays Cloud-only |
| `AgileClient::board().get_approximate_issue_count_for_backlog` | `GET /rest/software/1.0/board/{…}/backlog/approximate-count` | stays Cloud-only |
| `AgileClient::board().get_approximate_issue_count_for_board` | `GET /rest/software/1.0/board/{…}/issue/approximate-count` | stays Cloud-only |
| `AgileClient::board().get_board_by_filter_id` | `GET /rest/agile/1.0/board/filter/{…}` | stays Cloud-only |
| `AgileClient::board().get_features_for_board` | `GET /rest/agile/1.0/board/{…}/features` | stays Cloud-only |
| `AgileClient::board().get_projects_full` | `GET /rest/agile/1.0/board/{…}/project/full` | stays Cloud-only |
| `AgileClient::board().get_quick_filter` | `GET /rest/agile/1.0/board/{…}/quickfilter/{…}` | stays Cloud-only |
| `AgileClient::board().get_reports_for_board` | `GET /rest/agile/1.0/board/{…}/reports` | stays Cloud-only |
| `AgileClient::board().move_issues_to_board` | `POST /rest/agile/1.0/board/{…}/issue` | stays Cloud-only |
| `AgileClient::board().toggle_features` | `PUT /rest/agile/1.0/board/{…}/features` | stays Cloud-only |
| `AgileClient::builds().delete_build_by_key` | `DELETE /rest/builds/0.1/pipelines/{…}/builds/{…}` | stays Cloud-only |
| `AgileClient::builds().delete_builds_by_property` | `DELETE /rest/builds/0.1/bulkByProperties` | stays Cloud-only |
| `AgileClient::builds().get_build_by_key` | `GET /rest/builds/0.1/pipelines/{…}/builds/{…}` | stays Cloud-only |
| `AgileClient::builds().submit_builds` | `POST /rest/builds/0.1/bulk` | stays Cloud-only |
| `AgileClient::deployments().delete_deployment_by_key` | `DELETE /rest/deployments/0.1/pipelines/{…}/environments/{…}/deployments/{…}` | stays Cloud-only |
| `AgileClient::deployments().delete_deployments_by_property` | `DELETE /rest/deployments/0.1/bulkByProperties` | stays Cloud-only |
| `AgileClient::deployments().get_deployment_by_key` | `GET /rest/deployments/0.1/pipelines/{…}/environments/{…}/deployments/{…}` | stays Cloud-only |
| `AgileClient::deployments().get_deployment_gating_status_by_key` | `GET /rest/deployments/0.1/pipelines/{…}/environments/{…}/deployments/{…}/gating-status` | stays Cloud-only |
| `AgileClient::deployments().submit_deployments` | `POST /rest/deployments/0.1/bulk` | stays Cloud-only |
| `AgileClient::development_information().delete_by_properties` | `DELETE /rest/devinfo/0.10/bulkByProperties` | stays Cloud-only |
| `AgileClient::development_information().delete_entity` | `DELETE /rest/devinfo/0.10/repository/{…}/{…}/{…}` | stays Cloud-only |
| `AgileClient::development_information().delete_repository` | `DELETE /rest/devinfo/0.10/repository/{…}` | stays Cloud-only |
| `AgileClient::development_information().exists_by_properties` | `GET /rest/devinfo/0.10/existsByProperties` | stays Cloud-only |
| `AgileClient::development_information().get_repository` | `GET /rest/devinfo/0.10/repository/{…}` | stays Cloud-only |
| `AgileClient::development_information().store_development_information` | `POST /rest/devinfo/0.10/bulk` | stays Cloud-only |
| `AgileClient::devops_components().delete_component_by_id` | `DELETE /rest/devopscomponents/1.0/devopscomponents/{…}` | stays Cloud-only |
| `AgileClient::devops_components().delete_components_by_property` | `DELETE /rest/devopscomponents/1.0/bulkByProperties` | stays Cloud-only |
| `AgileClient::devops_components().get_component_by_id` | `GET /rest/devopscomponents/1.0/devopscomponents/{…}` | stays Cloud-only |
| `AgileClient::devops_components().submit_components` | `POST /rest/devopscomponents/1.0/bulk` | stays Cloud-only |
| `AgileClient::feature_flags().delete_feature_flag_by_id` | `DELETE /rest/featureflags/0.1/flag/{…}` | stays Cloud-only |
| `AgileClient::feature_flags().delete_feature_flags_by_property` | `DELETE /rest/featureflags/0.1/bulkByProperties` | stays Cloud-only |
| `AgileClient::feature_flags().get_feature_flag_by_id` | `GET /rest/featureflags/0.1/flag/{…}` | stays Cloud-only |
| `AgileClient::feature_flags().submit_feature_flags` | `POST /rest/featureflags/0.1/bulk` | stays Cloud-only |
| `AgileClient::operations().delete_entity_by_property` | `DELETE /rest/operations/1.0/bulkByProperties` | stays Cloud-only |
| `AgileClient::operations().delete_incident_by_id` | `DELETE /rest/operations/1.0/incidents/{…}` | stays Cloud-only |
| `AgileClient::operations().delete_review_by_id` | `DELETE /rest/operations/1.0/post-incident-reviews/{…}` | stays Cloud-only |
| `AgileClient::operations().delete_workspaces` | `DELETE /rest/operations/1.0/linkedWorkspaces/bulk` | stays Cloud-only |
| `AgileClient::operations().get_incident_by_id` | `GET /rest/operations/1.0/incidents/{…}` | stays Cloud-only |
| `AgileClient::operations().get_review_by_id` | `GET /rest/operations/1.0/post-incident-reviews/{…}` | stays Cloud-only |
| `AgileClient::operations().get_workspaces` | `GET /rest/operations/1.0/linkedWorkspaces` | stays Cloud-only |
| `AgileClient::operations().submit_entity` | `POST /rest/operations/1.0/bulk` | stays Cloud-only |
| `AgileClient::operations().submit_operations_workspaces` | `POST /rest/operations/1.0/linkedWorkspaces/bulk` | stays Cloud-only |
| `AgileClient::remote_links().delete_remote_link_by_id` | `DELETE /rest/remotelinks/1.0/remotelink/{…}` | stays Cloud-only |
| `AgileClient::remote_links().delete_remote_links_by_property` | `DELETE /rest/remotelinks/1.0/bulkByProperties` | stays Cloud-only |
| `AgileClient::remote_links().get_remote_link_by_id` | `GET /rest/remotelinks/1.0/remotelink/{…}` | stays Cloud-only |
| `AgileClient::remote_links().submit_remote_links` | `POST /rest/remotelinks/1.0/bulk` | stays Cloud-only |
| `AgileClient::security_information().delete_linked_workspaces` | `DELETE /rest/security/1.0/linkedWorkspaces/bulk` | stays Cloud-only |
| `AgileClient::security_information().delete_vulnerabilities_by_property` | `DELETE /rest/security/1.0/bulkByProperties` | stays Cloud-only |
| `AgileClient::security_information().delete_vulnerability_by_id` | `DELETE /rest/security/1.0/vulnerability/{…}` | stays Cloud-only |
| `AgileClient::security_information().get_linked_workspace_by_id` | `GET /rest/security/1.0/linkedWorkspaces/{…}` | stays Cloud-only |
| `AgileClient::security_information().get_linked_workspaces` | `GET /rest/security/1.0/linkedWorkspaces` | stays Cloud-only |
| `AgileClient::security_information().get_vulnerability_by_id` | `GET /rest/security/1.0/vulnerability/{…}` | stays Cloud-only |
| `AgileClient::security_information().submit_vulnerabilities` | `POST /rest/security/1.0/bulk` | stays Cloud-only |
| `AgileClient::security_information().submit_workspaces` | `POST /rest/security/1.0/linkedWorkspaces/bulk` | stays Cloud-only |
| `CloudClient::announcement_banner().get_banner` | `GET /rest/api/3/announcementBanner` | stays Cloud-only |
| `CloudClient::announcement_banner().set_banner` | `PUT /rest/api/3/announcementBanner` | stays Cloud-only |
| `CloudClient::api().get_worklogs_by_issue_id_and_worklog_id` | `POST /rest/internal/api/latest/worklog/bulk` | stays Cloud-only |
| `CloudClient::app_data_policies().get_policies` | `GET /rest/api/3/data-policy/project` | stays Cloud-only |
| `CloudClient::app_data_policies().get_policy` | `GET /rest/api/3/data-policy` | stays Cloud-only |
| `CloudClient::app_migration().update_entity_properties_value` | `PUT /rest/atlassian-connect/1/migration/properties/{…}` | stays Cloud-only |
| `CloudClient::app_migration().update_issue_fields` | `PUT /rest/atlassian-connect/1/migration/field` | stays Cloud-only |
| `CloudClient::app_migration().workflow_rule_search` | `POST /rest/atlassian-connect/1/migration/workflow/rule/search` | stays Cloud-only |
| `CloudClient::app_properties().delete_addon_property` | `DELETE /rest/atlassian-connect/1/addons/{…}/properties/{…}` | stays Cloud-only |
| `CloudClient::app_properties().delete_forge_app_property` | `DELETE /rest/forge/1/app/properties/{…}` | stays Cloud-only |
| `CloudClient::app_properties().get_addon_properties` | `GET /rest/atlassian-connect/1/addons/{…}/properties` | stays Cloud-only |
| `CloudClient::app_properties().get_addon_property` | `GET /rest/atlassian-connect/1/addons/{…}/properties/{…}` | stays Cloud-only |
| `CloudClient::app_properties().get_forge_app_property` | `GET /rest/forge/1/app/properties/{…}` | stays Cloud-only |
| `CloudClient::app_properties().get_forge_app_property_keys` | `GET /rest/forge/1/app/properties` | stays Cloud-only |
| `CloudClient::app_properties().put_addon_property` | `PUT /rest/atlassian-connect/1/addons/{…}/properties/{…}` | stays Cloud-only |
| `CloudClient::app_properties().put_forge_app_property` | `PUT /rest/forge/1/app/properties/{…}` | stays Cloud-only |
| `CloudClient::audit_records().get_audit_records` | `GET /rest/api/3/auditing/record` | stays Cloud-only |
| `CloudClient::avatars().get_avatar_image_by_id` | `GET /rest/api/3/universal_avatar/view/type/{…}/avatar/{…}` | stays Cloud-only |
| `CloudClient::avatars().get_avatar_image_by_owner` | `GET /rest/api/3/universal_avatar/view/type/{…}/owner/{…}` | stays Cloud-only |
| `CloudClient::avatars().get_avatar_image_by_type` | `GET /rest/api/3/universal_avatar/view/type/{…}` | stays Cloud-only |
| `CloudClient::avatars().store_avatar` | `POST /rest/api/3/universal_avatar/type/{…}/owner/{…}` | stays Cloud-only |
| `CloudClient::dashboards().get_dashboards_paginated` | `GET /rest/api/3/dashboard/search` | stays Cloud-only |
| `CloudClient::dynamic_modules().get_modules` | `GET /rest/atlassian-connect/1/app/module/dynamic` | stays Cloud-only |
| `CloudClient::dynamic_modules().register_modules` | `POST /rest/atlassian-connect/1/app/module/dynamic` | stays Cloud-only |
| `CloudClient::dynamic_modules().remove_modules` | `DELETE /rest/atlassian-connect/1/app/module/dynamic` | stays Cloud-only |
| `CloudClient::filters().delete_favourite_for_filter` | `DELETE /rest/api/3/filter/{…}/favourite` | stays Cloud-only |
| `CloudClient::filters().get_filters_paginated` | `GET /rest/api/3/filter/search` | stays Cloud-only |
| `CloudClient::filters().get_my_filters` | `GET /rest/api/3/filter/my` | stays Cloud-only |
| `CloudClient::filters().set_favourite_for_filter` | `PUT /rest/api/3/filter/{…}/favourite` | stays Cloud-only |
| `CloudClient::issue_attachments().get_attachment_content` | `GET /rest/api/3/attachment/content/{…}` | stays Cloud-only |
| `CloudClient::issue_attachments().get_attachment_thumbnail` | `GET /rest/api/3/attachment/thumbnail/{…}` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().get_available_transitions` | `GET /rest/api/3/bulk/issues/transition` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().get_bulk_editable_fields` | `GET /rest/api/3/bulk/issues/fields` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().get_bulk_operation_progress` | `GET /rest/api/3/bulk/queue/{…}` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().submit_bulk_delete` | `POST /rest/api/3/bulk/issues/delete` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().submit_bulk_edit` | `POST /rest/api/3/bulk/issues/fields` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().submit_bulk_move` | `POST /rest/api/3/bulk/issues/move` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().submit_bulk_transition` | `POST /rest/api/3/bulk/issues/transition` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().submit_bulk_unwatch` | `POST /rest/api/3/bulk/issues/unwatch` | stays Cloud-only |
| `CloudClient::issue_bulk_operations().submit_bulk_watch` | `POST /rest/api/3/bulk/issues/watch` | stays Cloud-only |
| `CloudClient::issue_comments().get_comments_by_ids` | `POST /rest/api/3/comment/list` | stays Cloud-only; add `.stream()` |
| `CloudClient::issue_custom_field_associations().create_associations` | `PUT /rest/api/3/field/association` | stays Cloud-only |
| `CloudClient::issue_custom_field_associations().remove_associations` | `DELETE /rest/api/3/field/association` | stays Cloud-only |
| `CloudClient::issue_custom_field_configuration_apps().get_custom_field_configuration` | `GET /rest/api/3/app/field/{…}/context/configuration` | stays Cloud-only |
| `CloudClient::issue_custom_field_configuration_apps().update_custom_field_configuration` | `PUT /rest/api/3/app/field/{…}/context/configuration` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().add_issue_types_to_context` | `PUT /rest/api/3/field/{…}/context/{…}/issuetype` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().assign_projects_to_custom_field_context` | `PUT /rest/api/3/field/{…}/context/{…}/project` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().create_custom_field_context` | `POST /rest/api/3/field/{…}/context` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().delete_custom_field_context` | `DELETE /rest/api/3/field/{…}/context/{…}` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().get_context_default_values` | `GET /rest/api/3/field/{…}/context/defaultValues` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().get_contexts_for_field` | `GET /rest/api/3/field/{…}/context` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().get_custom_field_contexts_for_projects_and_issue_types` | `POST /rest/api/3/field/{…}/context/mapping` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().get_issue_type_mappings_for_contexts` | `GET /rest/api/3/field/{…}/context/issuetypemapping` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().get_project_context_mapping` | `GET /rest/api/3/field/{…}/context/projectmapping` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().remove_custom_field_context_from_projects` | `POST /rest/api/3/field/{…}/context/{…}/project/remove` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().remove_issue_types_from_context` | `POST /rest/api/3/field/{…}/context/{…}/issuetype/remove` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().set_context_default_values` | `PUT /rest/api/3/field/{…}/context/defaultValues` | stays Cloud-only |
| `CloudClient::issue_custom_field_contexts().update_custom_field_context` | `PUT /rest/api/3/field/{…}/context/{…}` | stays Cloud-only |
| `CloudClient::issue_custom_field_options().create_custom_field_option` | `POST /rest/api/3/field/{…}/context/{…}/option` | stays Cloud-only |
| `CloudClient::issue_custom_field_options().delete_custom_field_option` | `DELETE /rest/api/3/field/{…}/context/{…}/option/{…}` | stays Cloud-only |
| `CloudClient::issue_custom_field_options().get_options_for_context` | `GET /rest/api/3/field/{…}/context/{…}/option` | stays Cloud-only |
| `CloudClient::issue_custom_field_options().reorder_custom_field_options` | `PUT /rest/api/3/field/{…}/context/{…}/option/move` | stays Cloud-only |
| `CloudClient::issue_custom_field_options().replace_custom_field_option` | `DELETE /rest/api/3/field/{…}/context/{…}/option/{…}/issue` | stays Cloud-only |
| `CloudClient::issue_custom_field_options().update_custom_field_option` | `PUT /rest/api/3/field/{…}/context/{…}/option` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().create_issue_field_option` | `POST /rest/api/3/field/{…}/option` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().delete_issue_field_option` | `DELETE /rest/api/3/field/{…}/option/{…}` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().get_all_issue_field_options` | `GET /rest/api/3/field/{…}/option` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().get_issue_field_option` | `GET /rest/api/3/field/{…}/option/{…}` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().get_selectable_issue_field_options` | `GET /rest/api/3/field/{…}/option/suggestions/edit` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().get_visible_issue_field_options` | `GET /rest/api/3/field/{…}/option/suggestions/search` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().replace_issue_field_option` | `DELETE /rest/api/3/field/{…}/option/{…}/issue` | stays Cloud-only |
| `CloudClient::issue_custom_field_options_apps().update_issue_field_option` | `PUT /rest/api/3/field/{…}/option/{…}` | stays Cloud-only |
| `CloudClient::issue_custom_field_values_apps().update_custom_field_value` | `PUT /rest/api/3/app/field/{…}/value` | stays Cloud-only |
| `CloudClient::issue_custom_field_values_apps().update_multiple_custom_field_values` | `POST /rest/api/3/app/field/value` | stays Cloud-only |
| `CloudClient::issue_fields().delete_custom_field` | `DELETE /rest/api/3/field/{…}` | stays Cloud-only |
| `CloudClient::issue_fields().get_fields_paginated` | `GET /rest/api/3/field/search` | stays Cloud-only |
| `CloudClient::issue_fields().get_trashed_fields_paginated` | `GET /rest/api/3/field/search/trashed` | stays Cloud-only |
| `CloudClient::issue_fields().restore_custom_field` | `POST /rest/api/3/field/{…}/restore` | stays Cloud-only |
| `CloudClient::issue_fields().trash_custom_field` | `POST /rest/api/3/field/{…}/trash` | stays Cloud-only |
| `CloudClient::issue_fields().update_custom_field` | `PUT /rest/api/3/field/{…}` | stays Cloud-only |
| `CloudClient::issue_notification_schemes().add_notifications` | `PUT /rest/api/3/notificationscheme/{…}/notification` | stays Cloud-only |
| `CloudClient::issue_notification_schemes().get_notification_scheme_to_project_mappings` | `GET /rest/api/3/notificationscheme/project` | stays Cloud-only; add `.stream()` |
| `CloudClient::issue_notification_schemes().remove_notification_from_notification_scheme` | `DELETE /rest/api/3/notificationscheme/{…}/notification/{…}` | stays Cloud-only |
| `CloudClient::issue_panels().bulk_pin_unpin_projects_async` | `POST /rest/api/3/forge/panel/action/bulk/async` | stays Cloud-only |
| `CloudClient::issue_panels().get_bulk_pin_status` | `POST /rest/api/3/forge/panel/action/bulk/status` | stays Cloud-only |
| `CloudClient::issue_priorities().create_priority` | `POST /rest/api/3/priority` | stays Cloud-only |
| `CloudClient::issue_priorities().delete_priority` | `DELETE /rest/api/3/priority/{…}` | stays Cloud-only |
| `CloudClient::issue_priorities().move_priorities` | `PUT /rest/api/3/priority/move` | stays Cloud-only |
| `CloudClient::issue_priorities().search_priorities` | `GET /rest/api/3/priority/search` | stays Cloud-only; add `.stream()` |
| `CloudClient::issue_priorities().set_default_priority` | `PUT /rest/api/3/priority/default` | stays Cloud-only |
| `CloudClient::issue_priorities().update_priority` | `PUT /rest/api/3/priority/{…}` | stays Cloud-only |
| `CloudClient::issue_properties().bulk_delete_issue_property` | `DELETE /rest/api/3/issue/properties/{…}` | stays Cloud-only |
| `CloudClient::issue_properties().bulk_set_issue_properties_by_issue` | `POST /rest/api/3/issue/properties/multi` | stays Cloud-only |
| `CloudClient::issue_properties().bulk_set_issue_property` | `PUT /rest/api/3/issue/properties/{…}` | stays Cloud-only |
| `CloudClient::issue_properties().bulk_set_issues_properties_list` | `POST /rest/api/3/issue/properties` | stays Cloud-only |
| `CloudClient::issue_redaction().get_redaction_status` | `GET /rest/api/3/redact/status/{…}` | stays Cloud-only |
| `CloudClient::issue_redaction().redact` | `POST /rest/api/3/redact` | stays Cloud-only |
| `CloudClient::issue_search().count_issues` | `POST /rest/api/3/search/approximate-count` | stays Cloud-only |
| `CloudClient::issue_search().match_issues` | `POST /rest/api/3/jql/match` | stays Cloud-only |
| `CloudClient::issue_search().search_issues_post` | `POST /rest/api/3/search/jql` | stays Cloud-only |
| `CloudClient::issue_security_level().get_issue_security_level_members` | `GET /rest/api/3/issuesecurityschemes/{…}/members` | stays Cloud-only |
| `CloudClient::issue_type_schemes().add_issue_types_to_issue_type_scheme` | `PUT /rest/api/3/issuetypescheme/{…}/issuetype` | stays Cloud-only |
| `CloudClient::issue_type_schemes().assign_issue_type_scheme_to_project` | `PUT /rest/api/3/issuetypescheme/project` | stays Cloud-only |
| `CloudClient::issue_type_schemes().get_issue_type_scheme_for_projects` | `GET /rest/api/3/issuetypescheme/project` | stays Cloud-only |
| `CloudClient::issue_type_schemes().get_issue_type_schemes_mapping` | `GET /rest/api/3/issuetypescheme/mapping` | stays Cloud-only |
| `CloudClient::issue_type_schemes().remove_issue_type_from_issue_type_scheme` | `DELETE /rest/api/3/issuetypescheme/{…}/issuetype/{…}` | stays Cloud-only |
| `CloudClient::issue_type_schemes().reorder_issue_types_in_issue_type_scheme` | `PUT /rest/api/3/issuetypescheme/{…}/issuetype/move` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().append_mappings_for_issue_type_screen_scheme` | `PUT /rest/api/3/issuetypescreenscheme/{…}/mapping` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().assign_issue_type_screen_scheme_to_project` | `PUT /rest/api/3/issuetypescreenscheme/project` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().create_issue_type_screen_scheme` | `POST /rest/api/3/issuetypescreenscheme` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().delete_issue_type_screen_scheme` | `DELETE /rest/api/3/issuetypescreenscheme/{…}` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().get_issue_type_screen_scheme_mappings` | `GET /rest/api/3/issuetypescreenscheme/mapping` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().get_issue_type_screen_scheme_project_associations` | `GET /rest/api/3/issuetypescreenscheme/project` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().get_issue_type_screen_schemes` | `GET /rest/api/3/issuetypescreenscheme` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().get_projects_for_issue_type_screen_scheme` | `GET /rest/api/3/issuetypescreenscheme/{…}/project` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().remove_mappings_from_issue_type_screen_scheme` | `POST /rest/api/3/issuetypescreenscheme/{…}/mapping/remove` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().update_default_screen_scheme` | `PUT /rest/api/3/issuetypescreenscheme/{…}/mapping/default` | stays Cloud-only |
| `CloudClient::issue_type_screen_schemes().update_issue_type_screen_scheme` | `PUT /rest/api/3/issuetypescreenscheme/{…}` | stays Cloud-only |
| `CloudClient::issue_types().create_issue_type_avatar` | `POST /rest/api/3/issuetype/{…}/avatar2` | stays Cloud-only |
| `CloudClient::issue_watchers().get_is_watching_issue_bulk` | `POST /rest/api/3/issue/watching` | stays Cloud-only |
| `CloudClient::issue_worklog_properties().delete_worklog_property` | `DELETE /rest/api/3/issue/{…}/worklog/{…}/properties/{…}` | stays Cloud-only |
| `CloudClient::issue_worklog_properties().get_worklog_property` | `GET /rest/api/3/issue/{…}/worklog/{…}/properties/{…}` | stays Cloud-only |
| `CloudClient::issue_worklog_properties().get_worklog_property_keys` | `GET /rest/api/3/issue/{…}/worklog/{…}/properties` | stays Cloud-only |
| `CloudClient::issue_worklog_properties().set_worklog_property` | `PUT /rest/api/3/issue/{…}/worklog/{…}/properties/{…}` | stays Cloud-only |
| `CloudClient::issues().bulk_fetch_issues` | `POST /rest/api/3/issue/bulkfetch` | stays Cloud-only |
| `CloudClient::issues().get_bulk_changelogs` | `POST /rest/api/3/changelog/bulkfetch` | stays Cloud-only |
| `CloudClient::issues().get_change_logs` | `GET /rest/api/3/issue/{…}/changelog` | stays Cloud-only |
| `CloudClient::issues().get_change_logs_by_ids` | `POST /rest/api/3/issue/{…}/changelog/list` | stays Cloud-only; add `.stream()` |
| `CloudClient::jira_expressions().analyse_expression` | `POST /rest/api/3/expression/analyse` | stays Cloud-only |
| `CloudClient::jira_expressions().evaluate_expression` | `POST /rest/api/3/expression/evaluate` | stays Cloud-only |
| `CloudClient::jql().get_auto_complete_post` | `POST /rest/api/3/jql/autocompletedata` | stays Cloud-only |
| `CloudClient::jql().migrate_queries` | `POST /rest/api/3/jql/pdcleaner` | stays Cloud-only |
| `CloudClient::jql().parse_jql_queries` | `POST /rest/api/3/jql/parse` | stays Cloud-only |
| `CloudClient::jql_functions_apps().get_precomputations` | `GET /rest/api/3/jql/function/computation` | stays Cloud-only |
| `CloudClient::jql_functions_apps().get_precomputations_by_id` | `POST /rest/api/3/jql/function/computation/search` | stays Cloud-only |
| `CloudClient::jql_functions_apps().update_precomputations` | `POST /rest/api/3/jql/function/computation` | stays Cloud-only |
| `CloudClient::labels().get_all_labels` | `GET /rest/api/3/label` | stays Cloud-only; add `.stream()` |
| `CloudClient::migration_of_connect_modules_to_forge().fetch_migration_task` | `GET /rest/atlassian-connect/1/migration/{…}/{…}/task` | stays Cloud-only |
| `CloudClient::migration_of_connect_modules_to_forge().submit_task` | `POST /rest/atlassian-connect/1/migration/{…}/{…}/task` | stays Cloud-only |
| `CloudClient::myself().get_locale` | `GET /rest/api/3/mypreferences/locale` | stays Cloud-only |
| `CloudClient::permissions().get_bulk_permissions` | `POST /rest/api/3/permissions/check` | stays Cloud-only |
| `CloudClient::permissions().get_permitted_projects` | `POST /rest/api/3/permissions/project` | stays Cloud-only |
| `CloudClient::project_avatars().create_project_avatar` | `POST /rest/api/3/project/{…}/avatar2` | stays Cloud-only |
| `CloudClient::project_components().find_components_for_projects` | `GET /rest/api/3/component` | stays Cloud-only |
| `CloudClient::project_components().get_project_components_paginated` | `GET /rest/api/3/project/{…}/component` | stays Cloud-only |
| `CloudClient::project_email().get_project_email` | `GET /rest/api/3/project/{…}/email` | stays Cloud-only |
| `CloudClient::project_email().update_project_email` | `PUT /rest/api/3/project/{…}/email` | stays Cloud-only |
| `CloudClient::project_features().get_features_for_project` | `GET /rest/api/3/project/{…}/features` | stays Cloud-only |
| `CloudClient::project_features().toggle_feature_for_project` | `PUT /rest/api/3/project/{…}/features/{…}` | stays Cloud-only |
| `CloudClient::project_key_and_name_validation().get_valid_project_key` | `GET /rest/api/3/projectvalidate/validProjectKey` | stays Cloud-only |
| `CloudClient::project_key_and_name_validation().get_valid_project_name` | `GET /rest/api/3/projectvalidate/validProjectName` | stays Cloud-only |
| `CloudClient::project_roles().get_project_role_details` | `GET /rest/api/3/project/{…}/roledetails` | stays Cloud-only |
| `CloudClient::project_templates().create_project_with_custom_template` | `POST /rest/api/3/project-template` | stays Cloud-only |
| `CloudClient::project_types().get_all_accessible_project_types` | `GET /rest/api/3/project/type/accessible` | stays Cloud-only |
| `CloudClient::project_versions().create_related_work` | `POST /rest/api/3/version/{…}/relatedwork` | stays Cloud-only |
| `CloudClient::project_versions().delete_related_work` | `DELETE /rest/api/3/version/{…}/relatedwork/{…}` | stays Cloud-only |
| `CloudClient::project_versions().get_related_work` | `GET /rest/api/3/version/{…}/relatedwork` | stays Cloud-only |
| `CloudClient::project_versions().update_related_work` | `PUT /rest/api/3/version/{…}/relatedwork` | stays Cloud-only |
| `CloudClient::projects().archive_project` | `POST /rest/api/3/project/{…}/archive` | stays Cloud-only |
| `CloudClient::projects().get_hierarchy` | `GET /rest/api/3/project/{…}/hierarchy` | stays Cloud-only |
| `CloudClient::screen_schemes().create_screen_scheme` | `POST /rest/api/3/screenscheme` | stays Cloud-only |
| `CloudClient::screen_schemes().delete_screen_scheme` | `DELETE /rest/api/3/screenscheme/{…}` | stays Cloud-only |
| `CloudClient::screen_schemes().get_screen_schemes` | `GET /rest/api/3/screenscheme` | stays Cloud-only |
| `CloudClient::screen_schemes().update_screen_scheme` | `PUT /rest/api/3/screenscheme/{…}` | stays Cloud-only |
| `CloudClient::screens().get_screens_for_field` | `GET /rest/api/3/field/{…}/screens` | stays Cloud-only |
| `CloudClient::status().create_statuses` | `POST /rest/api/3/statuses` | stays Cloud-only |
| `CloudClient::status().delete_statuses_by_id` | `DELETE /rest/api/3/statuses` | stays Cloud-only |
| `CloudClient::status().get_project_issue_type_usages_for_status` | `GET /rest/api/3/statuses/{…}/project/{…}/issueTypeUsages` | stays Cloud-only; add `.stream()` |
| `CloudClient::status().get_project_usages_for_status` | `GET /rest/api/3/statuses/{…}/projectUsages` | stays Cloud-only; add `.stream()` |
| `CloudClient::status().get_statuses_by_id` | `GET /rest/api/3/statuses` | stays Cloud-only |
| `CloudClient::status().get_statuses_by_name` | `GET /rest/api/3/statuses/byNames` | stays Cloud-only |
| `CloudClient::status().get_workflow_usages_for_status` | `GET /rest/api/3/statuses/{…}/workflowUsages` | stays Cloud-only; add `.stream()` |
| `CloudClient::status().search_statuses` | `GET /rest/api/3/statuses/search` | stays Cloud-only; add `.stream()` |
| `CloudClient::status().update_statuses` | `PUT /rest/api/3/statuses` | stays Cloud-only |
| `CloudClient::tasks().get_task` | `GET /rest/api/3/task/{…}` | stays Cloud-only |
| `CloudClient::time_tracking().get_available_time_tracking_implementations` | `GET /rest/api/3/configuration/timetracking/list` | stays Cloud-only |
| `CloudClient::time_tracking().get_selected_time_tracking_implementation` | `GET /rest/api/3/configuration/timetracking` | stays Cloud-only |
| `CloudClient::time_tracking().get_shared_time_tracking_configuration` | `GET /rest/api/3/configuration/timetracking/options` | stays Cloud-only |
| `CloudClient::time_tracking().select_time_tracking_implementation` | `PUT /rest/api/3/configuration/timetracking` | stays Cloud-only |
| `CloudClient::time_tracking().set_shared_time_tracking_configuration` | `PUT /rest/api/3/configuration/timetracking/options` | stays Cloud-only |
| `CloudClient::ui_modifications_apps().create_ui_modification` | `POST /rest/api/3/uiModifications` | stays Cloud-only |
| `CloudClient::ui_modifications_apps().delete_ui_modification` | `DELETE /rest/api/3/uiModifications/{…}` | stays Cloud-only |
| `CloudClient::ui_modifications_apps().get_ui_modifications` | `GET /rest/api/3/uiModifications` | stays Cloud-only |
| `CloudClient::ui_modifications_apps().update_ui_modification` | `PUT /rest/api/3/uiModifications/{…}` | stays Cloud-only |
| `CloudClient::user_search().find_user_keys_by_query` | `GET /rest/api/3/user/search/query/key` | stays Cloud-only |
| `CloudClient::user_search().find_users_by_query` | `GET /rest/api/3/user/search/query` | stays Cloud-only |
| `CloudClient::user_search().find_users_with_all_permissions` | `GET /rest/api/3/user/permission/search` | stays Cloud-only; add `.stream()` |
| `CloudClient::users().get_all_users` | `GET /rest/api/3/users/search` | stays Cloud-only; add `.stream()` |
| `CloudClient::users().get_all_users_default` | `GET /rest/api/3/users` | stays Cloud-only; add `.stream()` |
| `CloudClient::users().get_user_email` | `GET /rest/api/3/user/email` | stays Cloud-only |
| `CloudClient::users().get_user_email_bulk` | `GET /rest/api/3/user/email/bulk` | stays Cloud-only |
| `CloudClient::users().get_user_groups` | `GET /rest/api/3/user/groups` | stays Cloud-only |
| `CloudClient::webhooks().delete_webhook_by_id` | `DELETE /rest/api/3/webhook` | stays Cloud-only |
| `CloudClient::webhooks().get_dynamic_webhooks_for_app` | `GET /rest/api/3/webhook` | stays Cloud-only |
| `CloudClient::webhooks().refresh_webhooks` | `PUT /rest/api/3/webhook/refresh` | stays Cloud-only |
| `CloudClient::webhooks().register_dynamic_webhooks` | `POST /rest/api/3/webhook` | stays Cloud-only |
| `CloudClient::workflow_scheme_drafts().publish_draft_workflow_scheme` | `POST /rest/api/3/workflowscheme/{…}/draft/publish` | stays Cloud-only |
| `CloudClient::workflow_scheme_project_associations().assign_scheme_to_project` | `PUT /rest/api/3/workflowscheme/project` | stays Cloud-only |
| `CloudClient::workflow_scheme_project_associations().get_workflow_scheme_project_associations` | `GET /rest/api/3/workflowscheme/project` | stays Cloud-only |
| `CloudClient::workflow_schemes().get_all_workflow_schemes` | `GET /rest/api/3/workflowscheme` | stays Cloud-only |
| `CloudClient::workflow_schemes().get_project_usages_for_workflow_scheme` | `GET /rest/api/3/workflowscheme/{…}/projectUsages` | stays Cloud-only; add `.stream()` |
| `CloudClient::workflow_schemes().get_required_workflow_scheme_mappings` | `POST /rest/api/3/workflowscheme/update/mappings` | stays Cloud-only |
| `CloudClient::workflow_schemes().read_workflow_schemes` | `POST /rest/api/3/workflowscheme/read` | stays Cloud-only |
| `CloudClient::workflow_schemes().update_schemes` | `POST /rest/api/3/workflowscheme/update` | stays Cloud-only |
| `CloudClient::workflow_transition_rules().delete_workflow_transition_rule_configurations` | `PUT /rest/api/3/workflow/rule/config/delete` | stays Cloud-only |
| `CloudClient::workflow_transition_rules().get_workflow_transition_rule_configurations` | `GET /rest/api/3/workflow/rule/config` | stays Cloud-only |
| `CloudClient::workflow_transition_rules().update_workflow_transition_rule_configurations` | `PUT /rest/api/3/workflow/rule/config` | stays Cloud-only |
| `CloudClient::workflows().copy_workflow` | `POST /rest/api/3/workflows/copy` | stays Cloud-only |
| `CloudClient::workflows().create_workflows` | `POST /rest/api/3/workflows/create` | stays Cloud-only |
| `CloudClient::workflows().delete_inactive_workflow` | `DELETE /rest/api/3/workflow/{…}` | stays Cloud-only |
| `CloudClient::workflows().get_default_editor` | `GET /rest/api/3/workflows/defaultEditor` | stays Cloud-only |
| `CloudClient::workflows().get_project_usages_for_workflow` | `GET /rest/api/3/workflow/{…}/projectUsages` | stays Cloud-only; add `.stream()` |
| `CloudClient::workflows().get_workflow_project_issue_type_usages` | `GET /rest/api/3/workflow/{…}/project/{…}/issueTypeUsages` | stays Cloud-only; add `.stream()` |
| `CloudClient::workflows().get_workflow_scheme_usages_for_workflow` | `GET /rest/api/3/workflow/{…}/workflowSchemes` | stays Cloud-only; add `.stream()` |
| `CloudClient::workflows().list_workflow_history` | `POST /rest/api/3/workflow/history/list` | stays Cloud-only |
| `CloudClient::workflows().read_workflow_from_history` | `POST /rest/api/3/workflow/history` | stays Cloud-only |
| `CloudClient::workflows().read_workflow_previews` | `POST /rest/api/3/workflows/preview` | stays Cloud-only |
| `CloudClient::workflows().read_workflows` | `POST /rest/api/3/workflows` | stays Cloud-only |
| `CloudClient::workflows().search_workflows` | `GET /rest/api/3/workflows/search` | stays Cloud-only; add `.stream()` |
| `CloudClient::workflows().update_workflows` | `POST /rest/api/3/workflows/update` | stays Cloud-only |
| `CloudClient::workflows().validate_create_workflows` | `POST /rest/api/3/workflows/create/validation` | stays Cloud-only |
| `CloudClient::workflows().validate_update_workflows` | `POST /rest/api/3/workflows/update/validation` | stays Cloud-only |
| `CloudClient::workflows().workflow_capabilities` | `GET /rest/api/3/workflows/capabilities` | stays Cloud-only |
