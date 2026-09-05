// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Lists of JQL reference data.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JQLReferenceData {
    /// List of JQL query reserved words.
    #[serde(rename = "jqlReservedWords", default, skip_serializing_if = "Option::is_none")]
    pub jql_reserved_words: Option<Vec<String>>,
    /// List of fields usable in JQL queries.
    #[serde(rename = "visibleFieldNames", default, skip_serializing_if = "Option::is_none")]
    pub visible_field_names: Option<Vec<FieldReferenceData>>,
    /// List of functions usable in JQL queries.
    #[serde(rename = "visibleFunctionNames", default, skip_serializing_if = "Option::is_none")]
    pub visible_function_names: Option<Vec<FunctionReferenceData>>,
}
