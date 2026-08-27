// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Request object to get SCIM links for an email address.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScimLinksForEmailRequest {
    /// The email address to look up SCIM links for
    pub email: String,
}
