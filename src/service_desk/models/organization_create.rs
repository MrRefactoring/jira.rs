// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationCreate {
    /// Name of the organization. Must contain 1-200 characters.
    pub name: String,
}
