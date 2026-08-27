// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Number of contexts where the field is used.
    #[serde(rename = "contextsCount", default, skip_serializing_if = "Option::is_none")]
    pub contexts_count: Option<i64>,
    /// The description of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the field.
    pub id: String,
    /// Whether the field is locked.
    #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// Whether the field is shown on screen or not.
    #[serde(rename = "isUnscreenable", default, skip_serializing_if = "Option::is_none")]
    pub is_unscreenable: Option<bool>,
    /// The key of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "lastUsed", default, skip_serializing_if = "Option::is_none")]
    pub last_used: Option<FieldLastUsed>,
    /// The name of the field.
    pub name: String,
    /// Number of projects where the field is used.
    #[serde(rename = "projectsCount", default, skip_serializing_if = "Option::is_none")]
    pub projects_count: Option<i64>,
    pub schema: JsonType,
    /// Number of screens where the field is used.
    #[serde(rename = "screensCount", default, skip_serializing_if = "Option::is_none")]
    pub screens_count: Option<i64>,
    /// The searcher key of the field. Returned for custom fields.
    #[serde(rename = "searcherKey", default, skip_serializing_if = "Option::is_none")]
    pub searcher_key: Option<String>,
    /// The stable ID of the field.
    #[serde(rename = "stableId", default, skip_serializing_if = "Option::is_none")]
    pub stable_id: Option<String>,
    /// The translated (i18n) description of the field for the current locale. Returned for custom fields.
    #[serde(rename = "translatedDescription", default, skip_serializing_if = "Option::is_none")]
    pub translated_description: Option<String>,
    /// The translated (i18n) name of the field for the current locale. Returned for custom fields.
    #[serde(rename = "translatedName", default, skip_serializing_if = "Option::is_none")]
    pub translated_name: Option<String>,
    /// The display name of the field type
    #[serde(rename = "typeDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub type_display_name: Option<String>,
    #[serde(rename = "areOptionsSupported", default, skip_serializing_if = "Option::is_none")]
    pub are_options_supported: Option<bool>,
    #[serde(rename = "isOptionsCountOverLimit", default, skip_serializing_if = "Option::is_none")]
    pub is_options_count_over_limit: Option<bool>,
}
