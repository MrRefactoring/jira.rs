// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProductAccessModel {
    /// Products accessed by the user
    pub product_access: Vec<UserProductLastActive>,
    /// Date the user was added to the organization in ISO 8601 format (UTC), with the format yyyy-MM-dd.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_to_org: Option<String>,
    /// Date and timestamp the user was added to the organization in ISO 8601 format (UTC), with the format yyyy-MM-dd'T'HH:mm:ss'Z'.
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub added_to_org_timestamp: Option<String>,
}
