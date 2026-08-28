// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// API Token information
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApiTokenModel {
    /// Human readable description for the token.
    pub label: String,
    /// Timestamp last time the token was used to Authenticate as a UTC-ISO8601 string
    #[serde(rename = "lastAccess", default, skip_serializing_if = "Option::is_none")]
    pub last_access: Option<String>,
    /// Timestamp of when the token was generated as a UTC-ISO8601 string
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Container token id. This is the identifier of the system user associated with the container token.
    pub id: String,
    /// Timestamp of the token expiry as a UTC-ISO8601 string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
}
