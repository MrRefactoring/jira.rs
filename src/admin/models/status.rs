// @generated. Do not edit: change the generator or the specification.

crate::open_enum! {
    /// The status for the user account. This status is a composite of `accountStatus` and `membershipStatus`.
    ///   - `active` - `accountStatus` is `active` and `membershipStatus` is `active`.
    ///   - `suspended` - `accountStatus` is `active` and `membershipStatus` is `suspended`.
    ///   - `not_invited` - `accountStatus` is `active` and `membershipStatus` is `no_membership`.
    ///   - `deactivated` - `accountStatus` is `inactive`.
    ///   - `for_deletion` - Indicates whether or not a managed account is scheduled for deletion.
    pub enum Status {
        Active => "active",
        Suspended => "suspended",
        NotInvited => "not_invited",
        Deactivated => "deactivated",
        ForDeletion => "for_deletion",
    }
}
