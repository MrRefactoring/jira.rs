// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerRequest {
    #[serde(rename = "_expands", default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<Vec<String>>,
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "issueKey", default, skip_serializing_if = "Option::is_none")]
    pub issue_key: Option<String>,
    #[serde(rename = "requestTypeId", default, skip_serializing_if = "Option::is_none")]
    pub request_type_id: Option<String>,
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<RequestType>,
    #[serde(rename = "serviceDeskId", default, skip_serializing_if = "Option::is_none")]
    pub service_desk_id: Option<String>,
    #[serde(rename = "serviceDesk", default, skip_serializing_if = "Option::is_none")]
    pub service_desk: Option<ServiceDesk>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<Date>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporter: Option<User>,
    #[serde(rename = "requestFieldValues", default, skip_serializing_if = "Option::is_none")]
    pub request_field_values: Option<Vec<CustomerRequestFieldValue>>,
    #[serde(rename = "currentStatus", default, skip_serializing_if = "Option::is_none")]
    pub current_status: Option<CustomerRequestStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PagedCustomerRequestStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<PagedUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sla: Option<PagedSlaInformation>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<CustomerRequestLink>,
}
