// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Status category the status belongs to.
    pub enum CustomerRequestStatusStatusCategory {
        Undefined => "UNDEFINED",
        New => "NEW",
        Indeterminate => "INDETERMINATE",
        Done => "DONE",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerRequestStatus {
    /// Name of the status condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Status category the status belongs to.
    #[serde(rename = "statusCategory", default, skip_serializing_if = "Option::is_none")]
    pub status_category: Option<CustomerRequestStatusStatusCategory>,
    #[serde(rename = "statusDate", default, skip_serializing_if = "Option::is_none")]
    pub status_date: Option<Date>,
}
