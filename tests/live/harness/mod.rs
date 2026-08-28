//! What every live suite is built on: credentials, shared clients, run-scoped names and cleanup.
//!
//! The suites run against a real Jira site and create real issues, so nothing here is a mock. Two rules keep that
//! honest. Every resource carries a run-scoped marker in its name, so concurrent runs cannot collide and a crashed
//! run leaves debris the sweep can recognise. And every resource is registered for deletion the moment it exists,
//! rather than at the end of the test that made it.

pub mod client;
pub mod entitlement;
pub mod env;
pub mod fixtures;
pub mod moment;
pub mod naming;
pub mod poll;
pub mod resources;
pub mod server_client;

pub use client::{admin_key_client, admin_surface, agile, client, cloud, org_id, service_desk, teams, user_management};
pub use entitlement::is_not_entitled;
pub use env::{has_admin_env, require_jsm_env, require_live_env, require_server_env};
pub use fixtures::{
    TEST_ISSUE_TYPE, TEST_PROJECT_KEY, TestBoard, await_agile_visibility, create_issue_with, create_test_board,
    create_test_issue, document_of, scrum_board,
};
pub use moment::{rendered, rendered_option};
pub use naming::{RESOURCE_MARKER, project_key, run_id, run_suffix, test_name};
pub use poll::{await_readable, await_refused, poll_until};
pub use resources::ResourceTracker;
pub use server_client::{assets_server, jsm_platform, server, server_client, service_desk_server};
