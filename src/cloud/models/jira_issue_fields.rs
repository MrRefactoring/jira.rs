// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraIssueFields {
    /// Add or clear a cascading select field:
    ///
    ///  *  To add, specify `optionId` for both parent and child.
    ///  *  To clear the child, set its `optionId` to null.
    ///  *  To clear both, set the parent's `optionId` to null.
    #[serde(rename = "cascadingSelectFields", default, skip_serializing_if = "Option::is_none")]
    pub cascading_select_fields: Option<Vec<JiraCascadingSelectField>>,
    /// Add or clear a number field:
    ///
    ///  *  To add, specify a numeric `value`.
    ///  *  To clear, set `value` to `null`.
    #[serde(rename = "clearableNumberFields", default, skip_serializing_if = "Option::is_none")]
    pub clearable_number_fields: Option<Vec<JiraNumberField>>,
    /// Add or clear a color field:
    ///
    ///  *  To add, specify the color `name`. Available colors are: `purple`, `blue`, `green`, `teal`, `yellow`, `orange`, `grey`, `dark purple`, `dark blue`, `dark green`, `dark teal`, `dark yellow`, `dark orange`, `dark grey`.
    ///  *  To clear, set the color `name` to an empty string.
    #[serde(rename = "colorFields", default, skip_serializing_if = "Option::is_none")]
    pub color_fields: Option<Vec<JiraColorField>>,
    /// Add or clear a date picker field:
    ///
    ///  *  To add, specify the date in `d/mmm/yy` format or ISO format `dd-mm-yyyy`.
    ///  *  To clear, set `formattedDate` to an empty string.
    #[serde(rename = "datePickerFields", default, skip_serializing_if = "Option::is_none")]
    pub date_picker_fields: Option<Vec<JiraDateField>>,
    /// Add or clear the planned start date and time:
    ///
    ///  *  To add, specify the date and time in ISO format for `formattedDateTime`.
    ///  *  To clear, provide an empty string for `formattedDateTime`.
    #[serde(rename = "dateTimePickerFields", default, skip_serializing_if = "Option::is_none")]
    pub date_time_picker_fields: Option<Vec<JiraDateTimeField>>,
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<JiraIssueTypeField>,
    /// Edit a labels field:
    ///
    ///  *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.
    ///  *  To clear labels, use the `REMOVE_ALL` option with an empty `labels` array.
    #[serde(rename = "labelsFields", default, skip_serializing_if = "Option::is_none")]
    pub labels_fields: Option<Vec<JiraLabelsField>>,
    /// Add or clear a multi-group picker field:
    ///
    ///  *  To add groups, provide an array of groups with `groupName`s.
    ///  *  To clear all groups, use an empty `groups` array.
    #[serde(rename = "multipleGroupPickerFields", default, skip_serializing_if = "Option::is_none")]
    pub multiple_group_picker_fields: Option<Vec<JiraMultipleGroupPickerField>>,
    /// Assign or unassign multiple users to/from a field:
    ///
    ///  *  To assign, provide an array of user `accountId`s.
    ///  *  To clear, set `users` to `null`.
    #[serde(rename = "multipleSelectClearableUserPickerFields", default, skip_serializing_if = "Option::is_none")]
    pub multiple_select_clearable_user_picker_fields: Option<Vec<JiraMultipleSelectUserPickerField>>,
    /// Add or clear a multi-select field:
    ///
    ///  *  To add, provide an array of options with `optionId`s.
    ///  *  To clear, use an empty `options` array.
    #[serde(rename = "multipleSelectFields", default, skip_serializing_if = "Option::is_none")]
    pub multiple_select_fields: Option<Vec<JiraMultipleSelectField>>,
    /// Edit a multi-version picker field like Fix Versions/Affects Versions:
    ///
    ///  *  Options include `ADD`, `REPLACE`, `REMOVE`, or `REMOVE_ALL` for bulk edits.
    ///  *  To clear the field, use the `REMOVE_ALL` option with an empty `versions` array.
    #[serde(rename = "multipleVersionPickerFields", default, skip_serializing_if = "Option::is_none")]
    pub multiple_version_picker_fields: Option<Vec<JiraMultipleVersionPickerField>>,
    #[serde(rename = "multiselectComponents", default, skip_serializing_if = "Option::is_none")]
    pub multiselect_components: Option<JiraMultiSelectComponentField>,
    #[serde(rename = "originalEstimateField", default, skip_serializing_if = "Option::is_none")]
    pub original_estimate_field: Option<JiraDurationField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<JiraPriorityField>,
    /// Add or clear a rich text field:
    ///
    ///  *  To add, provide `adfValue`. Note that rich text fields only support ADF values.
    ///  *  To clear, use an empty `richText` object.
    ///
    /// For ADF format details, refer to: [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure).
    #[serde(rename = "richTextFields", default, skip_serializing_if = "Option::is_none")]
    pub rich_text_fields: Option<Vec<JiraRichTextField>>,
    /// Add or clear a single group picker field:
    ///
    ///  *  To add, specify the group with `groupName`.
    ///  *  To clear, set `groupName` to an empty string.
    #[serde(rename = "singleGroupPickerFields", default, skip_serializing_if = "Option::is_none")]
    pub single_group_picker_fields: Option<Vec<JiraSingleGroupPickerField>>,
    /// Add or clear a single line text field:
    ///
    ///  *  To add, provide the `text` value.
    ///  *  To clear, set `text` to an empty string.
    #[serde(rename = "singleLineTextFields", default, skip_serializing_if = "Option::is_none")]
    pub single_line_text_fields: Option<Vec<JiraSingleLineTextField>>,
    /// Edit assignment for single select user picker fields like Assignee/Reporter:
    ///
    ///  *  To assign an issue, specify the user's `accountId`.
    ///  *  To unassign an issue, set `user` to `null`.
    ///  *  For automatic assignment, set `accountId` to `-1`.
    #[serde(rename = "singleSelectClearableUserPickerFields", default, skip_serializing_if = "Option::is_none")]
    pub single_select_clearable_user_picker_fields: Option<Vec<JiraSingleSelectUserPickerField>>,
    /// Add or clear a single select field:
    ///
    ///  *  To add, specify the option with an `optionId`.
    ///  *  To clear, pass an option with `optionId` as `-1`.
    #[serde(rename = "singleSelectFields", default, skip_serializing_if = "Option::is_none")]
    pub single_select_fields: Option<Vec<JiraSingleSelectField>>,
    /// Add or clear a single version picker field:
    ///
    ///  *  To add, specify the version with a `versionId`.
    ///  *  To clear, set `versionId` to `-1`.
    #[serde(rename = "singleVersionPickerFields", default, skip_serializing_if = "Option::is_none")]
    pub single_version_picker_fields: Option<Vec<JiraSingleVersionPickerField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<JiraStatusInput>,
    #[serde(rename = "timeTrackingField", default, skip_serializing_if = "Option::is_none")]
    pub time_tracking_field: Option<JiraTimeTrackingField>,
    /// Add or clear a URL field:
    ///
    ///  *  To add, provide the `url` with the desired URL value.
    ///  *  To clear, set `url` to an empty string.
    #[serde(rename = "urlFields", default, skip_serializing_if = "Option::is_none")]
    pub url_fields: Option<Vec<JiraUrlField>>,
}
