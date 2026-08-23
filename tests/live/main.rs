//! The live suites: every operation, against a real Jira site.
//!
//! They are `#[ignore]`d so `cargo test` stays fast and green without credentials. To run them:
//!
//! ```sh
//! cargo test --test live --all-features -- --ignored --test-threads=1
//! ```
//!
//! Single-threaded on purpose. The suites share one tenant, and Jira rate-limits a client that talks to it from
//! twenty threads at once — a red run that says nothing about the library.

// The harness is a surface the suites grow into: a helper no suite has reached yet is not dead code, it is a
// helper the next surface will use.
#[allow(dead_code, unused_imports)]
mod harness;

mod cloud;
