use jira::service_desk_server::QueueCreate;

use super::fixtures::{asset_name, service_desk_licensed, service_desk_project};
use crate::harness::service_desk_server;

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_reads_updates_and_deletes_a_queue() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();

    let created = service_desk_server()
        .queues()
        .create_queue(desk.clone())
        .queue_create(QueueCreate {
            name: Some(asset_name("queue")),
            jql: Some(format!("project = {}", project.project_key)),
            fields: Some(vec!["issuekey".to_owned(), "summary".to_owned()]),
        })
        .send()
        .await
        .expect("a service desk accepts a queue");

    let id = created.id.clone().expect("a created queue carries an id");

    let listed =
        service_desk_server().queues().get_queues(desk.clone()).send().await.expect("a service desk lists its queues");

    assert!(listed.values.iter().any(|queue| queue.id == created.id), "the queue just made is among them");

    let loaded = service_desk_server()
        .queues()
        .get_queue(id.clone(), desk.clone())
        .send()
        .await
        .expect("a queue reads back by id");

    assert_eq!(loaded.id, created.id, "the queue read back is the one asked for");

    let updated = service_desk_server()
        .queues()
        .update_queue(id.clone(), desk.clone())
        .queue_create(QueueCreate {
            name: Some(asset_name("renamed queue")),
            jql: Some(format!("project = {}", project.project_key)),
            fields: Some(vec!["issuekey".to_owned()]),
        })
        .send()
        .await
        .expect("a queue can be rewritten");

    assert!(
        updated.name.as_deref().is_some_and(|name| name.contains("renamed queue")),
        "the name the body carried is the name that stuck",
    );

    let issues = service_desk_server()
        .queues()
        .get_issues_in_queue(id.clone(), desk.clone())
        .send()
        .await
        .expect("a queue lists the issues in it");

    assert!(
        issues.values.iter().all(|issue| issue.key.is_some()),
        "every issue the queue lists is addressable by a key",
    );

    service_desk_server().queues().delete_queue(id, desk).send().await.expect("a queue can be removed");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reorders_the_queues_of_a_service_desk() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();

    let queues =
        service_desk_server().queues().get_queues(desk.clone()).send().await.expect("a service desk lists its queues");

    let mut order: Vec<i64> = queues.values.iter().filter_map(|queue| queue.id.as_ref()?.parse().ok()).collect();

    assert!(!order.is_empty(), "the service desk template brings queues with it");

    order.reverse();

    service_desk_server()
        .queues()
        .reorder_queues(desk)
        .body(order)
        .send()
        .await
        .expect("the queues of a service desk can be reordered, and the body is every id in the new order");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reads_and_writes_the_queue_count_settings() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;

    service_desk_server()
        .queue_settings()
        .get_queue_settings_on_project(project.project_key.clone())
        .send()
        .await
        .expect("a project reports its queue settings");

    service_desk_server()
        .queue_settings()
        .set_should_queues_include_count_on_project(project.project_key.clone())
        .body(true)
        .send()
        .await
        .expect("a project's queues can be told to carry a count");

    service_desk_server()
        .queue_settings()
        .set_should_queues_use_count_cache_on_project(project.project_key.clone())
        .body(true)
        .send()
        .await
        .expect("a project's queue counts can be told to come from a cache");

    service_desk_server()
        .queue_settings()
        .set_should_queues_include_count_globally()
        .body(true)
        .send()
        .await
        .expect("the instance can be told the same thing for every project");

    service_desk_server()
        .queue_settings()
        .set_should_queues_use_count_cache_globally()
        .body(true)
        .send()
        .await
        .expect("and the same for the cache");
}
