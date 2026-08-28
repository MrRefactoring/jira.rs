// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Type name of this object
    pub enum DomainModelType {
        Domains => "domains",
    }
}

crate::open_enum! {
    pub enum DomainModelAttributesClaimType {
        Http => "http",
        Dns => "dns",
    }
}

crate::open_enum! {
    /// Verification Status of the Domain Claim
    pub enum DomainModelAttributesClaimStatus {
        Verified => "verified",
        Deleted => "deleted",
        Unverified => "unverified",
        Superseded => "superseded",
        MissingToken => "missing_token",
    }
}

/// Claim for the domain
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DomainModelAttributesClaim {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DomainModelAttributesClaimType>,
    /// Verification Status of the Domain Claim
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DomainModelAttributesClaimStatus>,
}

/// Attributes of this object
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DomainModelAttributes {
    /// Domain Name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Claim for the domain
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<DomainModelAttributesClaim>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DomainModel {
    /// Unique identifier of the Domain
    pub id: String,
    /// Type name of this object
    pub r#type: DomainModelType,
    /// Attributes of this object
    pub attributes: DomainModelAttributes,
    pub links: LinkSelfModel,
}
