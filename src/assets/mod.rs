// @generated. Do not edit: change the generator or the specification.

//! The Assets surface.

pub mod api;
pub mod models;

pub use api::*;
pub use models::*;

/// Every Assets operation, grouped the way the API documents them.
///
/// Build the transport once and hand it to each surface: a second client is a second set of OAuth tokens,
/// and whichever refreshes first invalidates the other.
pub struct AssetsClient {
    client: crate::core::Client,
}

impl AssetsClient {
    pub fn new(client: crate::core::Client) -> Self {
        Self { client }
    }

    /// The transport underneath, for a request this surface does not describe.
    pub fn client(&self) -> &crate::core::Client {
        &self.client
    }

    /// The Icons operations.
    pub fn icons(&self) -> IconsService<'_> {
        IconsService::new(&self.client)
    }

    /// The Imports operations.
    pub fn imports(&self) -> ImportsService<'_> {
        ImportsService::new(&self.client)
    }

    /// The ImportSources operations.
    pub fn import_sources(&self) -> ImportSourcesService<'_> {
        ImportSourcesService::new(&self.client)
    }

    /// The Objects operations.
    pub fn objects(&self) -> ObjectsService<'_> {
        ObjectsService::new(&self.client)
    }

    /// The ConnectedTickets operations.
    pub fn connected_tickets(&self) -> ConnectedTicketsService<'_> {
        ConnectedTicketsService::new(&self.client)
    }

    /// The ObjectSchemas operations.
    pub fn object_schemas(&self) -> ObjectSchemasService<'_> {
        ObjectSchemasService::new(&self.client)
    }

    /// The ObjectTypes operations.
    pub fn object_types(&self) -> ObjectTypesService<'_> {
        ObjectTypesService::new(&self.client)
    }

    /// The ObjectTypeAttributes operations.
    pub fn object_type_attributes(&self) -> ObjectTypeAttributesService<'_> {
        ObjectTypeAttributesService::new(&self.client)
    }

    /// The Progress operations.
    pub fn progress(&self) -> ProgressService<'_> {
        ProgressService::new(&self.client)
    }

    /// The StatusTypes operations.
    pub fn status_types(&self) -> StatusTypesService<'_> {
        StatusTypesService::new(&self.client)
    }

    /// The ReferenceTypes operations.
    pub fn reference_types(&self) -> ReferenceTypesService<'_> {
        ReferenceTypesService::new(&self.client)
    }

    /// The GlobalConfig operations.
    pub fn global_config(&self) -> GlobalConfigService<'_> {
        GlobalConfigService::new(&self.client)
    }

    /// The Usage operations.
    pub fn usage(&self) -> UsageService<'_> {
        UsageService::new(&self.client)
    }
}

impl From<crate::core::Client> for AssetsClient {
    fn from(client: crate::core::Client) -> Self {
        Self::new(client)
    }
}
