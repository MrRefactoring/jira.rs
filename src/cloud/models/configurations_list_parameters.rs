// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// List of custom fields identifiers which will be used to filter configurations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigurationsListParameters {
    /// List of IDs or keys of the custom fields. It can be a mix of IDs and keys in the same query.
    #[serde(rename = "fieldIdsOrKeys")]
    pub field_ids_or_keys: Vec<String>,
}
