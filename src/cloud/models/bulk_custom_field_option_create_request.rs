// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the options to create for a custom field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkCustomFieldOptionCreateRequest {
    /// Details of options to create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOptionCreate>>,
}
