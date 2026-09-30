use jira::server::WebhookInput;

use crate::harness::{ResourceTracker, server, test_name};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn registers_a_webhook_replaces_it_and_unregisters_it() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .webhooks()
        .create_webhook(WebhookInput {
            name: test_name("hook"),
            url: "https://example.com/jirars/created".to_owned(),
            events: Some(vec!["jira:issue_created".to_owned()]),
            ..WebhookInput::default()
        })
        .send()
        .await
        .expect("the instance registers a webhook");

    let webhook_id = created.id;

    tracker.defer(move || async move { server().webhooks().delete_webhook(webhook_id).send().await });

    assert_eq!(created.url, "https://example.com/jirars/created", "registration answers with the url it was given");
    assert!(
        created.events.iter().flatten().any(|event| event == "jira:issue_created"),
        "registration answers with the events it was given: {created:?}",
    );

    let read = server().webhooks().get_webhook(webhook_id).send().await.expect("the webhook reads back by id");

    assert_eq!(read.id, webhook_id, "the webhook read back is the webhook asked for");

    let updated = server()
        .webhooks()
        .update_webhook(
            webhook_id,
            WebhookInput {
                name: test_name("hook2"),
                url: "https://example.com/jirars/updated".to_owned(),
                events: Some(vec!["jira:issue_updated".to_owned()]),
                exclude_body: Some(true),
                ..WebhookInput::default()
            },
        )
        .send()
        .await
        .expect("the webhook is replaced wholesale");

    assert_eq!(updated.url, "https://example.com/jirars/updated", "the replacement answers with the new url");
    assert!(
        updated.events.iter().flatten().any(|event| event == "jira:issue_updated"),
        "the replacement answers with the new events: {updated:?}",
    );

    let plain = server().webhooks().get_webhooks().limit(50).send().await.expect("the instance lists its webhooks");
    let listed = plain.iter().find(|hook| hook.id == webhook_id).expect("the listing carries the webhook just made");

    assert!(listed.statistics.is_none(), "statistics are left out unless they are asked for: {listed:?}");

    let expanded = server()
        .webhooks()
        .get_webhooks()
        .limit(50)
        .statistics(true)
        .send()
        .await
        .expect("the listing takes a statistics flag");
    let with_statistics =
        expanded.iter().find(|hook| hook.id == webhook_id).expect("the expanded listing carries the webhook too");

    assert!(
        with_statistics.statistics.as_ref().is_some_and(|statistics| statistics.counts.is_some()),
        "asking for statistics puts delivery counts on each entry: {with_statistics:?}",
    );

    let by_event = server()
        .webhooks()
        .get_webhooks()
        .event("jira:issue_updated")
        .limit(50)
        .send()
        .await
        .expect("the listing takes an event filter");

    assert!(
        by_event.iter().any(|hook| hook.id == webhook_id),
        "the filter returns the webhook that was replaced to deliver that event",
    );

    server().webhooks().delete_webhook(webhook_id).send().await.expect("the webhook is unregistered");

    let gone = server()
        .webhooks()
        .get_webhook(webhook_id)
        .send()
        .await
        .expect_err("an unregistered webhook cannot be read back");

    assert!(gone.status().is_some_and(|status| (400..500).contains(&status)), "{gone}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reports_on_a_webhook_that_has_never_been_delivered() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .webhooks()
        .create_webhook(WebhookInput {
            name: test_name("hook stats"),
            url: "https://example.com/jirars/statistics".to_owned(),
            events: Some(vec!["jira:issue_updated".to_owned()]),
            ..WebhookInput::default()
        })
        .send()
        .await
        .expect("the instance registers a webhook");

    let webhook_id = created.id;

    tracker.defer(move || async move { server().webhooks().delete_webhook(webhook_id).send().await });

    let statistics = server()
        .webhooks()
        .get_webhook_statistics(webhook_id)
        .send()
        .await
        .expect("the instance reports how the webhook has been delivering");

    let counts = statistics.counts.expect("delivery statistics carry counts");

    assert_eq!(counts.successes, Some(0), "a webhook that has never fired has delivered nothing");
    assert!(
        counts.window.and_then(|window| window.duration).is_some_and(|duration| duration > 0),
        "the counts are over a window of some length",
    );

    let summary = server()
        .webhooks()
        .get_webhook_statistics_summary(webhook_id)
        .send()
        .await
        .expect("the instance reports one entry per event the webhook delivers");

    let for_event = summary.get("jira:issue_updated").expect("the summary is keyed by the event it delivers");

    assert!(for_event.counts.is_some(), "each entry in the summary carries its own counts");

    let transitions = server()
        .webhooks()
        .get_webhook_transitions(webhook_id)
        .send()
        .await
        .expect("the instance reports the transitions the webhook has been through");

    assert!(transitions.is_array(), "the transitions are a list: {transitions}");

    let latest = server()
        .webhooks()
        .get_latest_webhook_invocation(webhook_id)
        .send()
        .await
        .expect("the instance answers for a delivery that never happened");

    assert!(latest.is_null(), "there is no most recent delivery of a webhook that has never fired: {latest}");

    tracker.cleanup().await;
}
