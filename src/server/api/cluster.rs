// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Cluster operations.
pub struct ClusterService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ClusterService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Delete the node from the cluster if state of node is OFFLINE.
    pub fn delete_node(&self, node_id: impl Into<String>) -> DeleteNodeRequest<'a> {
        DeleteNodeRequest::new(self.client, node_id)
    }

    /// Change the node's state to offline if the node is reporting as active, but is not alive.
    pub fn change_node_state_to_offline(&self, node_id: impl Into<String>) -> ChangeNodeStateToOfflineRequest<'a> {
        ChangeNodeStateToOfflineRequest::new(self.client, node_id)
    }

    /// Returns all nodes in cluster.
    pub fn get_all_nodes(&self) -> GetAllNodesRequest<'a> {
        GetAllNodesRequest::new(self.client)
    }

    /// Approves the cluster upgrade.
    pub fn approve_upgrade(&self) -> ApproveUpgradeRequest<'a> {
        ApproveUpgradeRequest::new(self.client)
    }

    /// Cancels the ongoing cluster upgrade.
    pub fn cancel_upgrade(&self) -> CancelUpgradeRequest<'a> {
        CancelUpgradeRequest::new(self.client)
    }

    /// Retries the cluster upgrade.
    pub fn acknowledge_errors(&self) -> AcknowledgeErrorsRequest<'a> {
        AcknowledgeErrorsRequest::new(self.client)
    }

    /// Starts the cluster upgrade.
    pub fn set_ready_to_upgrade(&self) -> SetReadyToUpgradeRequest<'a> {
        SetReadyToUpgradeRequest::new(self.client)
    }

    /// Returns the current state of the cluster upgrade.
    pub fn get_state(&self) -> GetStateRequest<'a> {
        GetStateRequest::new(self.client)
    }
}

/// Delete the node from the cluster if state of node is OFFLINE.
#[derive(Clone)]
pub struct DeleteNodeRequest<'a> {
    client: &'a crate::core::Client,
    node_id: String,
}

impl<'a> DeleteNodeRequest<'a> {
    fn new(client: &'a crate::core::Client, node_id: impl Into<String>) -> Self {
        Self { client, node_id: node_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/cluster/node/{}", crate::core::encode_path_segment(&self.node_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Change the node's state to offline if the node is reporting as active, but is not alive.
#[derive(Clone)]
pub struct ChangeNodeStateToOfflineRequest<'a> {
    client: &'a crate::core::Client,
    node_id: String,
}

impl<'a> ChangeNodeStateToOfflineRequest<'a> {
    fn new(client: &'a crate::core::Client, node_id: impl Into<String>) -> Self {
        Self { client, node_id: node_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/cluster/node/{}/offline", crate::core::encode_path_segment(&self.node_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all nodes in cluster.
#[derive(Clone)]
pub struct GetAllNodesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllNodesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/cluster/nodes".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Node>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Approves the cluster upgrade.
#[derive(Clone)]
pub struct ApproveUpgradeRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ApproveUpgradeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/cluster/zdu/approve".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Cancels the ongoing cluster upgrade.
#[derive(Clone)]
pub struct CancelUpgradeRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CancelUpgradeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/cluster/zdu/cancel".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Retries the cluster upgrade.
#[derive(Clone)]
pub struct AcknowledgeErrorsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AcknowledgeErrorsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/cluster/zdu/retryUpgrade".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Starts the cluster upgrade.
#[derive(Clone)]
pub struct SetReadyToUpgradeRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> SetReadyToUpgradeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/cluster/zdu/start".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the current state of the cluster upgrade.
#[derive(Clone)]
pub struct GetStateRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetStateRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/cluster/zdu/state".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ClusterState> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
