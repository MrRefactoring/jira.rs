use crate::harness::service_desk;

/// The Service Management `info` and `servicedesk` APIs, whose availability is not a single yes or no.
///
/// The product can be installed on a site — `get_info` answers with a version — while every service-desk endpoint
/// still refuses with 403, because the account holds no agent licence. Those are two different gates, and a suite
/// that probed only the first would report a licensing fact as a library defect.
///
/// So reachability is established against the listing rather than against `get_info`, and the typed refusal is
/// asserted as a first-class outcome rather than skipped past.
///
/// `get_info` needs no agent licence, and no login at all.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_product_version_which_needs_no_agent_licence() {
    let info = service_desk().info().get_info().send().await.expect("the instance reports what it is running");

    assert!(info.version.as_ref().is_some_and(|version| !version.is_empty()), "the instance names its own version");
    assert!(
        info.platform_version.as_ref().is_some_and(|version| !version.is_empty()),
        "and the Jira platform version it is built on",
    );
}

/// The listing is where the licence gate shows itself: without an agent licence it is a 403, and with one it is a
/// page of desks each tied to a project. Both outcomes are proven here rather than one of them skipped.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_service_desks_each_tied_to_a_project_or_refuses_typed() {
    let desks = match service_desk().servicedesk().get_service_desks().limit(10).send().await {
        Ok(desks) => desks,
        Err(error) => {
            assert!(error.is_forbidden(), "an account without an agent licence is refused by rights: {error}");
            assert_eq!(error.status(), Some(403), "and the refusal keeps its status: {error}");

            return;
        }
    };

    assert!(desks.is_last_page.is_some(), "a page says whether it is the last one");

    for desk in &desks.values {
        assert!(desk.id.as_ref().is_some_and(|id| !id.is_empty()), "a service desk carries its own id");
        assert!(desk.project_id.as_ref().is_some_and(|id| !id.is_empty()), "and the id of its peer project");
        assert!(desk.project_key.is_some(), "and that project's key");
    }
}

/// This surface pages with `start` and `limit`, not with the platform's `startAt` and `maxResults`.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_with_start_and_limit_rather_than_start_at_and_max_results() {
    let page = match service_desk().servicedesk().get_service_desks().start(0).limit(1).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden(), "an account without an agent licence is refused by rights: {error}");

            return;
        }
    };

    assert_eq!(page.limit, Some(1), "the page size asked for is the page size returned");
    assert_eq!(page.start, Some(0), "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 1, "a page holds no more than it says it does: {}", page.values.len());
    assert!(page.is_last_page.is_some(), "a page says whether it is the last one");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_service_desk_and_its_request_types() {
    let desks = match service_desk().servicedesk().get_service_desks().limit(1).send().await {
        Ok(desks) => desks,
        Err(error) => {
            assert!(error.is_forbidden(), "an account without an agent licence is refused by rights: {error}");

            return;
        }
    };

    let Some(first) = desks.values.first() else {
        assert_eq!(desks.is_last_page, Some(true), "a site with no service desk answers with one empty, final page");

        return;
    };

    let id = first.id.clone().expect("a listed service desk carries an id");

    let desk = service_desk()
        .servicedesk()
        .get_service_desk_by_id(&id)
        .send()
        .await
        .expect("a listed service desk reads back by its own id");

    assert_eq!(desk.id.as_deref(), Some(id.as_str()), "the desk read back is the desk listed");
    assert_eq!(desk.project_key, first.project_key, "and it names the same peer project");

    let request_types = service_desk()
        .servicedesk()
        .get_request_types(&id)
        .send()
        .await
        .expect("a service desk answers with the request types it offers");

    for request_type in &request_types.values {
        assert!(request_type.id.as_ref().is_some_and(|id| !id.is_empty()), "a request type carries its own id");
        assert_eq!(
            request_type.service_desk_id.as_deref(),
            Some(id.as_str()),
            "and belongs to the desk it was asked of",
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_service_desk_as_a_typed_error() {
    let error = service_desk()
        .servicedesk()
        .get_service_desk_by_id("99999999")
        .send()
        .await
        .expect_err("a service desk that does not exist cannot be read");

    assert!(
        error.is_not_found() || error.status() == Some(400) || error.is_forbidden(),
        "an unknown desk is missing, rejected or refused by licence — never untyped: {error}",
    );
}
