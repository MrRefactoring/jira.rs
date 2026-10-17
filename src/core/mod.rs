//! Transport, authentication and error handling — everything the generated API surfaces are built on.
//!
//! A [`Client`] carries only transport, auth and retry policy, so one instance drives every surface this crate
//! exposes, with one set of credentials and one connection pool.

#![warn(missing_docs)]

#[macro_use]
mod gates;

#[cfg(feature = "audit")]
pub mod audit;
#[cfg(feature = "coverage")]
pub mod coverage;

mod auth;
#[cfg(feature = "server")]
mod avatar;
mod body;
mod client;
#[cfg(feature = "chrono")]
mod datetime;
mod error;
mod extensible;
mod mime;
mod multipart;
pub mod oauth;
mod open_enum;
core_gate! { page_streams { mod paging; } }
core_gate! { path_segments { mod path; } }
mod product;
mod query;
mod retry;
mod tenant_context;
core_gate! { optional_dates { #[cfg(not(feature = "chrono"))] mod timestamp; } }
core_gate! { untagged_variants { pub(crate) mod untagged; } }

pub use auth::{Auth, BoxFuture, OAuth2Config, OAuth2ServerConfig, TokenProvider, TokenRefreshHook};
pub use body::Body;
pub use client::{AuthRefresher, Client, ClientBuilder, RequestBuilder, RequestConfig};
#[cfg(feature = "chrono")]
pub use datetime::parse as parse_datetime;
core_gate! { optional_dates { #[cfg(feature = "chrono")] pub(crate) use datetime::deserialize_datetime; } }
core_gate! { jira_datetime_serialization { #[cfg(feature = "chrono")] pub(crate) use datetime::serialize_datetime; } }
core_gate! { rfc3339_datetime_serialization { #[cfg(feature = "chrono")] pub(crate) use datetime::serialize_datetime_rfc3339; } }
pub use error::{
    ApiErrorDetails, ApiErrorKind, Error, OAuthErrorDetails, Result, SchemaMismatchIssue, SchemaMismatchReport,
};
pub use extensible::Extensible;
pub use mime::mime_type_for;
pub use multipart::{Attachment, MultipartBody};
#[cfg(feature = "admin")]
pub(crate) use paging::cursor_from_url;
core_gate! { page_streams { pub(crate) use paging::{PageStep, stream_pages}; } }
core_gate! { path_segments { pub(crate) use path::encode_path_segment; } }
pub use product::{USER_AGENT, VERSION};
pub use query::QueryValue;
pub use retry::{RetryConfig, RetryOptions, with_retry};
pub use tenant_context::{TenantContext, get_tenant_context};
core_gate! { required_dates { #[cfg(not(feature = "chrono"))] pub(crate) use timestamp::deserialize_required_timestamp; } }
core_gate! { optional_dates { #[cfg(not(feature = "chrono"))] pub(crate) use timestamp::deserialize_timestamp; } }

pub use reqwest::Method;
