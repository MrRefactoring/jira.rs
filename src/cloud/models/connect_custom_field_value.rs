// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of custom field.
    pub enum ConnectCustomFieldValueType {
        StringIssueField => "StringIssueField",
        NumberIssueField => "NumberIssueField",
        RichTextIssueField => "RichTextIssueField",
        SingleSelectIssueField => "SingleSelectIssueField",
        MultiSelectIssueField => "MultiSelectIssueField",
        TextIssueField => "TextIssueField",
    }
}

/// A list of custom field details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectCustomFieldValue {
    /// The type of custom field.
    #[serde(rename = "_type")]
    pub r#type: ConnectCustomFieldValueType,
    /// The custom field ID.
    #[serde(rename = "fieldID")]
    pub field_id: i64,
    /// The issue ID.
    #[serde(rename = "issueID")]
    pub issue_id: i64,
    /// The value of number type custom field when `_type` is `NumberIssueField`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    /// The value of single select and multiselect custom field type when `_type` is `SingleSelectIssueField` or `MultiSelectIssueField`.
    #[serde(rename = "optionID", default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
    /// The value of richText type custom field when `_type` is `RichTextIssueField`.
    #[serde(rename = "richText", default, skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
    /// The value of string type custom field when `_type` is `StringIssueField`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// The value of of text custom field type when `_type` is `TextIssueField`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
