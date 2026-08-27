// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The PostIncidentReviewData schema version used for this post-incident review data.
    ///
    /// Placeholder to support potential schema changes in the future.
    pub enum GetReviewByIdSchemaVersion {
        N10 => "1.0",
    }
}

crate::open_enum! {
    /// The current status of the Post-Incident Review.
    pub enum GetReviewByIdStatus {
        InProgress => "in progress",
        OutstandingActions => "outstanding actions",
        Completed => "completed",
        Unknown => "unknown",
    }
}

crate::open_enum! {
    /// the type of the association being made
    pub enum GetReviewByIdAssociationsAssociationType {
        IssueIdOrKeys => "issueIdOrKeys",
        ServiceIdOrKeys => "serviceIdOrKeys",
        AtiCloudCompassEventSource => "ati:cloud:compass:event-source",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetReviewByIdAssociations {
    /// the type of the association being made
    #[serde(rename = "associationType", default, skip_serializing_if = "Option::is_none")]
    pub association_type: Option<GetReviewByIdAssociationsAssociationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Data related to a specific post-incident review. Must specify at least one association to an incident.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReviewById {
    /// The PostIncidentReviewData schema version used for this post-incident review data.
    ///
    /// Placeholder to support potential schema changes in the future.
    #[serde(rename = "schemaVersion")]
    pub schema_version: GetReviewByIdSchemaVersion,
    /// The identifier for the Review. Must be unique for a given Provider.
    pub id: String,
    /// An ID used to apply an ordering to updates for this Review in the case of out-of-order receipt of update requests.
    ///
    /// This can be any monotonically increasing number. A suggested implementation is to use epoch millis from the Provider system, but other alternatives are valid (e.g. a Provider could store a counter against each Review and increment that on each update to Jira).
    ///
    /// Updates for a Review that are received with an updateSqeuenceId lower than what is currently stored will be ignored.
    #[serde(rename = "updateSequenceNumber")]
    pub update_sequence_number: i64,
    /// The IDs of the Incidents covered by this Review. Must be unique for a given Provider.
    pub reviews: Vec<String>,
    /// The human-readable summary for the Post-Incident Review. Will be shown in the UI.
    ///
    /// If not provided, will use the ID for display.
    pub summary: String,
    /// A description of the review in Markdown format. Will be shown in the UI and used when creating Jira Issues.
    pub description: String,
    /// A URL users can use to link to a summary view of this review, if appropriate.
    ///
    /// This could be any location that makes sense in the Provider system (e.g. if the summary information comes from a specific project, it might make sense to link the user to the review in that project).
    pub url: String,
    /// The timestamp to present to the user that shows when the Review was raised.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[serde(rename = "createdDate", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub created_date: String,
    /// The last-updated timestamp to present to the user the last time the Review was updated.
    ///
    /// Expected format is an RFC3339 formatted string.
    #[serde(rename = "lastUpdated", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub last_updated: String,
    /// The current status of the Post-Incident Review.
    pub status: GetReviewByIdStatus,
    /// The IDs of the Jira issues related to this Incident. Must be unique for a given Provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<GetReviewByIdAssociations>>,
}
