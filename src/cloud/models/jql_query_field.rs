// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A field used in a JQL query. See [Advanced searching - fields reference](https://confluence.atlassian.com/x/dAiiLQ) for more information about fields in JQL queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlQueryField {
    /// The encoded name of the field, which can be used directly in a JQL query.
    #[serde(rename = "encodedName", default, skip_serializing_if = "Option::is_none")]
    pub encoded_name: Option<String>,
    /// The name of the field.
    pub name: String,
    /// When the field refers to a value in an entity property, details of the entity property value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<JqlQueryFieldEntityProperty>>,
}
