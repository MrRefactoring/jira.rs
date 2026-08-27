// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectTypeAttribute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "defaultType", default, skip_serializing_if = "Option::is_none")]
    pub default_type: Option<DefaultType>,
    #[serde(rename = "typeValue", default, skip_serializing_if = "Option::is_none")]
    pub type_value: Option<String>,
    #[serde(rename = "typeValueMulti", default, skip_serializing_if = "Option::is_none")]
    pub type_value_multi: Option<Vec<String>>,
    #[serde(rename = "additionalValue", default, skip_serializing_if = "Option::is_none")]
    pub additional_value: Option<String>,
    #[serde(rename = "referenceType", default, skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ReferenceType>,
    #[serde(rename = "referenceObjectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub reference_object_type_id: Option<i64>,
    #[serde(rename = "referenceObjectType", default, skip_serializing_if = "Option::is_none")]
    pub reference_object_type: Option<ObjectType>,
    #[serde(rename = "confluenceTypeValue", default, skip_serializing_if = "Option::is_none")]
    pub confluence_type_value: Option<ApplicationLink>,
    #[serde(rename = "confluenceAddValue", default, skip_serializing_if = "Option::is_none")]
    pub confluence_add_value: Option<ConfluenceSpace>,
    #[serde(rename = "versionTypeValues", default, skip_serializing_if = "Option::is_none")]
    pub version_type_values: Option<Vec<Project>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(rename = "minimumCardinality", default, skip_serializing_if = "Option::is_none")]
    pub minimum_cardinality: Option<i64>,
    #[serde(rename = "maximumCardinality", default, skip_serializing_if = "Option::is_none")]
    pub maximum_cardinality: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removable: Option<bool>,
    #[serde(rename = "objectAttributeExists", default, skip_serializing_if = "Option::is_none")]
    pub object_attribute_exists: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(rename = "includeChildObjectTypes", default, skip_serializing_if = "Option::is_none")]
    pub include_child_object_types: Option<bool>,
    #[serde(rename = "uniqueAttribute", default, skip_serializing_if = "Option::is_none")]
    pub unique_attribute: Option<bool>,
    #[serde(rename = "regexValidation", default, skip_serializing_if = "Option::is_none")]
    pub regex_validation: Option<String>,
    #[serde(rename = "qlQuery", default, skip_serializing_if = "Option::is_none")]
    pub ql_query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iql: Option<String>,
    #[serde(rename = "versionTypeValue", default, skip_serializing_if = "Option::is_none")]
    pub version_type_value: Option<Project>,
}
