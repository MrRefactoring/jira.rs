// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of account
    pub enum PublicUserAccountType {
        Atlassian => "atlassian",
        Customer => "customer",
        App => "app",
    }
}

crate::open_enum! {
    /// The lifecycle status of the account
    pub enum PublicUserAccountStatus {
        Active => "active",
        Partial => "partial",
        Inactive => "inactive",
        Closed => "closed",
    }
}

/// The current page of search results
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PublicUser {
    /// Unique ID of the users account. The format is \[a-zA-Z0-9_|-:\]{1,128}
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The display name of the user. Should be used for contextual rendering of the authorship in content. If the user has restricted visibility of their name, their nickname is displayed as a substitute value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The nickname of the user. Should be used for mentions or other in content references to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The type of account
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PublicUserAccountType>,
    /// The lifecycle status of the account
    #[serde(rename = "accountStatus", default, skip_serializing_if = "Option::is_none")]
    pub account_status: Option<PublicUserAccountStatus>,
    /// The email address of the user. The email will be absent for any user with an account_type of `app`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The email verification status of the user.
    #[serde(rename = "emailVerified", default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    /// The status of the user in the userbase
    #[serde(rename = "statusInUserbase", default, skip_serializing_if = "Option::is_none")]
    pub status_in_userbase: Option<bool>,
}
