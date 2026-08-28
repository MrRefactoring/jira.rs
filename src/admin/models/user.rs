// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of account
    pub enum UserAccountType {
        Atlassian => "atlassian",
        Customer => "customer",
        App => "app",
    }
}

crate::open_enum! {
    /// The lifecycle status of the account
    pub enum UserAccountStatus {
        Active => "active",
        Inactive => "inactive",
        Closed => "closed",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// Unique ID of the users account. The format is \[a-zA-Z0-9_|-:\]{1,128}
    pub account_id: String,
    /// The type of account
    pub account_type: UserAccountType,
    /// The lifecycle status of the account
    pub account_status: UserAccountStatus,
    /// The display name of the user. Should be used for contextual rendering of the authorship in content. If the user has restricted visibility of their name, their nickname will be displayed as a substitute value
    pub name: String,
    /// The absolute URI (RFC3986) to the avatar name of the user. Should be used for contextual rendering of the authorship in content. If the user has restricted visibility of their avatar, an alternative URI will be provided as a substitute value
    pub picture: String,
    /// The email address of the user. If the user has restricted visibility of the email address, the property will be absent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Billable status of User in Atlassian Guard Standard
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_billable: Option<bool>,
    /// Last active date for a user
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    /// Last active date for a user
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub last_active: Option<String>,
    /// Products which the User is using
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_access: Option<Vec<Product>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkSelfModel>,
}
