//! The live suites: every operation, against a real Jira site.
//!
//! They are `#[ignore]`d so `cargo test` stays fast and green without credentials. To run them:
//!
//! ```sh
//! cargo live          # every surface a hosted site serves
//! cargo live-server   # the Data Center suites, after `cargo xtask jira-dc up`
//! cargo live-jsm      # the Service Management suites, after `cargo xtask jsm-dc up`
//! ```
//!
//! Three commands rather than one because two of the surfaces need a container brought up first, and the aliases in
//! `.cargo/config.toml` are where the boundary between them is written down. Together they cover the binary exactly.
//!
//! Single-threaded on purpose. The suites share one tenant, and Jira rate-limits a client that talks to it from
//! twenty threads at once — a red run that says nothing about the library.

// The harness is a surface the suites grow into: a helper no suite has reached yet is not dead code, it is a
// helper the next surface will use.
#[allow(dead_code, unused_imports)]
mod harness;

mod admin;
mod agile;
mod assets;
mod cloud;
mod jsm;
mod server;
mod service_desk;
mod teams;
mod user_management;
