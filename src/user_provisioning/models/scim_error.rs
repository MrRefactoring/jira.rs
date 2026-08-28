// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Keyword for SCIM detail error.
    pub enum ScimErrorScimType {
        InvalidFilter => "invalidFilter",
        TooMany => "tooMany",
        Uniqueness => "uniqueness",
        Mutability => "mutability",
        InvalidSyntax => "invalidSyntax",
        InvalidPath => "invalidPath",
        NoTarget => "noTarget",
        InvalidValue => "invalidValue",
        InvalidVers => "invalidVers",
        Sensitive => "sensitive",
    }
}

/// SCIM Error
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScimError {
    /// SCIM error schemas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// The HTTP status code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Keyword for SCIM detail error.
    #[serde(rename = "scimType", default, skip_serializing_if = "Option::is_none")]
    pub scim_type: Option<ScimErrorScimType>,
    /// Detailed human-readable message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
