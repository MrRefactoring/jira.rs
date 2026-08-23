use jira::cloud::FoundGroup;

use crate::harness::cloud;

/// A group that exists on this site, carrying both identifiers.
///
/// Every endpoint here is addressable by `groupname` or by `groupId`, and Atlassian is migrating away from the
/// former — code written against names keeps working until a group is renamed, then fails in a way that looks like
/// the group vanished. Both modes are exercised against the same group so the two answers can be compared.
async fn sample_group() -> FoundGroup {
    let found = cloud().groups().find_groups().max_results(10).send().await.expect("the site answers with its groups");

    found
        .groups
        .unwrap_or_default()
        .into_iter()
        .find(|group| group.group_id.is_some() && group.name.is_some())
        .expect("a Jira site has at least one group carrying both identifiers")
}

/// Read-only by design. Permission schemes grant rights *to groups*, so `removeGroup` takes a `swapGroup` parameter
/// precisely because deleting a group can strip permissions from everyone who was in it. A suite has no business
/// generating that risk on a working tenant, so membership is only ever read.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_groups_on_the_site_each_carrying_both_identifiers() {
    let found = cloud().groups().find_groups().max_results(10).send().await.expect("the site answers with its groups");
    let groups = found.groups.expect("a group search carries a groups list");

    assert!(!groups.is_empty(), "a Jira site is never without groups");

    for group in &groups {
        assert!(group.name.as_deref().is_some_and(|name| !name.is_empty()), "a group carries a name: {group:?}");
        assert!(group.group_id.as_deref().is_some_and(|id| !id.is_empty()), "a group carries an id: {group:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn narrows_the_search_by_query() {
    let sample = sample_group().await;
    let name = sample.name.clone().expect("the sample group is named");
    let prefix: String = name.chars().take(4).collect();

    let found =
        cloud().groups().find_groups().query(prefix).max_results(20).send().await.expect("a query narrows the search");

    let ids: Vec<_> = found.groups.unwrap_or_default().into_iter().filter_map(|group| group.group_id).collect();

    assert!(ids.contains(&sample.group_id.clone().unwrap_or_default()), "a prefix of its own name finds {name}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn matches_case_insensitively_when_asked() {
    let sample = sample_group().await;
    let name = sample.name.clone().expect("the sample group is named");

    let found = cloud()
        .groups()
        .find_groups()
        .query(name.to_uppercase())
        .case_insensitive(true)
        .max_results(20)
        .send()
        .await
        .expect("a case-insensitive query is accepted");

    let ids: Vec<_> = found.groups.unwrap_or_default().into_iter().filter_map(|group| group.group_id).collect();

    assert!(ids.contains(&sample.group_id.clone().unwrap_or_default()), "{name} is found whatever the case");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn excludes_named_groups_from_the_results() {
    let sample = sample_group().await;
    let group_id = sample.group_id.clone().expect("the sample group has an id");

    let found = cloud()
        .groups()
        .find_groups()
        .exclude_id([group_id.as_str()])
        .max_results(50)
        .send()
        .await
        .expect("an exclusion is accepted");

    let ids: Vec<_> = found.groups.unwrap_or_default().into_iter().filter_map(|group| group.group_id).collect();

    assert!(!ids.contains(&group_id), "an excluded group stays out of the results: {ids:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_members_of_a_group_by_id_and_by_name_alike() {
    let sample = sample_group().await;
    let group_id = sample.group_id.clone().expect("the sample group has an id");
    let name = sample.name.clone().expect("the sample group is named");

    let by_id = cloud()
        .groups()
        .get_users_from_group()
        .group_id(group_id)
        .max_results(5)
        .send()
        .await
        .expect("a group's members read back by id");

    let by_name = cloud()
        .groups()
        .get_users_from_group()
        .groupname(name)
        .max_results(5)
        .send()
        .await
        .expect("a group's members read back by name");

    assert_eq!(by_name.total, by_id.total, "both addressing modes describe the same group");

    for user in &by_id.values {
        assert!(user.account_id.as_deref().is_some_and(|id| !id.is_empty()), "a member carries an id: {user:?}");
        assert!(user.active.is_some(), "a member says whether it is active: {user:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn excludes_inactive_members_unless_asked_for_them() {
    let sample = sample_group().await;
    let group_id = sample.group_id.clone().expect("the sample group has an id");

    let active = cloud()
        .groups()
        .get_users_from_group()
        .group_id(group_id.as_str())
        .max_results(50)
        .send()
        .await
        .expect("a group's active members read back");

    let with_inactive = cloud()
        .groups()
        .get_users_from_group()
        .group_id(group_id.as_str())
        .include_inactive_users(true)
        .max_results(50)
        .send()
        .await
        .expect("inactive members can be asked for");

    assert!(
        with_inactive.total >= active.total,
        "asking for inactive members never returns fewer: {} against {}",
        with_inactive.total,
        active.total,
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_group_as_not_found() {
    let error = cloud()
        .groups()
        .get_users_from_group()
        .groupname("no-such-group-jrs")
        .send()
        .await
        .expect_err("a group that does not exist has no members");

    assert!(error.is_not_found(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path_without_ever_aiming_it_at_a_real_group() {
    let error = cloud()
        .groups()
        .remove_group()
        .groupname("no-such-group-jrs")
        .send()
        .await
        .expect_err("a group that does not exist cannot be removed");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}
