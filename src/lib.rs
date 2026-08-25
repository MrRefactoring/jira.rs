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
    Attachment, Auth, Body, Client, ClientBuilder, Error, MultipartBody, QueryValue, RequestConfig, Result,
    RetryConfig, RetryOptions, SchemaMismatchIssue, SchemaMismatchReport, TenantContext, with_retry,
};
