// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An object that is used to find the total count of objects returned for a given AQL query
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectAQLTotalCountParams {
    /// The AQL that will filter the objects.
    #[serde(rename = "qlQuery")]
    pub ql_query: String,
}
