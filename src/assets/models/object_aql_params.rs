// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An object that is used to find a paginated result set based on an AQL query
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectAQLParams {
    /// The AQL that will fetch the objects.
    #[serde(rename = "qlQuery")]
    pub ql_query: String,
}
