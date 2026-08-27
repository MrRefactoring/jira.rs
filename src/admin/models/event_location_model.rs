// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLocationModel {
    /// IP address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Geo location of the IP address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    /// Country location according to the IP address
    #[serde(rename = "countryName", default, skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    /// Region location according to the IP address
    #[serde(rename = "regionName", default, skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// City location according to the IP address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}
