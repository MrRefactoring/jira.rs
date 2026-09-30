use jira::cloud::GetAllDashboardsRequestFilter;

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_dashboards_with_the_older_offset_pagination() {
    let page = cloud().dashboards().get_all_dashboards().max_results(5).send().await.expect("the site lists its own");

    assert_eq!(page.start_at, Some(0), "an unoffset request starts at the beginning");
    assert_eq!(page.max_results, Some(5), "the page size asked for is the page size returned");
    assert!(page.total.is_some(), "the older listing counts what it paged over");
    assert!(page.dashboards.is_some(), "the listing carries a list of dashboards, empty or not");

    for dashboard in page.dashboards.iter().flatten() {
        let id = dashboard.id.as_deref().expect("a dashboard carries an id");

        assert!(id.chars().all(|character| character.is_ascii_digit()), "an id is digits: {id}");
        assert!(dashboard.name.as_ref().is_some_and(|name| !name.is_empty()), "a dashboard carries a name");
        assert!(
            dashboard.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
            "a dashboard carries the URL it reads back from: {:?}",
            dashboard.self_,
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_same_dashboards_under_a_different_shape_when_paginated() {
    let page =
        cloud().dashboards().get_dashboards_paginated().max_results(5).send().await.expect("the newer listing answers");

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");
    assert!(page.is_last || page.values.len() == 5, "a page that is not the last one is full");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_to_the_dashboards_the_account_owns() {
    let mine = cloud()
        .dashboards()
        .get_all_dashboards()
        .filter(GetAllDashboardsRequestFilter::My)
        .max_results(50)
        .send()
        .await
        .expect("the owned filter is accepted");

    let all = cloud().dashboards().get_all_dashboards().max_results(50).send().await.expect("the site lists its own");

    let owned = mine.dashboards.map_or(0, |dashboards| dashboards.len());
    let visible = all.dashboards.map_or(0, |dashboards| dashboards.len());

    assert!(owned <= visible, "the account cannot own more dashboards than it can see, {owned} of {visible}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_to_favourites_which_is_per_user_state() {
    let favourites = cloud()
        .dashboards()
        .get_all_dashboards()
        .filter(GetAllDashboardsRequestFilter::Favourite)
        .max_results(50)
        .send()
        .await
        .expect("the favourite filter is accepted");

    assert!(favourites.dashboards.is_some(), "the listing carries a list of dashboards, empty or not");

    for dashboard in favourites.dashboards.iter().flatten() {
        assert_eq!(dashboard.is_favourite, Some(true), "the favourite filter returns only what it was asked for");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn searches_by_name_through_the_paginated_listing() {
    let all =
        cloud().dashboards().get_dashboards_paginated().max_results(1).send().await.expect("the newer listing answers");

    let first = all.values.first().expect("every site carries at least the dashboard it ships with");
    let name = first.name.clone().expect("a dashboard carries a name");
    let id = first.id.clone().expect("a dashboard carries an id");

    let found = cloud()
        .dashboards()
        .get_dashboards_paginated()
        .dashboard_name(name)
        .max_results(10)
        .send()
        .await
        .expect("the name search is accepted");

    assert!(
        found.values.iter().any(|dashboard| dashboard.id.as_deref() == Some(id.as_str())),
        "a dashboard searched for by its own name is among the matches",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_dashboard_by_id() {
    let all = cloud().dashboards().get_all_dashboards().max_results(1).send().await.expect("the site lists its own");

    let first =
        all.dashboards.into_iter().flatten().next().expect("every site carries at least the dashboard it ships with");

    let id = first.id.clone().expect("a dashboard carries an id");
    let dashboard = cloud().dashboards().get_dashboard(id).send().await.expect("a listed dashboard reads back by id");

    assert_eq!(dashboard.id, first.id, "the dashboard read back is the one that was asked for");
    assert_eq!(dashboard.name, first.name, "the listing and the single read agree on the name");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_dashboard_as_not_found() {
    let error = cloud()
        .dashboards()
        .get_dashboard("99999999")
        .send()
        .await
        .expect_err("a dashboard that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}
