//! The Service Desk half of the surface, which needs a Service Management licence the Assets half does not.
//!
//! Assets ships with the image and its REST module does not check for a seat, so a Jira Software timebomb opens it
//! completely while every `/rest/servicedeskapi/` endpoint answers 403 with an HTML page. This suite therefore stands
//! down, visibly, when the instance is found unlicensed — the shape the Cloud suites established for a lapsed plan,
//! and for the same reason: a suite that fails over a licence buries the signal it exists to carry.
//!
//! It comes back the moment a Service Management Data Center timebomb is put in
//! `docker/jsm-dc/timebomb-license.txt`, without an edit here.

use super::fixtures::service_desk_licensed;
use crate::harness::service_desk_server;

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_what_the_application_is() {
    if !service_desk_licensed().await {
        eprintln!(
            "[live] Service Desk is not licensed on this instance, so its suite stands down. Assets runs either way.\n\
             [live] Put a Service Management Data Center timebomb in docker/jsm-dc/timebomb-license.txt to exercise \
             both."
        );

        return;
    }

    let info = service_desk_server().info().get_info().send().await.expect("a licensed instance describes itself");

    assert_eq!(info.is_licensed_for_use, Some(true), "the instance that answered says it is licensed");
    assert!(info.version.is_some(), "and names the version it is running");
}

/// The other side of the same fact: an unlicensed instance refuses this surface rather than answering it emptily.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn refuses_the_surface_outright_where_it_is_not_licensed() {
    if service_desk_licensed().await {
        return;
    }

    let error = service_desk_server()
        .info()
        .get_info()
        .send()
        .await
        .expect_err("an unlicensed instance does not answer this surface");

    assert!(error.status().is_some_and(|status| status >= 400), "the refusal is typed rather than a parse failure");
}
