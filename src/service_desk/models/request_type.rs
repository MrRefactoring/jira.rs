// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Whether request type is restricted or not.
    pub enum RequestTypeRestrictionStatus {
        Open => "OPEN",
        Restricted => "RESTRICTED",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestType {
    /// List of items that can be expanded in the response by specifying the expand query parameter.
    #[serde(rename = "_expands", default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<Vec<String>>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
    /// Whether the user has permission to create a request with this request type.
    #[serde(rename = "canCreateRequest", default, skip_serializing_if = "Option::is_none")]
    pub can_create_request: Option<bool>,
    /// Description of the request type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<CustomerRequestCreateMeta>,
    /// List of the request type groups the request type belongs to.
    #[serde(rename = "groupIds", default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Help text for the request type.
    #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<RequestTypeIcon>,
    /// ID for the request type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ID of the issue type the request type is based upon.
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    /// Short name for the request type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID of the customer portal associated with the service desk project.
    #[serde(rename = "portalId", default, skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,
    /// The request type's practice
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub practice: Option<String>,
    /// Whether request type is restricted or not.
    #[serde(rename = "restrictionStatus", default, skip_serializing_if = "Option::is_none")]
    pub restriction_status: Option<RequestTypeRestrictionStatus>,
    /// ID of the service desk the request type belongs to.
    #[serde(rename = "serviceDeskId", default, skip_serializing_if = "Option::is_none")]
    pub service_desk_id: Option<String>,
}
