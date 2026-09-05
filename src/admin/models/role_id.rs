// @generated. Do not edit: change the generator or the specification.

crate::open_enum! {
    /// The list of role IDs. The Atlassian canonical roles are used to determine the permissions of the user against resources within  the organization. The allowed roles are:
    ///    - `atlassian/user` - Can access the product, with no product admin permissions
    ///    - `atlassian/admin` - Can access the product, with product admin permissions
    ///    - `atlassian/guest` - Can only access one space you or space admins specify
    ///    - `atlassian/customer` - (Jira Service Management) Can visit help center, submit help requests, and view articles (non-billable)
    ///    - `atlassian/user-access-admin` - No product access. Can administer users and groups for this product in Atlassian Administration
    ///    - `atlassian/contributor` - Can access the product to view, comment, and vote only (non-billable)
    ///    - `atlassian/basic` - Can access basic product features, with no product admin permissions (non-billable)
    ///    - `atlassian/stakeholder` - Can receive incident updates and has the same product access as Customer. Non-billable but available only on Premium and Enterprise plans
    ///    - `atlassian/org-admin` - An organization admin is the highest level of admin and can complete any administrative task in Atlassian Administration
    ///    - `atlassian/site-admin` - Site admins can access Atlassian Administration and complete tasks related to the specific site they are administering.
    ///    - `atlassian/ai-access` - Can use AI features in AI-enabled apps they have access to.
    pub enum RoleId {
        AtlassianUser => "atlassian/user",
        AtlassianAdmin => "atlassian/admin",
        AtlassianGuest => "atlassian/guest",
        AtlassianCustomer => "atlassian/customer",
        AtlassianUserAccessAdmin => "atlassian/user-access-admin",
        AtlassianContributor => "atlassian/contributor",
        AtlassianBasic => "atlassian/basic",
        AtlassianStakeholder => "atlassian/stakeholder",
        AtlassianOrgAdmin => "atlassian/org-admin",
        AtlassianSiteAdmin => "atlassian/site-admin",
        AtlassianAiAccess => "atlassian/ai-access",
    }
}
