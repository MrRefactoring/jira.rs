// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlassianAccountUser {
    #[serde(flatten)]
    pub user: User,
    pub nickname: Nickname,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zoneinfo: Option<ZoneInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<Locale>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_profile: Option<ExtendedProfile>,
}
