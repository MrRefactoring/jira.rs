use jira::service_desk::RequestCreate;

use crate::harness::service_desk;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_customer_requests_or_refuses_typed_without_an_agent_licence() {
    let page = match service_desk().request().get_customer_requests().limit(5).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden(), "an account without an agent licence is refused by rights: {error}");
            assert_eq!(error.status(), Some(403), "and the refusal keeps its status: {error}");

            return;
        }
    };

    assert!(page.is_last_page.is_some(), "a page says whether it is the last one");
    assert!(page.values.len() <= 5, "a page holds no more than the limit asked for: {}", page.values.len());

    for request in &page.values {
        assert!(request.issue_id.as_ref().is_some_and(|id| !id.is_empty()), "a request is an issue, with an id");
        assert!(request.issue_key.is_some(), "and that issue's key");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_single_request_lookup_typed() {
    let error = service_desk()
        .request()
        .get_customer_request_by_id_or_key("NOSUCH-1")
        .send()
        .await
        .expect_err("a request that does not exist cannot be read");

    assert!(
        error.is_forbidden() || error.is_not_found(),
        "the lookup is refused by licence or reported missing, never untyped: {error}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_comment_reads_typed() {
    let error = service_desk()
        .request()
        .get_request_comments("NOSUCH-1")
        .limit(5)
        .send()
        .await
        .expect_err("the comments of a request that does not exist cannot be read");

    assert!(
        error.is_forbidden() || error.is_not_found(),
        "the comment read is refused by licence or reported missing, never untyped: {error}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_sla_reads_typed() {
    let error = service_desk()
        .request()
        .get_sla_information("NOSUCH-1")
        .send()
        .await
        .expect_err("the SLAs of a request that does not exist cannot be read");

    assert!(
        error.is_forbidden() || error.is_not_found(),
        "the SLA read is refused by licence or reported missing, never untyped: {error}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_participant_and_subscription_reads_typed() {
    let participants = service_desk()
        .request()
        .get_request_participants("NOSUCH-1")
        .send()
        .await
        .expect_err("the participants of a request that does not exist cannot be read");

    let subscription = service_desk()
        .request()
        .get_subscription_status("NOSUCH-1")
        .send()
        .await
        .expect_err("the subscription status of a request that does not exist cannot be read");

    assert!(
        participants.is_forbidden() || participants.is_not_found(),
        "the participant read is refused by licence or reported missing: {participants}",
    );
    assert!(
        subscription.is_forbidden() || subscription.is_not_found(),
        "the subscription read is refused by licence or reported missing: {subscription}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn never_opens_a_real_ticket_and_fails_typed_on_the_attempt() {
    let error = service_desk()
        .request()
        .create_customer_request(RequestCreate {
            service_desk_id: Some("99999999".to_owned()),
            request_type_id: Some("99999999".to_owned()),
            ..RequestCreate::default()
        })
        .send()
        .await
        .expect_err("a service desk that does not exist cannot take a request");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "the refused write is typed: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn searches_the_knowledge_base_or_refuses_typed() {
    let page = match service_desk().knowledgebase().get_articles("jirars live test", false).limit(5).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(404),
                "the search is refused by licence or the knowledge base is absent: {error}",
            );

            return;
        }
    };

    assert!(page.values.len() <= 5, "a page holds no more than the limit asked for: {}", page.values.len());

    for article in &page.values {
        assert!(article.title.is_some(), "a found article names itself");
    }
}
