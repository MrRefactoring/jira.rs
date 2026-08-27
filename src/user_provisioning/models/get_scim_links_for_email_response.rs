// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response containing SCIM user links for an email address.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScimLinksForEmailResponse {
    /// List of SCIM user links associated with the email address.
    #[serde(rename = "scimLinks", default, skip_serializing_if = "Option::is_none")]
    pub scim_links: Option<Vec<ScimUserLink>>,
}
