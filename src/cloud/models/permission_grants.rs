// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of permission grants.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PermissionGrants {
    /// Expand options that include additional permission grant details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Permission grants list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionGrant>>,
}
