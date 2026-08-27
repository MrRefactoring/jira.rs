// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue Bulk Transition Payload
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueBulkTransitionPayload {
    /// List of objects and each object has two properties:
    ///
    ///  *  Issues that will be bulk transitioned.
    ///  *  TransitionId that corresponds to a specific transition of issues that share the same workflow.
    #[serde(rename = "bulkTransitionInputs")]
    pub bulk_transition_inputs: Vec<BulkTransitionSubmitInput>,
    /// A boolean value that indicates whether to send a bulk change notification when the issues are being transitioned.
    ///
    /// If `true`, dispatches a bulk notification email to users about the updates.
    #[serde(rename = "sendBulkNotification", default, skip_serializing_if = "Option::is_none")]
    pub send_bulk_notification: Option<bool>,
}
