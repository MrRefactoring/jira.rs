// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Container for a list of audit records.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditRecords {
    /// The requested or default limit on the number of audit items to be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The number of audit items skipped before the first item in this list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// The list of audit items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<AuditRecord>>,
    /// The total number of audit items returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
