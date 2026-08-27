// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// List issues archived within a specified date range.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateRangeFilterRequest {
    /// List issues archived after a specified date, passed in the YYYY-MM-DD format.
    #[serde(rename = "dateAfter")]
    pub date_after: String,
    /// List issues archived before a specified date provided in the YYYY-MM-DD format.
    #[serde(rename = "dateBefore")]
    pub date_before: String,
}
