//! Transport, authentication and error handling — everything the generated API surfaces are built on.
//!
//! A [`Client`] carries only transport, auth and retry policy, so one instance drives every surface this crate
//! exposes, with one set of credentials and one connection pool.

#[cfg(feature = "audit")]
pub mod audit;
#[cfg(feature = "coverage")]
pub mod coverage;

mod auth;
mod body;
mod client;
mod error;
mod mime;
mod multipart;
pub mod oauth;
mod open_enum;
mod product;
mod query;
mod retry;
mod tenant_context;
mod timestamp;

pub use auth::{Auth, BoxFuture, OAuth2Config, OAuth2ServerConfig, TokenProvider, TokenRefreshHook};
pub use body::Body;
pub use client::{AuthRefresher, Client, ClientBuilder, RequestBuilder, RequestConfig};
pub use error::{
    ApiErrorDetails, ApiErrorKind, Error, OAuthErrorDetails, Result, SchemaMismatchIssue, SchemaMismatchReport,
    TRANSIENT_HTTP_STATUSES, create_api_error, is_transient_status, parse_retry_after,
};
pub use mime::{DEFAULT_MIME_TYPE, mime_type_for};
pub use multipart::{Attachment, MultipartBody};
pub use product::{GATEWAY_SLUG, PACKAGE_NAME, USER_AGENT, VERSION};
pub use query::{QueryValue, build_url_with_search_params, header_value};
pub use retry::{RetryConfig, RetryOptions, with_retry};
pub use tenant_context::{TenantContext, get_tenant_context};
pub use timestamp::{deserialize_required_timestamp, deserialize_timestamp};

pub use reqwest::Method;
