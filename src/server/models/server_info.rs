// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerInfo {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub version: String,
    #[serde(rename = "versionNumbers")]
    pub version_numbers: Vec<i64>,
    #[serde(rename = "deploymentType")]
    pub deployment_type: String,
    #[serde(rename = "buildNumber", default, skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i64>,
    #[serde(rename = "buildDate", default, skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "databaseBuildNumber", default, skip_serializing_if = "Option::is_none")]
    pub database_build_number: Option<i64>,
    #[serde(rename = "serverTime", default, skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(rename = "scmInfo", default, skip_serializing_if = "Option::is_none")]
    pub scm_info: Option<String>,
    #[serde(rename = "serverTitle", default, skip_serializing_if = "Option::is_none")]
    pub server_title: Option<String>,
}
