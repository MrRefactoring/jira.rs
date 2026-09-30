// @generated. Do not edit: change the generator or the specification.

//! The ServiceDesk surface.

pub mod api;
pub mod models;

pub use api::*;
pub use models::*;

/// Every ServiceDesk operation, grouped the way the API documents them.
///
/// Build the transport once and hand it to each surface: a second client is a second set of OAuth tokens,
/// and whichever refreshes first invalidates the other.
pub struct ServiceDeskClient {
    client: crate::core::Client,
}

impl ServiceDeskClient {
    pub fn new(client: crate::core::Client) -> Self {
        Self { client }
    }

    /// The transport underneath, for a request this surface does not describe.
    pub fn client(&self) -> &crate::core::Client {
        &self.client
    }

    /// The Assets operations.
    pub fn assets(&self) -> AssetsService<'_> {
        AssetsService::new(&self.client)
    }

    /// The Customer operations.
    pub fn customer(&self) -> CustomerService<'_> {
        CustomerService::new(&self.client)
    }

    /// The Info operations.
    pub fn info(&self) -> InfoService<'_> {
        InfoService::new(&self.client)
    }

    /// The Knowledgebase operations.
    pub fn knowledgebase(&self) -> KnowledgebaseService<'_> {
        KnowledgebaseService::new(&self.client)
    }

    /// The Organization operations.
    pub fn organization(&self) -> OrganizationService<'_> {
        OrganizationService::new(&self.client)
    }

    /// The Request operations.
    pub fn request(&self) -> RequestService<'_> {
        RequestService::new(&self.client)
    }

    /// The Servicedesk operations.
    pub fn servicedesk(&self) -> ServicedeskService<'_> {
        ServicedeskService::new(&self.client)
    }
}

impl From<crate::core::Client> for ServiceDeskClient {
    fn from(client: crate::core::Client) -> Self {
        Self::new(client)
    }
}
