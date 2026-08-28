// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraDurationField {
    #[serde(rename = "originalEstimateField")]
    pub original_estimate_field: String,
}
