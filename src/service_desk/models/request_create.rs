// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestCreate {
    /// (Experimental) Shows extra information for the request channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<Form>,
    /// (Experimental) Whether to accept rich text fields in Atlassian Document Format (ADF).
    #[serde(rename = "isAdfRequest", default, skip_serializing_if = "Option::is_none")]
    pub is_adf_request: Option<bool>,
    /// The `accountId` of the customer that the request is being raised on behalf of.
    #[serde(rename = "raiseOnBehalfOf", default, skip_serializing_if = "Option::is_none")]
    pub raise_on_behalf_of: Option<String>,
    /// JSON map of Jira field IDs and their values representing the content of the request.
    #[serde(rename = "requestFieldValues", default, skip_serializing_if = "Option::is_none")]
    pub request_field_values: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// List of customers to participate in the request, as a list of `accountId` values.
    #[serde(rename = "requestParticipants", default, skip_serializing_if = "Option::is_none")]
    pub request_participants: Option<Vec<String>>,
    /// ID of the request type for the request.
    #[serde(rename = "requestTypeId", default, skip_serializing_if = "Option::is_none")]
    pub request_type_id: Option<String>,
    /// ID of the service desk in which to create the request.
    #[serde(rename = "serviceDeskId", default, skip_serializing_if = "Option::is_none")]
    pub service_desk_id: Option<String>,
}
