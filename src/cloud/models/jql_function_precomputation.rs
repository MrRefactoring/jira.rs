// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Jql function precomputation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlFunctionPrecomputation {
    /// The list of arguments function was invoked with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    /// The timestamp of the precomputation creation.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The timestamp of the precomputation creation.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// The error message to be displayed to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The field the function was executed against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The function key.
    #[serde(rename = "functionKey", default, skip_serializing_if = "Option::is_none")]
    pub function_key: Option<String>,
    /// The name of the function.
    #[serde(rename = "functionName", default, skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// The id of the precomputation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The operator in context of which function was executed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// The timestamp of the precomputation last update.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// The timestamp of the precomputation last update.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    /// The timestamp of the precomputation last usage.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub used: Option<chrono::DateTime<chrono::Utc>>,
    /// The timestamp of the precomputation last usage.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub used: Option<String>,
    /// The JQL fragment stored as the precomputation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
