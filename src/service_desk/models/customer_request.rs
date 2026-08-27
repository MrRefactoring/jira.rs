// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerRequest {
    /// List of items that can be expanded in the response by specifying the expand query parameter.
    #[serde(rename = "_expands", default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<Vec<String>>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<CustomerRequestLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<CustomerRequestActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<PagedAttachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<PagedComment>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<Date>,
    #[serde(rename = "currentStatus", default, skip_serializing_if = "Option::is_none")]
    pub current_status: Option<CustomerRequestStatus>,
    /// ID of the request, as the peer issue ID.
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    /// Key of the request, as the peer issue key.
    #[serde(rename = "issueKey", default, skip_serializing_if = "Option::is_none")]
    pub issue_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<PagedUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<User>,
    /// JSON map of Jira field IDs and their values representing the content of the request. This list does not include hidden fields.
    #[serde(rename = "requestFieldValues", default, skip_serializing_if = "Option::is_none")]
    pub request_field_values: Option<Vec<CustomerRequestFieldValue>>,
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<RequestType>,
    /// ID of the request type for the request.
    #[serde(rename = "requestTypeId", default, skip_serializing_if = "Option::is_none")]
    pub request_type_id: Option<String>,
    #[serde(rename = "serviceDesk", default, skip_serializing_if = "Option::is_none")]
    pub service_desk: Option<ServiceDesk>,
    /// ID of the service desk the request belongs to.
    #[serde(rename = "serviceDeskId", default, skip_serializing_if = "Option::is_none")]
    pub service_desk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sla: Option<PagedSlaInformation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PagedCustomerRequestStatus>,
    /// Summary of the request created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
