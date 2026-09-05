//! The Teams API, which answers on the organization rather than on the site.
//!
//! Its fixtures are organization-level rather than project-level, which makes this suite the one place where the
//! resource tracker is doing something no sweep could do afterwards: a team is not scoped to a project, so nothing in
//! the issue purge would ever recognise it as debris. Every team is therefore registered for deletion the moment it
//! exists, and each test creates its own rather than sharing one.

use jira::teams::{
    BulkOperationRequest, MembershipFetchPayload, TeamCreationPayload, TeamCreationPayloadTeamType, TeamResponseState,
    TeamResponseWithMembers, TeamResponseWithMembersState, TeamResponseWithMembersTeamType, TeamUpdatePayload,
};

use crate::harness::{ResourceTracker, await_readable, await_refused, org_id, poll_until, teams, test_name};

/// Creates a team on the organization and registers its deletion.
///
/// Unlike every other fixture in these suites the resource is organization-level rather than project-level: a team is
/// not scoped to a project and no issue sweep will ever collect it. So the teardown is registered the moment the team
/// exists, and it treats `410 Gone` as success — a team the test deleted itself answers that rather than `404`, and
/// the tracker would otherwise report a resource that is demonstrably absent as leaked.
#[allow(deprecated, reason = "`site_id` is required by the payload and deprecated only in its documentation")]
async fn create_team(tracker: &mut ResourceTracker, org: &str, label: &str) -> TeamResponseWithMembers {
    let team = teams()
        .teams()
        .create_team(
            org,
            TeamCreationPayload {
                description: "Created by the jira live suite.".to_owned(),
                display_name: test_name(label),
                site_id: None,
                team_type: TeamCreationPayloadTeamType::MemberInvite,
            },
        )
        .send()
        .await
        .expect("the organization accepts a new team");

    let org = org.to_owned();
    let team_id = team.team_id.clone();

    tracker.defer(move || {
        let (org, team_id) = (org.clone(), team_id.clone());

        async move {
            match teams().teams().delete_team(org, team_id).send().await {
                Err(error) if error.status() == Some(410) => Ok(()),
                outcome => outcome,
            }
        }
    });

    team
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn creates_a_team_and_puts_the_creating_account_in_it() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "create").await;

    assert_eq!(team.team_id.len(), 36, "a team id is a UUID: {}", team.team_id);
    assert!(
        team.team_id.chars().all(|character| character.is_ascii_hexdigit() || character == '-'),
        "a team id is a UUID: {}",
        team.team_id,
    );
    assert_eq!(team.organization_id, org, "a team belongs to the organization it was created on");
    assert_eq!(team.state, TeamResponseWithMembersState::Active);
    assert_eq!(team.team_type, TeamResponseWithMembersTeamType::MemberInvite, "the type asked for is the type made");
    assert!(!team.members.is_empty(), "creating a team puts the creating account in it");
    assert!(team.user_permissions.delete_team, "whoever made the team may remove it");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reads_a_team_back_by_id() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "read").await;

    let read = teams().teams().get_team(&org, &team.team_id).send().await.expect("the team reads back by id");

    assert_eq!(read.team_id, team.team_id);
    assert_eq!(read.display_name, team.display_name);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_organization_teams_cursor_and_all() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "list").await;

    let page = teams().teams().query_teams(&org).size(300).send().await.expect("the organization lists its teams");

    let listed = page
        .entities
        .iter()
        .find(|entity| entity.team_id == team.team_id)
        .expect("a team that was just created is in the listing");

    assert_eq!(listed.display_name, team.display_name);
    assert_eq!(listed.organization_id, org);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn renames_a_team_and_the_change_survives_a_re_read() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "update").await;
    let renamed = test_name("update renamed");

    let updated = teams()
        .teams()
        .update_team(&org, &team.team_id, TeamUpdatePayload { description: None, display_name: Some(renamed.clone()) })
        .send()
        .await
        .expect("the team can be renamed");

    assert_eq!(updated.display_name, renamed);

    let read =
        await_readable("the renamed team reads back", || teams().teams().get_team(&org, &team.team_id).send()).await;

    assert_eq!(read.display_name, renamed, "the rename is observable on the next read");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn archives_and_unarchives_in_bulk_and_the_state_follows() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "archive").await;

    let archived = teams()
        .teams()
        .archive_teams(&org, BulkOperationRequest { team_ids: vec![team.team_id.clone()] })
        .send()
        .await
        .expect("the team can be archived");

    assert_eq!(archived.successful_team_ids, vec![team.team_id.clone()], "the bulk answer names what it acted on");
    assert!(archived.errors.is_empty(), "{:?}", archived.errors);

    poll_until("the team to report itself archived", || async {
        let after_archive =
            teams().teams().get_team(&org, &team.team_id).send().await.expect("the team still reads back");

        (after_archive.state == TeamResponseState::Archived).then_some(())
    })
    .await;

    let unarchived = teams()
        .teams()
        .unarchive_teams(&org, BulkOperationRequest { team_ids: vec![team.team_id.clone()] })
        .send()
        .await
        .expect("the team can be unarchived");

    assert_eq!(unarchived.successful_team_ids, vec![team.team_id.clone()]);

    poll_until("unarchiving to put the team back where it was", || async {
        let after_unarchive = teams().teams().get_team(&org, &team.team_id).send().await.expect("the team reads back");

        (after_unarchive.state == TeamResponseState::Active).then_some(())
    })
    .await;

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_members_which_come_back_under_a_cursor_of_their_own() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "members").await;

    let page = teams()
        .team_members()
        .fetch_members(&org, &team.team_id)
        .membership_fetch_payload(MembershipFetchPayload { after: None, first: Some(10) })
        .send()
        .await
        .expect("the team lists its members");

    assert!(!page.results.is_empty(), "the creating account is a member of what it created");
    assert!(
        page.results.iter().all(|member| !member.account_id.is_empty()),
        "a member is named by an account id and nothing else",
    );
    assert!(
        !page.page_info.has_next_page || page.page_info.end_cursor.is_some(),
        "a page that promises more says where to continue from",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn deletes_a_team_and_the_api_then_reports_it_gone_rather_than_missing() {
    let org = org_id().await;
    let mut tracker = ResourceTracker::new();
    let team = create_team(&mut tracker, &org, "delete").await;

    teams().teams().delete_team(&org, &team.team_id).send().await.expect("the team can be deleted");

    let error =
        await_refused("a deleted team cannot be read", || teams().teams().get_team(&org, &team.team_id).send()).await;

    assert!(error.is_api(), "{error}");
    assert_eq!(error.status(), Some(410), "a deleted team is gone, not absent: {error}");
    assert!(!error.is_not_found(), "410 and 404 are different answers, and Teams gives the first");

    tracker.cleanup().await;
}
