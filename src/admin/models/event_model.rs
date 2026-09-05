// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Type name of this object
    pub enum EventModelType {
        Events => "events",
    }
}

/// Attributes of this object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventModelAttributes {
    /// The date and time of the event
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time of the event
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub time: String,
    /// Kind of Event. Complete list see `event-actions` API.
    pub action: String,
    pub actor: EventActorModel,
    /// Describes one or more entities that the action was performed against. This field describes the "what" of the event.
    pub context: Vec<EventObjectModel>,
    /// Describes the location where the action was performed. This field describes the "where" of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<Vec<EventObjectModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<EventLocationModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventModel {
    /// Unique identifier of the Event
    pub id: String,
    /// Type name of this object
    pub r#type: EventModelType,
    /// Attributes of this object
    pub attributes: EventModelAttributes,
    pub links: LinkSelfModel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<EventMessageModel>,
}
