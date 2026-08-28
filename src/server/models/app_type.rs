// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AppType {
    #[serde(rename = "i18nKey", default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}
