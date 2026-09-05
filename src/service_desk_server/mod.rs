// @generated. Do not edit: change the generator or the specification.

//! The ServiceDeskServer surface.

pub mod api;
pub mod models;

pub use api::*;
pub use models::*;

/// Every ServiceDeskServer operation, grouped the way the API documents them.
///
/// Build the transport once and hand it to each surface: a second client is a second set of OAuth tokens,
/// and whichever refreshes first invalidates the other.
pub struct ServiceDeskServerClient {
    client: crate::core::Client,
}

impl ServiceDeskServerClient {
    pub fn new(client: crate::core::Client) -> Self {
        Self { client }
    }

    /// The transport underneath, for a request this surface does not describe.
    pub fn client(&self) -> &crate::core::Client {
        &self.client
    }

    /// The Approvals operations.
    pub fn approvals(&self) -> ApprovalsService<'_> {
        ApprovalsService::new(&self.client)
    }

    /// The RequestAttachments operations.
    pub fn request_attachments(&self) -> RequestAttachmentsService<'_> {
        RequestAttachmentsService::new(&self.client)
    }

    /// The CustomerRequests operations.
    pub fn customer_requests(&self) -> CustomerRequestsService<'_> {
        CustomerRequestsService::new(&self.client)
    }

    /// The Customers operations.
    pub fn customers(&self) -> CustomersService<'_> {
        CustomersService::new(&self.client)
    }

    /// The CustomerTransitions operations.
    pub fn customer_transitions(&self) -> CustomerTransitionsService<'_> {
        CustomerTransitionsService::new(&self.client)
    }

    /// The Organizations operations.
    pub fn organizations(&self) -> OrganizationsService<'_> {
        OrganizationsService::new(&self.client)
    }

    /// The ServiceDeskOrganizations operations.
    pub fn service_desk_organizations(&self) -> ServiceDeskOrganizationsService<'_> {
        ServiceDeskOrganizationsService::new(&self.client)
    }

    /// The Portals operations.
    pub fn portals(&self) -> PortalsService<'_> {
        PortalsService::new(&self.client)
    }

    /// The Queues operations.
    pub fn queues(&self) -> QueuesService<'_> {
        QueuesService::new(&self.client)
    }

    /// The RequestTypes operations.
    pub fn request_types(&self) -> RequestTypesService<'_> {
        RequestTypesService::new(&self.client)
    }

    /// The RequestTypePermissions operations.
    pub fn request_type_permissions(&self) -> RequestTypePermissionsService<'_> {
        RequestTypePermissionsService::new(&self.client)
    }

    /// The ServiceDesks operations.
    pub fn service_desks(&self) -> ServiceDesksService<'_> {
        ServiceDesksService::new(&self.client)
    }

    /// The Info operations.
    pub fn info(&self) -> InfoService<'_> {
        InfoService::new(&self.client)
    }

    /// The QueueSettings operations.
    pub fn queue_settings(&self) -> QueueSettingsService<'_> {
        QueueSettingsService::new(&self.client)
    }
}

impl From<crate::core::Client> for ServiceDeskServerClient {
    fn from(client: crate::core::Client) -> Self {
        Self::new(client)
    }
}
