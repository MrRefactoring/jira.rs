use jira::cloud::{
    DeleteAndReplaceVersion, GetProjectVersionsPaginatedRequestOrderBy, GetVersionRequestExpand, IssueUpdateDetails,
    Version, VersionMove, VersionMovePosition,
};
use serde_json::json;

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, poll_until, test_name};

/// A version, from creation to merge, inside the standing test project.
///
/// Versions are the one piece of project configuration with genuinely interesting semantics — they order relative to
/// each other, they merge, and deleting one has to say what happens to the issues that referenced it. All three are
/// walked here, because none of them is visible in the types.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_version_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("version").replace(['[', ']'], "");

    let project = cloud().projects().get_project(TEST_PROJECT_KEY).send().await.expect("the test project reads back");
    let project_id: i64 = project.id.expect("a project carries an id").parse().expect("a project id is a number");

    let created = cloud()
        .project_versions()
        .create_version(Version {
            name: Some(name.clone()),
            description: Some("created by the live suite".to_owned()),
            project_id: Some(project_id),
            released: Some(false),
            ..Version::default()
        })
        .send()
        .await
        .expect("the test project accepts a new version");

    let version_id = created.id.clone().expect("a created version carries an id");

    {
        let id = version_id.clone();

        tracker.defer(move || {
            let id = id.clone();

            async move {
                cloud()
                    .project_versions()
                    .delete_and_replace_version(id, DeleteAndReplaceVersion::default())
                    .send()
                    .await
            }
        });
    }

    assert!(version_id.chars().all(|c| c.is_ascii_digit()), "an id is digits: {version_id}");
    assert_eq!(created.name.as_deref(), Some(name.as_str()));
    assert_eq!(created.released, Some(false));
    assert_eq!(created.archived, Some(false));

    let read = cloud().project_versions().get_version(&version_id).send().await.expect("the version reads back by id");

    assert_eq!(read.id.as_deref(), Some(version_id.as_str()));
    assert_eq!(read.project_id, Some(project_id));
    assert!(read.issues_status_for_fix_version.is_none(), "issue counts are not returned unless asked for");

    let expanded = cloud()
        .project_versions()
        .get_version(&version_id)
        .expand(GetVersionRequestExpand::Variant1(vec!["issuesstatus".to_owned()]))
        .send()
        .await
        .expect("the expand parameter is accepted");

    let counts = expanded.issues_status_for_fix_version.expect("expanding issuesstatus returns the counts");

    assert!(counts.unmapped.is_some(), "the counts carry an unmapped bucket");

    let updated = cloud()
        .project_versions()
        .update_version(
            &version_id,
            Version { released: Some(true), release_date: Some("2026-01-15".to_owned()), ..Version::default() },
        )
        .send()
        .await
        .expect("the version can be released");

    assert_eq!(updated.released, Some(true), "the release is observable in the response");
    assert!(updated.release_date.is_some_and(|date| !date.is_empty()), "a released version carries a release date");

    let second = cloud()
        .project_versions()
        .create_version(Version {
            name: Some(format!("{name}-2")),
            project_id: Some(project_id),
            released: Some(false),
            ..Version::default()
        })
        .send()
        .await
        .expect("the test project accepts a second version");

    let second_id = second.id.clone().expect("a created version carries an id");

    {
        let id = second_id.clone();

        tracker.defer(move || {
            let id = id.clone();

            async move {
                cloud()
                    .project_versions()
                    .delete_and_replace_version(id, DeleteAndReplaceVersion::default())
                    .send()
                    .await
            }
        });
    }

    let before = cloud()
        .project_versions()
        .get_project_versions(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project lists its versions");
    let order_before: Vec<Option<String>> = before.iter().map(|version| version.id.clone()).collect();

    cloud()
        .project_versions()
        .move_version(&second_id, VersionMove { position: Some(VersionMovePosition::First), ..VersionMove::default() })
        .send()
        .await
        .expect("a version can be moved to the front");

    let after = cloud()
        .project_versions()
        .get_project_versions(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the reordered project lists its versions");
    let order_after: Vec<Option<String>> = after.iter().map(|version| version.id.clone()).collect();

    assert_eq!(order_after.first().and_then(Clone::clone).as_deref(), Some(second_id.as_str()));
    assert_ne!(order_after, order_before, "moving a version changes the order the project reports");

    let issue = create_test_issue(&mut tracker, Some(&test_name("version holder"))).await;

    cloud()
        .issues()
        .edit_issue(
            &issue.key,
            IssueUpdateDetails {
                fields: Some([("fixVersions".to_owned(), json!([{ "id": version_id }]))].into_iter().collect()),
                ..IssueUpdateDetails::default()
            },
        )
        .send()
        .await
        .expect("the version can be set as a fix version");

    let related = poll_until("the fix version count to catch up", || async {
        let counts = cloud()
            .project_versions()
            .get_version_related_issues(&version_id)
            .send()
            .await
            .expect("the version reports the issues that reference it");

        counts.issues_fixed_count.filter(|count| *count == 1).map(|_| counts)
    })
    .await;

    assert_eq!(related.issues_fixed_count, Some(1));

    let unresolved = poll_until("the unresolved issue count to catch up", || async {
        let counts = cloud()
            .project_versions()
            .get_version_unresolved_issues(&version_id)
            .send()
            .await
            .expect("the version reports its unresolved issues");

        counts.issues_count.filter(|count| *count == 1).map(|_| counts)
    })
    .await;

    assert_eq!(unresolved.issues_count, Some(1));

    cloud()
        .project_versions()
        .merge_versions(&version_id, &second_id)
        .send()
        .await
        .expect("one version can be merged into another");

    let gone = cloud()
        .project_versions()
        .get_version(&version_id)
        .send()
        .await
        .expect_err("a merged version no longer exists");

    assert!(gone.is_not_found(), "{gone}");

    let fetched = cloud()
        .issues()
        .get_issue(&issue.key)
        .fields(["fixVersions"])
        .send()
        .await
        .expect("the issue that carried the version reads back");

    let fix_versions: Vec<Option<&str>> = fetched
        .fields
        .as_ref()
        .and_then(|fields| fields.get("fixVersions"))
        .and_then(|value| value.as_array())
        .expect("an issue reports its fix versions as a list")
        .iter()
        .map(|version| version.get("id").and_then(serde_json::Value::as_str))
        .collect();

    assert_eq!(fix_versions, vec![Some(second_id.as_str())], "the merge moves the issue onto the surviving version");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_paginated_version_listing() {
    let page = cloud()
        .project_versions()
        .get_project_versions_paginated(TEST_PROJECT_KEY)
        .max_results(1)
        .order_by(GetProjectVersionsPaginatedRequestOrderBy::Name)
        .send()
        .await
        .expect("the paginated version listing is accepted");

    assert!(page.values.len() <= 1, "one result was asked for, {} arrived", page.values.len());
    assert_eq!(page.max_results, 1);
    assert!(page.total >= page.values.len() as i64, "a page never reports fewer results than it carries");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_version_as_not_found() {
    let error = cloud()
        .project_versions()
        .get_version("99999999")
        .send()
        .await
        .expect_err("a version that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}
