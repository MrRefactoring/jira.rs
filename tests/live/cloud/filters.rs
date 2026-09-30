use jira::cloud::{ColumnRequestBody, Filter};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, await_readable, await_refused, cloud, poll_until, test_name};

async fn create_filter(tracker: &mut ResourceTracker, name: &str, jql: &str) -> Filter {
    let filter = cloud()
        .filters()
        .create_filter(Filter {
            name: name.to_owned(),
            description: Some("created by the live suite".to_owned()),
            jql: Some(jql.to_owned()),
            ..Filter::default()
        })
        .send()
        .await
        .expect("the account may create a filter of its own");

    let id = id_of(&filter);

    tracker.defer(move || async move { cloud().filters().delete_filter(id).send().await });

    poll_until("the filter just created to read back", || async { cloud().filters().get_filter(id).send().await.ok() })
        .await;

    filter
}

fn id_of(filter: &Filter) -> i64 {
    filter.id.as_deref().expect("a created filter has an id").parse().expect("a filter id is a number")
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn creates_reads_and_updates_a_filter() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("filter lifecycle");
    let jql = format!("project = {TEST_PROJECT_KEY} ORDER BY created DESC");
    let created = create_filter(&mut tracker, &name, &jql).await;
    let id = id_of(&created);

    assert!(created.id.as_deref().is_some_and(|id| id.chars().all(|c| c.is_ascii_digit())), "an id is digits");
    assert_eq!(created.name, name);
    assert!(
        created.owner.as_ref().and_then(|owner| owner.account_id.as_deref()).is_some_and(|id| !id.is_empty()),
        "a filter carries the account that owns it",
    );
    assert!(created.share_permissions.as_deref().unwrap_or_default().is_empty(), "a new filter is private");

    let read = cloud().filters().get_filter(id).send().await.expect("the filter reads back by id");

    assert_eq!(read.jql.as_deref(), Some(jql.as_str()), "the JQL is stored verbatim");
    assert!(read.search_url.as_deref().is_some_and(|url| url.starts_with("https://")), "{:?}", read.search_url);
    assert!(read.view_url.as_deref().is_some_and(|url| url.starts_with("https://")), "{:?}", read.view_url);
    assert_ne!(read.view_url, read.search_url, "searching a filter and viewing it are two different URLs");

    let mine = cloud().filters().get_my_filters().send().await.expect("the account lists the filters it owns");

    assert!(
        mine.iter().any(|filter| filter.id.as_deref() == created.id.as_deref()),
        "the filter is one of the account's own",
    );

    let page = cloud()
        .filters()
        .get_filters_paginated()
        .filter_name(name.as_str())
        .max_results(10)
        .send()
        .await
        .expect("the paginated search accepts a name");

    assert!(
        page.values.iter().any(|filter| filter.id.as_deref() == created.id.as_deref()),
        "the search by name finds the filter",
    );
    assert!(page.total >= 1, "a page that holds the filter counts at least one");

    let replaced = format!("project = {TEST_PROJECT_KEY} AND status != Done");

    let updated = cloud()
        .filters()
        .update_filter(id, Filter { name: name.clone(), jql: Some(replaced.clone()), ..Filter::default() })
        .send()
        .await
        .expect("the filter's JQL can be replaced");

    assert_eq!(updated.jql.as_deref(), Some(replaced.as_str()), "the update replaces the JQL rather than merging it");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_jql_the_site_cannot_make_sense_of() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("filter jql");
    let filter = create_filter(&mut tracker, &name, &format!("project = {TEST_PROJECT_KEY}")).await;
    let id = id_of(&filter);

    let unknown_field = cloud()
        .filters()
        .update_filter(id, Filter { name: name.clone(), jql: Some("nosuchfield = 1".to_owned()), ..Filter::default() })
        .send()
        .await
        .expect_err("JQL naming a field that does not exist is refused");

    assert_eq!(unknown_field.status(), Some(400), "{unknown_field}");

    let unparseable = cloud()
        .filters()
        .update_filter(
            id,
            Filter { name: name.clone(), jql: Some("project = \"unterminated".to_owned()), ..Filter::default() },
        )
        .send()
        .await
        .expect_err("JQL that does not parse is refused");

    assert_eq!(unparseable.status(), Some(400), "{unparseable}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn marks_the_filter_as_a_favourite() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("filter favourite");
    let filter = create_filter(&mut tracker, &name, &format!("project = {TEST_PROJECT_KEY}")).await;
    let id = id_of(&filter);

    let favourited =
        cloud().filters().set_favourite_for_filter(id).send().await.expect("a filter can be made a favourite");

    assert_eq!(favourited.favourite, Some(true));

    poll_until("the filter to appear among the account's favourites", || async {
        let favourites =
            cloud().filters().get_favourite_filters().send().await.expect("the account lists its favourites");

        favourites.iter().any(|favourite| favourite.id.as_deref() == filter.id.as_deref()).then_some(())
    })
    .await;

    let unfavourited =
        cloud().filters().delete_favourite_for_filter(id).send().await.expect("a favourite can be given up");

    assert_eq!(unfavourited.favourite, Some(false));

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_filter_whose_name_collides_with_an_existing_one() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("filter collision");
    let jql = format!("project = {TEST_PROJECT_KEY}");

    create_filter(&mut tracker, &name, &jql).await;

    let error = cloud()
        .filters()
        .create_filter(Filter { name: name.clone(), jql: Some(jql), ..Filter::default() })
        .send()
        .await
        .expect_err("an account cannot own two filters of the same name");

    assert_eq!(error.status(), Some(400), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn makes_the_filter_unreadable_once_deleted() {
    let mut tracker = ResourceTracker::new();
    let filter =
        create_filter(&mut tracker, &test_name("filter throwaway"), &format!("project = {TEST_PROJECT_KEY}")).await;
    let id = id_of(&filter);

    cloud().filters().delete_filter(id).send().await.expect("the owner may delete the filter");

    let error = await_refused("a deleted filter cannot be read", || cloud().filters().get_filter(id).send()).await;

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_filter_as_not_found() {
    let error = cloud().filters().get_filter(99_999_999).send().await.expect_err("a filter that does not exist");

    assert!(error.is_not_found(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn gives_a_filter_its_own_columns_then_takes_them_away_again() {
    let mut tracker = ResourceTracker::new();
    let filter =
        create_filter(&mut tracker, &test_name("filter columns"), &format!("project = {TEST_PROJECT_KEY}")).await;
    let id = id_of(&filter);

    let missing = cloud().filters().get_columns(id).send().await.expect_err("a filter starts with no layout");

    assert!(missing.is_not_found(), "{missing}");

    cloud()
        .filters()
        .set_columns(id, ColumnRequestBody { columns: Some(vec!["summary".to_owned(), "status".to_owned()]) })
        .send()
        .await
        .expect("a filter can be given columns of its own");

    let columns = await_readable("the columns read back", || cloud().filters().get_columns(id).send()).await;

    assert_eq!(
        columns.iter().map(|column| column.value.clone()).collect::<Vec<_>>(),
        vec![Some("summary".to_owned()), Some("status".to_owned())],
        "the columns come back in the order they were set",
    );

    cloud().filters().reset_columns(id).send().await.expect("the columns can be taken away again");

    let after_reset = await_refused("the layout is gone again", || cloud().filters().get_columns(id).send()).await;

    assert!(after_reset.is_not_found(), "{after_reset}");

    tracker.cleanup().await;
}
