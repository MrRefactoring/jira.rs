use crate::harness::cloud;

/// The issue priorities API, read-only.
///
/// Priorities are site-wide: deleting one asks Jira to migrate every issue that used it, and changing the default
/// changes what every new issue gets. Neither belongs in a suite running against a working site, so the admin-only
/// half is pinned only through its error channel.
///
/// The detail worth a live check is that `search_priorities` types its pagination as *strings* — `start_at` and
/// `max_results` take `String` here and numbers everywhere else in the API. That is inherited from the specification,
/// and it is the kind of thing that compiles fine and then serializes to something unexpected.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_site_priorities_each_fully_typed() {
    let page = cloud().issue_priorities().search_priorities().send().await.expect("the site lists its priorities");

    assert!(!page.values.is_empty(), "a site always carries the priorities its default scheme needs");

    for priority in &page.values {
        let id = priority.id.as_deref().expect("a priority carries an id");

        assert!(id.chars().all(|c| c.is_ascii_digit()) && !id.is_empty(), "a priority id is digits: {id}");
        assert!(priority.name.as_ref().is_some_and(|name| !name.is_empty()), "a priority carries a name");
        assert!(priority.status_color.as_ref().is_some_and(|color| !color.is_empty()), "a priority carries a colour");
        assert!(
            priority.self_.as_ref().is_some_and(|link| link.starts_with("https://")),
            "a priority links back to itself over https",
        );
    }
}

/// The search endpoint reports every priority as non-default, including the one that actually is the default.
/// `only_default` is the parameter that answers that question; this pins that the flag on the row does not.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn does_not_mark_any_priority_as_the_default_through_this_endpoint() {
    let page = cloud().issue_priorities().search_priorities().send().await.expect("the site lists its priorities");

    assert!(
        page.values.iter().all(|priority| priority.is_default == Some(false)),
        "the listing reports every priority as non-default",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn honours_pagination_typed_as_strings_rather_than_numbers() {
    let page = cloud()
        .issue_priorities()
        .search_priorities()
        .max_results("1")
        .send()
        .await
        .expect("a page size given as a string is accepted");

    assert!(page.values.len() <= 1, "a page holds no more than it was asked for");
    assert_eq!(page.max_results, 1, "the string that went out comes back as the number the page declares");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_by_name_and_by_id() {
    let all = cloud().issue_priorities().search_priorities().send().await.expect("the site lists its priorities");
    let sample = all.values.first().expect("a site carries at least one priority").clone();
    let id = sample.id.clone().expect("a priority carries an id");
    let name = sample.name.clone().expect("a priority carries a name");

    let by_id =
        cloud().issue_priorities().search_priorities().id([&id]).send().await.expect("the id filter is accepted");

    assert_eq!(
        by_id.values.iter().map(|priority| priority.id.clone()).collect::<Vec<_>>(),
        vec![sample.id.clone()],
        "the id filter returns exactly what it was asked for",
    );

    let by_name = cloud()
        .issue_priorities()
        .search_priorities()
        .priority_name(&name)
        .send()
        .await
        .expect("the name filter is accepted");

    assert!(
        by_name.values.iter().any(|priority| priority.id == sample.id),
        "the name filter finds the priority that carries that name",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_priority_identical_to_its_listing_entry() {
    let all = cloud().issue_priorities().search_priorities().send().await.expect("the site lists its priorities");
    let sample = all.values.first().expect("a site carries at least one priority").clone();
    let id = sample.id.clone().expect("a priority carries an id");

    let priority = cloud().issue_priorities().get_priority(&id).send().await.expect("a priority reads back by id");

    assert_eq!(priority.id, sample.id);
    assert_eq!(priority.name, sample.name, "the single read agrees with the listing entry");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_priority_as_not_found() {
    let error = cloud()
        .issue_priorities()
        .get_priority("99999999")
        .send()
        .await
        .expect_err("a priority that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

/// The destructive path, proven through its error channel and never aimed at a priority that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .issue_priorities()
        .delete_priority("99999999")
        .send()
        .await
        .expect_err("a priority that does not exist cannot be deleted");

    assert!(
        error.is_not_found() || error.is_forbidden() || error.status() == Some(400),
        "the delete is refused typed: {error}",
    );
}
