// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ExternalReferenceSource {
        AtlassianGroup => "ATLASSIAN_GROUP",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalReference {
    pub id: String,
    pub source: ExternalReferenceSource,
}
