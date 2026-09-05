// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response containing SCIM links for an Atlassian account ID
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetScimLinksForAaIdInAnOrgResponse {
    /// List of SCIM user links associated with the Atlassian account ID.
    #[serde(rename = "scimLinks", default, skip_serializing_if = "Option::is_none")]
    pub scim_links: Option<Vec<ScimUserLink>>,
}
