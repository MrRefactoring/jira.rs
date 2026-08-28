// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "descriptionI18nKey", default, skip_serializing_if = "Option::is_none")]
    pub description_i18n_key: Option<String>,
    #[serde(rename = "formattedKey", default, skip_serializing_if = "Option::is_none")]
    pub formatted_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
