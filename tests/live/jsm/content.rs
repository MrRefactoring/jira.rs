//! The bytes and prose hung off an object: comments, attachments, the icon it wears and the QR code it prints.

use jira::assets_server::Comment;

use super::fixtures::{asset_name, fixtures};
use crate::harness::assets_server;

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn comments_on_an_object_and_reads_the_comment_back() {
    let fixtures = fixtures().await;

    let created = assets_server()
        .comments()
        .create_comment()
        .comment(Comment {
            object_id: Some(fixtures.object_id),
            comment: Some(asset_name("comment")),
            role: Some(0),
            ..Comment::default()
        })
        .send()
        .await
        .expect("an object accepts a comment");

    let id = created.id.expect("a created comment carries an id");

    let comments = assets_server()
        .comments()
        .get_comments(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object lists its comments");

    assert!(comments.iter().any(|comment| comment.id == Some(id)), "the comment just written is among them");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn attaches_a_file_to_an_object_lists_it_and_removes_it() {
    let fixtures = fixtures().await;

    let attached = assets_server()
        .attachments()
        .add_attachments(
            fixtures.object_id.to_string(),
            [jira::Attachment::new("jira-rs.txt", &b"attached by the live suite"[..])],
        )
        .send()
        .await
        .expect("an object accepts an attachment");

    let id = attached.first().and_then(|attachment| attachment.id).expect("an uploaded attachment carries an id");

    let listed = assets_server()
        .attachments()
        .get_attachments(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object lists its attachments");

    assert!(listed.iter().any(|attachment| attachment.id == Some(id)), "the attachment just uploaded is among them");

    assets_server().attachments().delete_attachment(id.to_string()).send().await.expect("an attachment can be removed");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_global_icons_and_loads_one() {
    let fixtures = fixtures().await;

    let icons = assets_server().icons().find_global_icons().send().await.expect("the instance lists its global icons");

    assert!(icons.iter().any(|icon| icon.id == Some(fixtures.icon_id)), "the icon the fixtures chose is a global one");

    let icon =
        assets_server().icons().get_icon(fixtures.icon_id.to_string()).send().await.expect("an icon reads back by id");

    assert_eq!(icon.id, Some(fixtures.icon_id), "the icon read back is the one asked for");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_icons_a_schema_declares_of_its_own() {
    let fixtures = fixtures().await;

    let icons = assets_server()
        .icons()
        .find_icons(fixtures.schema_id.to_string())
        .send()
        .await
        .expect("a schema lists the icons it declares");

    assert!(icons.iter().all(|icon| icon.id.is_some()), "every icon a schema declares is addressable by an id");
}

/// The one endpoint on this surface that answers with an image.
///
/// The document declares an empty `application/json` body for it, so what is proven here is that the bytes arrive as
/// a PNG rather than through the JSON parser — a generated call that trusted the declared media type would fail on
/// the first byte.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn prints_an_object_as_a_qr_code() {
    let fixtures = fixtures().await;

    let png = assets_server()
        .qr_code()
        .get_object_qr_code(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object prints as a QR code");

    assert_eq!(&png[..4], &[0x89, 0x50, 0x4e, 0x47], "the bytes are a PNG rather than a parsed document");
}
