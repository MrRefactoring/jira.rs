// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ConnectedTickets operations.
pub struct ConnectedTicketsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ConnectedTicketsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Relation between Jira issues and Assets objects
    pub fn find_object_tickets(&self, object_id: impl Into<String>) -> FindObjectTicketsRequest<'a> {
        FindObjectTicketsRequest::new(self.client, object_id)
    }
}

/// Relation between Jira issues and Assets objects
pub struct FindObjectTicketsRequest<'a> {
    client: &'a crate::core::Client,
    object_id: String,
}

impl<'a> FindObjectTicketsRequest<'a> {
    fn new(client: &'a crate::core::Client, object_id: impl Into<String>) -> Self {
        Self { client, object_id: object_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/objectconnectedtickets/{}/tickets", crate::core::encode_path_segment(&self.object_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Tickets> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
