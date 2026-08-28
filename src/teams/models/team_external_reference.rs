// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum TeamExternalReferenceSource {
        AtlassianGroup => "ATLASSIAN_GROUP",
        Hris => "HRIS",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TeamExternalReference {
    pub id: String,
    pub source: TeamExternalReferenceSource,
}
