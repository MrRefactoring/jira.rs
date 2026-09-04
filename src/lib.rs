#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

//! A Rust client for the Atlassian Jira REST APIs.
//!
//! ```no_run
//! use jira::{Auth, Client};
//!
//! # async fn example() -> jira::Result<()> {
//! let client = Client::builder()
//!     .host("https://your-domain.atlassian.net")
//!     .auth(Auth::api_token("you@example.com", "YOUR_API_TOKEN"))
//!     .build()?;
//!
//! let myself: serde_json::Value = client.get("/rest/api/3/myself").send().await?;
//!
//! println!("{}", myself["displayName"]);
//! # Ok(())
//! # }
//! ```

pub mod core;

pub mod jql;

#[cfg(feature = "cloud")]
mod paging;

#[cfg(feature = "admin")]
#[cfg_attr(docsrs, doc(cfg(feature = "admin")))]
pub mod admin;

#[cfg(feature = "agile")]
#[cfg_attr(docsrs, doc(cfg(feature = "agile")))]
pub mod agile;

#[cfg(feature = "assets")]
#[cfg_attr(docsrs, doc(cfg(feature = "assets")))]
pub mod assets;

#[cfg(feature = "assets-server")]
#[cfg_attr(docsrs, doc(cfg(feature = "assets-server")))]
pub mod assets_server;

#[cfg(feature = "cloud")]
#[cfg_attr(docsrs, doc(cfg(feature = "cloud")))]
pub mod cloud;

#[cfg(feature = "server")]
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub mod server;

#[cfg(feature = "service-desk")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-desk")))]
pub mod service_desk;

#[cfg(feature = "service-desk-server")]
#[cfg_attr(docsrs, doc(cfg(feature = "service-desk-server")))]
pub mod service_desk_server;

#[cfg(feature = "teams")]
#[cfg_attr(docsrs, doc(cfg(feature = "teams")))]
pub mod teams;

#[cfg(feature = "user-management")]
#[cfg_attr(docsrs, doc(cfg(feature = "user-management")))]
pub mod user_management;

#[cfg(feature = "user-provisioning")]
#[cfg_attr(docsrs, doc(cfg(feature = "user-provisioning")))]
pub mod user_provisioning;

#[cfg(feature = "webhooks")]
#[cfg_attr(docsrs, doc(cfg(feature = "webhooks")))]
pub mod webhooks;

pub use crate::core::{
    ApiErrorDetails, ApiErrorKind, Attachment, Auth, Body, Client, ClientBuilder, Error, MultipartBody,
    OAuthErrorDetails, QueryValue, RequestConfig, Result, RetryConfig, RetryOptions, SchemaMismatchIssue,
    SchemaMismatchReport, TenantContext, with_retry,
};

/// The HTTP client underneath, so a caller can name the same version this crate was built against.
///
/// It is part of the public API whether it is re-exported or not — [`ClientBuilder::http_client`] takes a
/// `reqwest::Client`, and [`Error::Network`] carries a `reqwest::Error` — and without this a caller who depends on
/// `reqwest` themselves has to match the version by hand to pass one in.
pub use reqwest;

/// The stream machinery every request builder's `stream()` answers with.
///
/// Its `TryStreamExt` is what reads a stream, and the trait has to be in scope to be used, so re-exporting it saves
/// a caller a dependency they would otherwise have to add and keep in step with this one.
pub use futures_util;
