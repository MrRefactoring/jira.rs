// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServiceAccount {
    #[serde(rename = "clientConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub client_configuration_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectKeys", default, skip_serializing_if = "Option::is_none")]
    pub project_keys: Option<Vec<String>>,
}
