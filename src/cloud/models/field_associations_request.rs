// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of field associations with projects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldAssociationsRequest {
    /// Contexts to associate/unassociate the fields with.
    #[serde(rename = "associationContexts")]
    pub association_contexts: Vec<AssociationContextObject>,
    /// Fields to associate/unassociate with projects.
    pub fields: Vec<FieldIdentifierObject>,
}
