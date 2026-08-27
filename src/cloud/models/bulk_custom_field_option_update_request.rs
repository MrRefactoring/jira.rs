// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the options to update for a custom field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkCustomFieldOptionUpdateRequest {
    /// Details of the options to update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOptionUpdate>>,
}
