// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used. Both iconUrl and avatarId cannot be defined.
    pub enum UpdatePriorityDetailsIconUrl {
        ImagesIconsPrioritiesBlockerPng => "/images/icons/priorities/blocker.png",
        ImagesIconsPrioritiesCriticalPng => "/images/icons/priorities/critical.png",
        ImagesIconsPrioritiesHighPng => "/images/icons/priorities/high.png",
        ImagesIconsPrioritiesHighestPng => "/images/icons/priorities/highest.png",
        ImagesIconsPrioritiesLowPng => "/images/icons/priorities/low.png",
        ImagesIconsPrioritiesLowestPng => "/images/icons/priorities/lowest.png",
        ImagesIconsPrioritiesMajorPng => "/images/icons/priorities/major.png",
        ImagesIconsPrioritiesMediumPng => "/images/icons/priorities/medium.png",
        ImagesIconsPrioritiesMinorPng => "/images/icons/priorities/minor.png",
        ImagesIconsPrioritiesTrivialPng => "/images/icons/priorities/trivial.png",
        ImagesIconsPrioritiesBlockerNewPng => "/images/icons/priorities/blocker_new.png",
        ImagesIconsPrioritiesCriticalNewPng => "/images/icons/priorities/critical_new.png",
        ImagesIconsPrioritiesHighNewPng => "/images/icons/priorities/high_new.png",
        ImagesIconsPrioritiesHighestNewPng => "/images/icons/priorities/highest_new.png",
        ImagesIconsPrioritiesLowNewPng => "/images/icons/priorities/low_new.png",
        ImagesIconsPrioritiesLowestNewPng => "/images/icons/priorities/lowest_new.png",
        ImagesIconsPrioritiesMajorNewPng => "/images/icons/priorities/major_new.png",
        ImagesIconsPrioritiesMediumNewPng => "/images/icons/priorities/medium_new.png",
        ImagesIconsPrioritiesMinorNewPng => "/images/icons/priorities/minor_new.png",
        ImagesIconsPrioritiesTrivialNewPng => "/images/icons/priorities/trivial_new.png",
    }
}

/// Details of an issue priority.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdatePriorityDetails {
    /// The ID for the avatar for the priority. This parameter is nullable and both iconUrl and avatarId cannot be defined.
    #[serde(rename = "avatarId", default, skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of an icon for the priority. Accepted protocols are HTTP and HTTPS. Built in icons can also be used. Both iconUrl and avatarId cannot be defined.
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<UpdatePriorityDetailsIconUrl>,
    /// The name of the priority. Must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status color of the priority in 3-digit or 6-digit hexadecimal format.
    #[serde(rename = "statusColor", default, skip_serializing_if = "Option::is_none")]
    pub status_color: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
