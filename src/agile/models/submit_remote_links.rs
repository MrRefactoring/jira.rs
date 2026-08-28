// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SubmitRemoteLinksUnknownAssociations {
    IssueIdOrKeysAssociation(IssueIdOrKeysAssociation),
    ServiceIdOrKeysAssociation(ServiceIdOrKeysAssociation),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The result of a successful `submitRemoteLinks` request.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitRemoteLinks {
    /// The IDs of Remote Links that have been accepted for submission.
    ///
    /// A Remote Link may be rejected if it was only associated with unknown issue keys, unknown service IDs, or if
    /// the submitted data for that Remote Link does not match the required schema.
    ///
    /// Note that a Remote Link that isn't updated due to it's `updateSequenceNumber` being out of order is not
    /// considered a failed submission.
    #[serde(rename = "acceptedRemoteLinks", default, skip_serializing_if = "Option::is_none")]
    pub accepted_remote_links: Option<Vec<String>>,
    /// Details of Remote Links that have not been accepted for submission, usually due to a problem with the request data.
    ///
    /// A Remote Link may be rejected if it was only associated with unknown issue keys, unknown service IDs, or
    /// if the submitted data for the Remote Link does not match the required schema.
    ///
    /// The object (if present) will be keyed by Remote Link ID and include any errors associated with that
    /// Remote Link that have prevented it being submitted.
    #[serde(rename = "rejectedRemoteLinks", default, skip_serializing_if = "Option::is_none")]
    pub rejected_remote_links: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Issue keys or services IDs or keys that are not known on this Jira instance (if any).
    #[serde(rename = "unknownAssociations", default, skip_serializing_if = "Option::is_none")]
    pub unknown_associations: Option<Vec<SubmitRemoteLinksUnknownAssociations>>,
}
