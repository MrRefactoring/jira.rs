// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationRole {
    #[serde(rename = "defaultGroups", default, skip_serializing_if = "Option::is_none")]
    pub default_groups: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defined: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "hasUnlimitedSeats", default, skip_serializing_if = "Option::is_none")]
    pub has_unlimited_seats: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfSeats", default, skip_serializing_if = "Option::is_none")]
    pub number_of_seats: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<bool>,
    #[serde(rename = "remainingSeats", default, skip_serializing_if = "Option::is_none")]
    pub remaining_seats: Option<i64>,
    #[serde(rename = "selectedByDefault", default, skip_serializing_if = "Option::is_none")]
    pub selected_by_default: Option<bool>,
    #[serde(rename = "userCount", default, skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
    #[serde(rename = "userCountDescription", default, skip_serializing_if = "Option::is_none")]
    pub user_count_description: Option<String>,
}
