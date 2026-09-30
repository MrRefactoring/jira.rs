use jira::cloud::{
    AddNotificationsDetails, GetNotificationSchemeRequestExpand, GetNotificationSchemeRequestExpandValue,
};

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_scheme_listing_or_refuses_typed_without_rights() {
    let page = match cloud().issue_notification_schemes().get_notification_schemes().max_results("2").send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token that may administer no project is refused typed: {error}",
            );

            return;
        }
    };

    assert_eq!(page.max_results, 2, "the page size asked for is the page size returned");
    assert!(page.values.len() <= 2, "a page holds no more than it says it does");
    assert!(page.is_last || page.values.len() == 2, "a page that is not the last one is full");

    for scheme in &page.values {
        assert!(scheme.id.is_some_and(|id| id > 0), "a notification scheme carries an id");
        assert!(scheme.name.as_ref().is_some_and(|name| !name.is_empty()), "a notification scheme carries a name");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_notification_events_only_when_expanded() {
    let Some(scheme_id) = first_scheme_id().await else {
        return;
    };

    let plain = cloud()
        .issue_notification_schemes()
        .get_notification_scheme(scheme_id)
        .send()
        .await
        .expect("a notification scheme reads back by id");

    let expanded = cloud()
        .issue_notification_schemes()
        .get_notification_scheme(scheme_id)
        .expand(GetNotificationSchemeRequestExpand::One(GetNotificationSchemeRequestExpandValue::All))
        .send()
        .await
        .expect("the expand parameter is accepted");

    assert!(plain.notification_scheme_events.is_none(), "an unexpanded scheme carries no events");
    assert!(expanded.notification_scheme_events.is_some(), "an expanded scheme carries its events");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pairs_each_event_with_the_holders_that_get_notified() {
    let Some(scheme_id) = first_scheme_id().await else {
        return;
    };

    let scheme = cloud()
        .issue_notification_schemes()
        .get_notification_scheme(scheme_id)
        .expand(GetNotificationSchemeRequestExpand::One(GetNotificationSchemeRequestExpandValue::All))
        .send()
        .await
        .expect("an expanded notification scheme reads back");

    let events = scheme.notification_scheme_events.expect("an expanded scheme carries its events");

    for event in &events {
        assert!(
            event.event.as_ref().and_then(|event| event.id).is_some_and(|id| id > 0),
            "an entry names the event it fires on",
        );

        let notifications = event.notifications.as_ref().expect("an event carries its recipients, empty or not");

        for notification in notifications {
            let kind = notification.notification_type.as_ref().expect("a recipient names how it is addressed");

            assert!(kind.is_documented(), "a recipient kind is one the specification lists, got {}", kind.as_str());
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn maps_schemes_to_the_projects_using_them() {
    let page = match cloud().issue_notification_schemes().get_notification_scheme_to_project_mappings().send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden() || error.status() == Some(401), "a refused mapping listing is typed: {error}",);

            return;
        }
    };

    assert!(page.max_results > 0, "a page declares the size it was capped at");

    for mapping in &page.values {
        assert!(mapping.notification_scheme_id.as_ref().is_some_and(|id| !id.is_empty()), "a mapping names the scheme",);
        assert!(mapping.project_id.as_ref().is_some_and(|id| !id.is_empty()), "a mapping names the project");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_scheme_as_a_typed_error() {
    let error = cloud()
        .issue_notification_schemes()
        .get_notification_scheme(99_999_999)
        .send()
        .await
        .expect_err("a notification scheme that does not exist cannot be read");

    assert!(error.is_not_found() || error.is_forbidden(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_write_without_ever_aiming_it_at_a_real_scheme() {
    let error = cloud()
        .issue_notification_schemes()
        .add_notifications(
            "99999999",
            AddNotificationsDetails { notification_scheme_events: Vec::new(), ..AddNotificationsDetails::default() },
        )
        .send()
        .await
        .expect_err("a scheme that does not exist cannot take notifications");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

async fn first_scheme_id() -> Option<i64> {
    let page = match cloud().issue_notification_schemes().get_notification_schemes().max_results("5").send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token that may administer no project is refused typed: {error}",
            );

            return None;
        }
    };

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");

    page.values.first().and_then(|scheme| scheme.id)
}
