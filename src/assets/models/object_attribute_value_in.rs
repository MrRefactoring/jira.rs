// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The input attribute values
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectAttributeValueIn {
    /// | Type (of the object type attribute)        | Description |
    /// | ------------------------------------------ | ----------- |
    /// | Default     | The value must be of a valid format based on the additional type of the object type attribute, like Text, Integer, URL, Email etc. for date and datetime the value should be in ISO8601 format|
    /// | Object      | The value is the Object Key to set |
    /// | User        | The value is the Jira User key to set |
    /// | Group       | The value is the Jira Group key to set |
    /// | Status      | The value is the Status ID in Assets |
    pub value: String,
}
