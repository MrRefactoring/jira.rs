// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityType {
    #[serde(rename = "applicationTypeClassName", default, skip_serializing_if = "Option::is_none")]
    pub application_type_class_name: Option<String>,
    #[serde(rename = "i18nKey", default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "pluralizedI18nKey", default, skip_serializing_if = "Option::is_none")]
    pub pluralized_i18n_key: Option<String>,
}
