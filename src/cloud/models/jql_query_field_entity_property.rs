// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of the property value extraction. Not available if the extraction for the property is not registered on the instance with the [Entity property](https://developer.atlassian.com/cloud/jira/platform/modules/entity-property/) module.
    pub enum JqlQueryFieldEntityPropertyType {
        Number => "number",
        String => "string",
        Text => "text",
        Date => "date",
        User => "user",
    }
}

/// Details of an entity property.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlQueryFieldEntityProperty {
    /// The object on which the property is set.
    pub entity: String,
    /// The key of the property.
    pub key: String,
    /// The path in the property value to query.
    pub path: String,
    /// The type of the property value extraction. Not available if the extraction for the property is not registered on the instance with the [Entity property](https://developer.atlassian.com/cloud/jira/platform/modules/entity-property/) module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<JqlQueryFieldEntityPropertyType>,
}
