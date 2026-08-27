// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LicenseValidationResults {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "licenseString", default, skip_serializing_if = "Option::is_none")]
    pub license_string: Option<String>,
}
