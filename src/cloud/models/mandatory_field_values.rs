// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Can contain multiple field values of following types depending on `type` key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MandatoryFieldValues {
    MandatoryFieldValue(MandatoryFieldValue),
    MandatoryFieldValueForADF(MandatoryFieldValueForADF),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}
