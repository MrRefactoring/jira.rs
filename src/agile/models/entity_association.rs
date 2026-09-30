// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Defines the association type. Currently supported entities can be found in this field's value enums list.
    pub enum EntityAssociationAssociationType {
        Commit => "commit",
        Repository => "repository",
    }
}

/// Identifies an individual commit in a repository.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityAssociationValuesValue {
    /// The hash for the Commit.
    #[serde(rename = "commitHash")]
    pub commit_hash: String,
    /// The ID of the Repository that the Commit belongs to.
    #[serde(rename = "repositoryId")]
    pub repository_id: String,
}

/// Identifies an individual repository.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityAssociationValuesValue2 {
    #[serde(rename = "repositoryId")]
    pub repository_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EntityAssociationValues {
    Variant0(EntityAssociationValuesValue),
    Variant1(EntityAssociationValuesValue2),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// An association type referencing another entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityAssociation {
    /// Defines the association type. Currently supported entities can be found in this field's value enums list.
    #[serde(rename = "associationType")]
    pub association_type: EntityAssociationAssociationType,
    /// The entity keys that represent the entities to be associated.
    /// The number of values counted across all associationTypes must not exceed a limit of 500.
    pub values: Vec<EntityAssociationValues>,
}
