// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the statuses being created and their scope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCreateRequest {
    pub scope: StatusScope,
    /// Details of the statuses being created.
    pub statuses: Vec<StatusCreate>,
}
