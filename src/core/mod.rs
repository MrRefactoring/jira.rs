//! Transport, authentication and error handling — everything the generated API surfaces are built on.
//!
//! A [`Client`] carries only transport, auth and retry policy, so one instance drives every surface this crate
//! exposes, with one set of credentials and one connection pool.

#![warn(missing_docs)]

#[cfg(feature = "audit")]
pub mod audit;
#[cfg(feature = "coverage")]
pub mod coverage;

mod auth;
mod body;
mod client;
#[cfg(feature = "chrono")]
mod datetime;
mod error;
mod mime;
mod multipart;
pub mod oauth;
mod open_enum;
mod paging;
mod path;
mod product;
mod query;
mod retry;
mod tenant_context;
#[cfg(not(feature = "chrono"))]
mod timestamp;

pub use auth::{Auth, BoxFuture, OAuth2Config, OAuth2ServerConfig, TokenProvider, TokenRefreshHook};
pub use body::Body;
pub use client::{AuthRefresher, Client, ClientBuilder, RequestBuilder, RequestConfig};
#[cfg(feature = "chrono")]
pub use datetime::parse as parse_datetime;
#[cfg(feature = "chrono")]
pub(crate) use datetime::{deserialize_datetime, serialize_datetime};
pub use error::{
    ApiErrorDetails, ApiErrorKind, Error, OAuthErrorDetails, Result, SchemaMismatchIssue, SchemaMismatchReport,
};
pub use mime::mime_type_for;
pub use multipart::{Attachment, MultipartBody};
pub(crate) use paging::{PageStep, Paged, stream_pages};
pub(crate) use path::encode_path_segment;
pub use product::{USER_AGENT, VERSION};
pub use query::QueryValue;
#[allow(unused_imports)]
pub(crate) use query::header_value;
pub use retry::{RetryConfig, RetryOptions, with_retry};
pub use tenant_context::{TenantContext, get_tenant_context};
#[cfg(not(feature = "chrono"))]
pub(crate) use timestamp::{deserialize_required_timestamp, deserialize_timestamp};

pub use reqwest::Method;
