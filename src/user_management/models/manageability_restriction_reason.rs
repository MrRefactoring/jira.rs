// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The property or action is restricted because:
    ///
    /// - _administrative_: The property or action is restricted
    ///   because it is intended exclusively for administrative use
    /// - _administrative.notMyself_: The property or action
    ///   is restricted because it is intended for administrative use and
    ///   is forbidden for self-use.
    /// - _authPolicy.saml_: The property is restricted as it is set on login by SAML
    /// - _blocked.exportControl_: The property/action is restricted because
    ///   the user is blocked by US export control
    /// - _externalDirectory.scim_: The property/action is restricted because
    ///   the user is managed by an external SCIM directory
    /// - _externalDirectory.google_: The property/action is restricted because
    ///   the user is managed by an external Google directory
    /// - _myselfOnly_: The property or action is restricted because it is
    ///   available only to the user which the account belongs to
    /// - _managedAccount_: The property or action is restricted because it is
    ///   available only to the user's organisation administrator
    pub enum ManageabilityRestrictionReasonKey {
        Administrative => "administrative",
        AdministrativeNotMyself => "administrative.notMyself",
        AuthPolicySaml => "authPolicy.saml",
        BlockedExportControl => "blocked.exportControl",
        ExternalDirectoryScim => "externalDirectory.scim",
        ExternalDirectoryGoogle => "externalDirectory.google",
        MyselfOnly => "myselfOnly",
        ManagedAccount => "managedAccount",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ManageabilityRestrictionReason {
    /// The property or action is restricted because:
    ///
    /// - _administrative_: The property or action is restricted
    ///   because it is intended exclusively for administrative use
    /// - _administrative.notMyself_: The property or action
    ///   is restricted because it is intended for administrative use and
    ///   is forbidden for self-use.
    /// - _authPolicy.saml_: The property is restricted as it is set on login by SAML
    /// - _blocked.exportControl_: The property/action is restricted because
    ///   the user is blocked by US export control
    /// - _externalDirectory.scim_: The property/action is restricted because
    ///   the user is managed by an external SCIM directory
    /// - _externalDirectory.google_: The property/action is restricted because
    ///   the user is managed by an external Google directory
    /// - _myselfOnly_: The property or action is restricted because it is
    ///   available only to the user which the account belongs to
    /// - _managedAccount_: The property or action is restricted because it is
    ///   available only to the user's organisation administrator
    pub key: ManageabilityRestrictionReasonKey,
}
