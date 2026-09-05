// @generated. Do not edit: change the generator or the specification.

/// The SearchLimits operations.
pub struct SearchLimitsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> SearchLimitsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the maximum number of aggregation buckets allowed by the underlying search platform
    ///
    /// Available since Jira Data Center 11.3.
    pub fn get_max_aggregation_buckets(&self) -> GetMaxAggregationBucketsRequest<'a> {
        GetMaxAggregationBucketsRequest::new(self.client)
    }

    /// Returns the maximum number of search results that can be returned by the underlying search platform
    ///
    /// Available since Jira Data Center 11.3.
    pub fn get_max_result_window(&self) -> GetMaxResultWindowRequest<'a> {
        GetMaxResultWindowRequest::new(self.client)
    }
}

/// Returns the maximum number of aggregation buckets allowed by the underlying search platform
///
/// Available since Jira Data Center 11.3.
#[derive(Clone)]
pub struct GetMaxAggregationBucketsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetMaxAggregationBucketsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/searchLimits/maxAggregationBuckets".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the maximum number of search results that can be returned by the underlying search platform
///
/// Available since Jira Data Center 11.3.
#[derive(Clone)]
pub struct GetMaxResultWindowRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetMaxResultWindowRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/searchLimits/maxResultWindow".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
