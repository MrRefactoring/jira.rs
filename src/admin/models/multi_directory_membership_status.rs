// @generated. Do not edit: change the generator or the specification.

crate::open_enum! {
    /// The user's membership status in the directory mapped to this resource.
    ///   - `active` - The user has an active membership in the directory.
    ///   - `suspended` - The user is suspended in the directory.
    ///   - `no_membership` - The user has no membership in the directory.
    pub enum MultiDirectoryMembershipStatus {
        Active => "active",
        Suspended => "suspended",
        NoMembership => "no_membership",
    }
}
