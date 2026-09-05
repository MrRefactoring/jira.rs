// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The data classification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataClassificationLevels {
    /// The data classifications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<DataClassificationTag>>,
}
