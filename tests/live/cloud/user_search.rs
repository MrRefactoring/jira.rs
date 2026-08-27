use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, poll_until, test_name};

/// The account the token authenticates as, which every search here is expected to find.
async fn current_account_id() -> String {
    cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site names the authenticating account")
        .account_id
        .expect("the authenticating account carries an id")
}

/// Eight endpoints that all look like "search for a user" and are not interchangeable.
///
/// Each answers a different question — who exists, who can be assigned this issue, who can see this project — and
/// nothing in the types distinguishes them. Picking the wrong one produces a picker that offers people who will be
/// rejected on save.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_a_user_by_exact_account_id() {
    let account_id = current_account_id().await;

    let found = cloud()
        .user_search()
        .find_users()
        .account_id(account_id.as_str())
        .send()
        .await
        .expect("an exact account id is a valid search");

    assert_eq!(found.len(), 1, "an exact account id matches exactly one user");
    assert_eq!(found[0].account_id.as_deref(), Some(account_id.as_str()));
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_an_unmatched_query_with_an_empty_list_not_an_error() {
    let found = cloud()
        .user_search()
        .find_users()
        .query("nobodymatchesthisquerystring")
        .send()
        .await
        .expect("a query that matches nobody is still a valid search");

    assert!(found.is_empty(), "an unmatched query returns nothing, not an error: {found:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_a_total_alongside_the_page_through_the_picker() {
    let picker = cloud()
        .user_search()
        .find_users_for_picker("")
        .max_results(5)
        .send()
        .await
        .expect("the picker answers an empty query");

    let total = picker.total.expect("the picker reports a total alongside the page");
    let users = picker.users.unwrap_or_default();

    assert!(users.len() <= 5, "maxResults caps the page, got {} users", users.len());
    assert!(
        usize::try_from(total).expect("a total is not negative") >= users.len(),
        "the total counts at least the page it came with",
    );

    for user in &users {
        assert!(user.account_id.as_deref().is_some_and(|id| !id.is_empty()), "a picked user carries an id");
        assert!(user.display_name.is_some(), "the picker names the users it offers: {user:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_who_can_be_assigned_a_specific_issue() {
    let mut tracker = ResourceTracker::new();
    let account_id = current_account_id().await;
    let issue = create_test_issue(&mut tracker, Some(&test_name("assignable"))).await;

    let assignable = poll_until("the assignable-user search to see the issue", || async {
        cloud().user_search().find_assignable_users().issue_key(&issue.key).send().await.ok()
    })
    .await;

    let ids: Vec<_> = assignable.iter().filter_map(|user| user.account_id.clone()).collect();

    assert!(ids.contains(&account_id), "the account that created the issue can be assigned it, got {ids:?}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn scopes_assignability_to_a_project_as_well_as_to_an_issue() {
    let account_id = current_account_id().await;

    let by_project = cloud()
        .user_search()
        .find_assignable_users()
        .project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("a project names who can be assigned its issues");

    let ids: Vec<_> = by_project.iter().filter_map(|user| user.account_id.clone()).collect();

    assert!(ids.contains(&account_id), "the account the suite runs as is assignable in the test project, got {ids:?}");
}

/// Assignability is meaningless without something to be assigned to, and the API says so rather than answering with
/// an empty list a caller would read as "nobody can be assigned".
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn requires_enough_context_to_answer_at_all() {
    let error = cloud()
        .user_search()
        .find_assignable_users()
        .query("")
        .send()
        .await
        .expect_err("assignability needs an issue or a project");

    assert_eq!(error.status(), Some(400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_who_can_browse_the_project_but_only_when_given_a_query_too() {
    let account_id = current_account_id().await;

    let error = cloud()
        .user_search()
        .find_users_with_browse_permission()
        .project_key(TEST_PROJECT_KEY)
        .send()
        .await
        .expect_err("a project alone does not say who to look for");

    assert_eq!(error.status(), Some(400), "{error}");

    let browsers = cloud()
        .user_search()
        .find_users_with_browse_permission()
        .project_key(TEST_PROJECT_KEY)
        .account_id(account_id.as_str())
        .send()
        .await
        .expect("a project and an account id together are enough");

    let ids: Vec<_> = browsers.iter().filter_map(|user| user.account_id.clone()).collect();

    assert!(ids.contains(&account_id), "the account the suite runs as can browse the test project, got {ids:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_permission_scoped_searches_with_the_permissions_named() {
    let account_id = current_account_id().await;

    let found = cloud()
        .user_search()
        .find_users_with_all_permissions("BROWSE_PROJECTS")
        .project_key(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("a named permission scopes the search");

    let ids: Vec<_> = found.iter().filter_map(|user| user.account_id.clone()).collect();

    assert!(ids.contains(&account_id), "the account the suite runs as holds BROWSE_PROJECTS, got {ids:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn honours_max_results_across_the_search_endpoints() {
    let limited = cloud()
        .user_search()
        .find_users()
        .query("")
        .max_results(1)
        .send()
        .await
        .expect("an empty query with a limit is a valid search");

    assert!(limited.len() <= 1, "maxResults caps the page, got {} users", limited.len());
}
