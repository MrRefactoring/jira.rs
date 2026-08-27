// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTeamToExternalSourcePayload {
    #[serde(rename = "externalReference")]
    pub external_reference: ExternalReference,
    /// The siteId to help locate the externalReference. For example, when the externalReference is a group belonging to a site. \[Deprecated\] Omitting siteId is deprecated. With the introduction of Units, orgId alone is no longer sufficient to resolve the scope of teams. Always provide a valid siteId to ensure this operation continues to work in the future.
    #[serde(rename = "siteId", default, skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}
