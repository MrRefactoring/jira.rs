// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum UnclaimedDomainResponseKey {
        ForbiddenUnclaimedDomain => "forbidden.unclaimedDomain",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnclaimedDomainResponseContext {
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnclaimedDomainResponse {
    pub key: UnclaimedDomainResponseKey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<UnclaimedDomainResponseContext>,
}
