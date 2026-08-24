//! Assets and Service Management, as a self-hosted instance serves them.
//!
//! A separate rig from the Jira Data Center one, on its own port: `cargo xtask jsm-dc up`. The two do not fit side by
//! side on a machine with less memory than both want, so these run on their own.
//!
//! The container is thrown away after a run, so the fixtures here create the world rather than reconcile with what
//! was left in it, and nothing removes them afterwards. What a *test* makes, that test still removes — a developer
//! iterating on one file should not watch a schema fill with debris.
//!
//! `search_index` is named for where it has to run rather than for what it covers. A reindex holds the index for as
//! long as it takes and answers stale until it finishes, so it must come after everything that finds an object
//! through the index; the suites run in name order, and `search_index` sorts after `schemas` and before
//! `service_desk`, which reads nothing out of Assets at all.

mod configuration;
mod content;
mod fixtures;
mod objects;
mod schemas;
mod search_index;
mod service_desk;
