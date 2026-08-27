// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum RedactionJobStatusResponseJobStatus {
        Pending => "PENDING",
        InProgress => "IN_PROGRESS",
        Completed => "COMPLETED",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RedactionJobStatusResponse {
    #[serde(rename = "bulkRedactionResponse", default, skip_serializing_if = "Option::is_none")]
    pub bulk_redaction_response: Option<BulkRedactionResponse>,
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<RedactionJobStatusResponseJobStatus>,
}
