// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Defines the association type.
    pub enum IssueIdOrKeysAssociationAssociationType {
        IssueKeys => "issueKeys",
        IssueIdOrKeys => "issueIdOrKeys",
    }
}

/// An association type referencing issues in Jira.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueIdOrKeysAssociation {
    /// Defines the association type.
    #[serde(rename = "associationType")]
    pub association_type: IssueIdOrKeysAssociationAssociationType,
    /// The Jira issue keys or IDs to associate the entity with.
    ///
    /// The number of values counted across all associationTypes must not exceed a limit of 500.
    pub values: Vec<String>,
}
