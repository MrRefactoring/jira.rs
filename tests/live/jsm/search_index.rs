//! The Assets index, which this file is allowed to take away from everything else.
//!
//! A reindex holds the index for as long as it runs and answers stale until it finishes, so nothing that finds an
//! object through the index may run after this. The suites run in name order and this one sorts after `schemas`; the
//! only file behind it is `service_desk`, which reads nothing out of Assets.

use crate::harness::assets_server;

/// The document declares `IndexPath` with no properties at all, so the generated type is empty and the path it
/// carries is unreachable through it.
///
/// That gap is Atlassian's rather than the client's, and pinning it here is what makes it visible: the raw body is
/// read instead, and the day the document grows the property this test is what says so.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_where_the_index_lives() {
    let path = assets_server()
        .index_configuration()
        .get_index_path()
        .send_raw()
        .await
        .expect("the instance reports where its index lives");

    assert!(path.to_string().contains("insight"), "the index lives under the Assets directory: {path}");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn checks_the_integrity_of_the_index_on_this_node() {
    let integrity = assets_server()
        .index_configuration()
        .check_index_integrity()
        .send()
        .await
        .expect("the node checks its own index");

    assert!(integrity.object_index_ok.is_some(), "the check reports on the object index either way");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn writes_the_index_to_disk() {
    assets_server()
        .index_configuration()
        .persist_index_to_file()
        .send()
        .await
        .expect("the index can be written to disk");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reindexes_this_node() {
    let progress = assets_server()
        .index_configuration()
        .start_reindex_current_node()
        .send()
        .await
        .expect("a reindex of this node starts");

    assert!(progress.category.is_some(), "the reindex is queued as a task that names its category");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reindexes_the_whole_of_assets() {
    let progress = assets_server()
        .index_configuration()
        .start_reindex_insight()
        .clean("false")
        .send()
        .await
        .expect("a reindex of the whole of Assets starts");

    assert!(progress.category.is_some(), "the reindex is queued as a task that names its category");
}
