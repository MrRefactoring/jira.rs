use serde::Deserialize;
use serde_json::json;

use crate::core::client::Client;
use crate::core::error::{Error, Result, create_api_error};

/// The identifiers Atlassian's platform APIs address a site by.
///
/// A site has three names, and which one an API wants depends on the API. `cloud_id` addresses the site itself and is
/// what `https://api.atlassian.com/ex/jira/{cloudId}` is built from. `org_id` addresses the organization the site
/// belongs to, which is a level above it — several sites can share one — and is what the Teams API takes.
#[derive(Debug, Clone, Deserialize)]
pub struct TenantContext {
    /// Addresses the site. One per site.
    #[serde(rename = "cloudId")]
    pub cloud_id: String,
    /// Addresses the organization the site belongs to. Shared by every site in it.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// The site's host, e.g. `your-domain.atlassian.net`.
    #[serde(rename = "hostName")]
    pub host_name: String,
}

const TENANT_CONTEXT_QUERY: &str = "query TenantContext($hostNames: [String!]!) \
{ tenantContexts(hostNames: $hostNames) { cloudId orgId hostName } }";

#[derive(Debug, Deserialize)]
struct TenantContextFailure {
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    extensions: Option<TenantContextExtensions>,
}

#[derive(Debug, Deserialize)]
struct TenantContextExtensions {
    #[serde(rename = "statusCode", default)]
    status_code: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct TenantContextData {
    #[serde(rename = "tenantContexts", default)]
    tenant_contexts: Option<Vec<TenantContext>>,
}

#[derive(Debug, Deserialize)]
struct TenantContextResponse {
    #[serde(default)]
    data: Option<TenantContextData>,
    #[serde(default)]
    errors: Option<Vec<TenantContextFailure>>,
}

/// Resolves the site's cloud id, organization id and host name.
///
/// Atlassian publishes no REST endpoint for these — the GraphQL gateway is the documented way to ask, and an API
/// token is one of the auth categories it accepts. One request, through the client you already built, so it inherits
/// its proxy, retry policy and HTTP configuration.
///
/// Cloud only, and not under OAuth 2.0 (3LO): the gateway lives on the site's own host, which a 3LO client does not
/// have. A Data Center host does not serve it at all.
pub async fn get_tenant_context(client: &Client) -> Result<TenantContext> {
    let host = client.host().ok_or_else(|| {
        Error::config(
            "get_tenant_context needs the site it is asking about, and this client carries no host. Under OAuth 2.0 \
(3LO) the base URL is derived per request and there is nothing to ask about; pass a client configured with an \
explicit `host` instead.",
        )
    })?;

    let host_name = url::Url::parse(host)
        .ok()
        .and_then(|url| url.host_str().map(ToOwned::to_owned))
        .ok_or_else(|| Error::config(format!("`host` is not a valid URL: {host}")))?;

    let response: TenantContextResponse = client
        .post("/gateway/api/graphql")
        .json(&json!({
            "operationName": "TenantContext",
            "query": TENANT_CONTEXT_QUERY,
            "variables": { "hostNames": [host_name] },
        }))?
        .send()
        .await?;

    if let Some(context) =
        response.data.and_then(|data| data.tenant_contexts).and_then(|contexts| contexts.into_iter().next())
    {
        return Ok(context);
    }

    // The gateway answers 200 and reports the failure in the body, so the transport has already let this through as a
    // success. The real status rides in `extensions`.
    if let Some(failure) = response.errors.and_then(|errors| errors.into_iter().next()) {
        let message = failure.message.unwrap_or_else(|| "the GraphQL gateway reported an error".to_owned());
        let status = failure.extensions.and_then(|extensions| extensions.status_code).unwrap_or(502);

        return Err(create_api_error(
            format!("Could not resolve the tenant context: {message}"),
            status,
            "Bad Gateway".to_owned(),
            serde_json::Value::Null,
            None,
        ));
    }

    Err(create_api_error(
        format!("Could not resolve the tenant context: Atlassian does not know the site {host}."),
        404,
        "Not Found".to_owned(),
        serde_json::Value::Null,
        None,
    ))
}
