// @generated. Do not edit: change the generator or the specification.

//! The Teams surface.

pub mod api;
pub mod models;

pub use api::*;
pub use models::*;

/// Every Teams operation, grouped the way the API documents them.
///
/// Build the transport once and hand it to each surface: a second client is a second set of OAuth tokens,
/// and whichever refreshes first invalidates the other.
pub struct TeamsClient {
    client: crate::core::Client,
}

impl TeamsClient {
    pub fn new(client: crate::core::Client) -> Self {
        Self { client }
    }

    /// The transport underneath, for a request this surface does not describe.
    pub fn client(&self) -> &crate::core::Client {
        &self.client
    }

    /// The Teams operations.
    pub fn teams(&self) -> TeamsService<'_> {
        TeamsService::new(&self.client)
    }

    /// The ExternalTeams operations.
    pub fn external_teams(&self) -> ExternalTeamsService<'_> {
        ExternalTeamsService::new(&self.client)
    }

    /// The TeamMembers operations.
    pub fn team_members(&self) -> TeamMembersService<'_> {
        TeamMembersService::new(&self.client)
    }
}

impl From<crate::core::Client> for TeamsClient {
    fn from(client: crate::core::Client) -> Self {
        Self::new(client)
    }
}
