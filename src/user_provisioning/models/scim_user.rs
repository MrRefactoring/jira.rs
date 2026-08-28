// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// SCIM user
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimUser {
    /// SCIM schemas that define the attributes present in the current JSON structure This is a required field during user creation or modification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// Unique identifier defined by the provisioning client. Atlassian SCIM service will verify  the value and guarantee its uniqueness. This is a required field during  user creation or modification.
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// Email addresses of the User. This is a required field during user creation or modification.  One value must be marked as primary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<ScimUserEmail>>,
    /// Unique identifier defined by Atlassian SCIM Service. CaseExact. This is a read-only field and will be disregarded if included in the payload during user creation or modification..
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Identifier defined by provisioning client. This is a case-sensitive field. Uniqueness is  controlled by client.
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ScimUserName>,
    /// User's display name.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// User's nickname.
    #[serde(rename = "nickName", default, skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// User's title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// User's preferred language.
    #[serde(rename = "preferredLanguage", default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    /// User's department.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// User's organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// User's timezone. e.g. America/Los_Angeles .
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Phone numbers of the user.
    #[serde(rename = "phoneNumbers", default, skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<ScimUserPhoneNumber>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ScimMetadata>,
    /// Groups to which the user is associated in SCIM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ScimGroupForUser>>,
    #[serde(
        rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub urn_ietf_params_scim_schemas_extension_enterprise_2_0_user: Option<EnterpriseUserExtension>,
    #[serde(
        rename = "urn:scim:schemas:extension:atlassian-external:1.0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub urn_scim_schemas_extension_atlassian_external_1_0: Option<ExternalAtlassianScimExtension>,
    /// A boolean value indicating the user's administrative status. This value will default to true if not provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
