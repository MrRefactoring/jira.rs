// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// SCIM user name
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimUserName {
    /// The full name, including all middle names, titles, and suffixes as appropriate, formatted for display.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    /// The family name of the User.
    #[serde(rename = "familyName", default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// The given name of the User.
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// The middle name(s) of the User.
    #[serde(rename = "middleName", default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The honorific prefix(es) of the User, or title in most Western languages.
    #[serde(rename = "honorificPrefix", default, skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    /// The honorific suffix(es) of the User, or suffix in most Western languages.
    #[serde(rename = "honorificSuffix", default, skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
}
