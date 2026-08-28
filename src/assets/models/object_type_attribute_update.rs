// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Input entity to update an object type attribute
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectTypeAttributeUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// | Value | Description|
    /// | ----- | ----------- |
    /// | 0 | Default|
    /// | 1 | Object reference|
    /// | 2 | User|
    /// | 4 | Group |
    /// | 7 | Status |
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// | Id | Description (mandatory if type = Default) |
    /// | -- | ----------- |
    /// | -1 | None |
    /// | 0 | Text |
    /// | 1 | Integer |
    /// | 2 | Boolean |
    /// | 3 | Double |
    /// | 4 | Date |
    /// | 5 | Time |
    /// | 6 | DateTime |
    /// | 7 | Url |
    /// | 8 | Email |
    /// | 9 | Textarea |
    /// | 10 | Select |
    /// | 11 | IP Address |
    #[serde(rename = "defaultTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_type_id: Option<i64>,
    /// It is mandatory for Type = Object reference and should point to the referenced object type id
    #[serde(rename = "typeValue", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    /// Valid for Type User. The Jira groups to restrict selection to
    #[serde(rename = "typeValueMulti", default, skip_serializing_if = "Option::is_none")]
    pub type_value_multi: Option<Vec<String>>,
    /// Valid for Type Url, User, Object and Confluence. For Url (DISABLED, ENABLED), for Object (ReferenceTypeId), for User (SHOW_PROFILE, HIDE_PROFILE), for Confluence (Confluence Space Id). It is mandatory for Type = Object reference
    #[serde(rename = "additionalValue", default, skip_serializing_if = "Option::is_none")]
    pub additional_value: Option<String>,
    /// Valid for Type Email, Select, Object, User, Group, Version and Project
    #[serde(rename = "minimumCardinality", default, skip_serializing_if = "Option::is_none")]
    pub minimum_cardinality: Option<i64>,
    /// Valid for Type Email, Select, Object, User, Group, Version and Project
    #[serde(rename = "maximumCardinality", default, skip_serializing_if = "Option::is_none")]
    pub maximum_cardinality: Option<i64>,
    /// Valid for Integer and Double object type attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Valid for Type = Object reference and describes if children object types should be included in the selectable objects as well
    #[serde(rename = "includeChildObjectTypes", default, skip_serializing_if = "Option::is_none")]
    pub include_child_object_types: Option<bool>,
    /// Hide the object type attributes for Assets Users
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Should the values be unique for object attributes associated with this object type attribute
    #[serde(rename = "uniqueAttribute", default, skip_serializing_if = "Option::is_none")]
    pub unique_attribute: Option<bool>,
    /// Valid for Type Integer and Double. Should a sum be included in the view
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summable: Option<bool>,
    /// Valid for Type Text and Email
    #[serde(rename = "regexValidation", default, skip_serializing_if = "Option::is_none")]
    pub regex_validation: Option<String>,
    /// Valid for Type object reference. Allows specifying an AQL query to restrict which objects are selectable.
    #[serde(rename = "qlQuery", default, skip_serializing_if = "Option::is_none")]
    pub ql_query: Option<String>,
    /// Valid for Type Select. A comma separated list of all chosable options
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
