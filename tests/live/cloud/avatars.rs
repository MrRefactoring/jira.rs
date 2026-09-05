//! Ported from jira.js/tests/live/cloud/avatars.test.ts.
//!
//! The `avatars` surface, both directions of which carry bytes.
//!
//! The specification describes every one of these operations as JSON, and three of them answer with an image
//! instead. So both halves are asserted here: an image comes back as an image rather than as a parse failure that
//! reads like a corrupt response, and an upload is accepted as image bytes rather than as an object. Where the
//! TypeScript suite reads the media type off the `Blob`, the Rust operations hand back a bare `bytes::Bytes` with no
//! type attached, so the bytes themselves are what says whether an image arrived.
//!
//! The upload is the one write this file makes — a custom avatar added to the test project and deleted again.
//! Nothing selects it, so what the project displays never changes.

use jira::cloud::{
    Avatar, DeleteAvatarRequestType, GetAllSystemAvatarsRequestType, GetAvatarImageByIDRequestType,
    GetAvatarImageByOwnerRequestType, GetAvatarImageByTypeRequestType, GetAvatarsRequestType, StoreAvatarRequestType,
};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, poll_until};

/// The side, in pixels, of both the uploaded image and the crop asked of it.
const AVATAR_SIDE: u32 = 48;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_system_avatars_for_each_type() {
    let types = [
        GetAllSystemAvatarsRequestType::Project,
        GetAllSystemAvatarsRequestType::Issuetype,
        GetAllSystemAvatarsRequestType::User,
        GetAllSystemAvatarsRequestType::Priority,
    ];

    for r#type in types {
        let avatars = cloud()
            .avatars()
            .get_all_system_avatars(r#type.clone())
            .send()
            .await
            .expect("the site lists the avatars it ships with");

        let system = avatars.system.unwrap_or_default();

        assert!(!system.is_empty(), "Jira ships system avatars for every owner type, none for {type}");

        for avatar in &system {
            assert!(!avatar.id.is_empty(), "an avatar carries an id: {avatar:?}");
            assert_eq!(avatar.is_system_avatar, Some(true), "a system avatar says so: {avatar:?}");
            assert_eq!(avatar.is_deletable, Some(false), "one of Jira's own avatars cannot be deleted: {avatar:?}");
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn separates_system_from_custom_avatars_for_a_project() {
    let project_id = test_project_id().await;

    let avatars = cloud()
        .avatars()
        .get_avatars(GetAvatarsRequestType::Project, &project_id)
        .send()
        .await
        .expect("a project lists the avatars available to it");

    assert!(!avatars.system.unwrap_or_default().is_empty(), "the system catalogue is offered to every project");

    for avatar in avatars.custom.unwrap_or_default() {
        assert_eq!(avatar.is_system_avatar, Some(false), "an avatar listed as custom is not a system one: {avatar:?}");
    }
}

/// The operation the specification types as JSON and the API answers with a file.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_an_avatar_image_as_bytes_rather_than_json() {
    let avatar = a_system_project_avatar().await;
    let id = avatar.id.parse::<i64>().expect("an avatar id is a number");

    let image = cloud()
        .avatars()
        .get_avatar_image_by_id(GetAvatarImageByIDRequestType::Project, id)
        .send()
        .await
        .expect("an avatar serves its own image");

    assert!(image.len() > 50, "an image is more than a handful of bytes, got {}", image.len());
    assert!(looks_like_an_image(&image), "the bytes are an image: {:?}", &image[..image.len().min(16)]);
    assert!(
        serde_json::from_slice::<serde_json::Value>(&image).is_err(),
        "the response is a file, not the JSON the specification describes",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_default_image_for_a_type_and_the_image_of_an_owner() {
    let project_id = test_project_id().await;

    let by_type = cloud()
        .avatars()
        .get_avatar_image_by_type(GetAvatarImageByTypeRequestType::Project)
        .send()
        .await
        .expect("a type serves its default avatar image");

    let by_owner = cloud()
        .avatars()
        .get_avatar_image_by_owner(GetAvatarImageByOwnerRequestType::Project, &project_id)
        .send()
        .await
        .expect("a project serves the avatar image it displays");

    for image in [&by_type, &by_owner] {
        assert!(image.len() > 50, "an image is more than a handful of bytes, got {}", image.len());
        assert!(looks_like_an_image(image), "the bytes are an image: {:?}", &image[..image.len().min(16)]);
    }
}

/// The one write: an avatar built from image bytes, listed as custom, served back, and deleted again.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn stores_a_custom_avatar_from_image_bytes_and_serves_it_back() {
    let mut tracker = ResourceTracker::new();
    let project_id = test_project_id().await;

    let stored = cloud()
        .avatars()
        .store_avatar(StoreAvatarRequestType::Project, &project_id, i64::from(AVATAR_SIDE), png_bytes(AVATAR_SIDE))
        // Jira reads the declared media type rather than sniffing the bytes, and refuses an upload without one as
        // "not a supported image format".
        .content_type("image/png")
        .x(0)
        .y(0)
        .send()
        .await
        .expect("a project takes an avatar built from image bytes");

    assert!(!stored.id.is_empty(), "a stored avatar carries an id");
    assert_eq!(stored.is_system_avatar, Some(false), "an uploaded avatar is not one of Jira's own");
    assert_eq!(stored.is_deletable, Some(true), "what was uploaded can be removed again");

    let avatar_id = stored.id.parse::<i64>().expect("an avatar id is a number");
    let owner = project_id.clone();

    tracker.defer(move || {
        let owner = owner.clone();

        async move { cloud().avatars().delete_avatar(DeleteAvatarRequestType::Project, owner, avatar_id).send().await }
    });

    poll_until("the upload to be offered to the project as a custom avatar", || async {
        let avatars = cloud()
            .avatars()
            .get_avatars(GetAvatarsRequestType::Project, &project_id)
            .send()
            .await
            .expect("the project lists the avatars available to it");

        avatars.custom.unwrap_or_default().into_iter().find(|candidate| candidate.id == stored.id)
    })
    .await;

    let image = cloud()
        .avatars()
        .get_avatar_image_by_id(GetAvatarImageByIDRequestType::Project, avatar_id)
        .send()
        .await
        .expect("the avatar serves the image it was given");

    assert!(image.len() > 50, "an image is more than a handful of bytes, got {}", image.len());
    assert_eq!(&image[..4], b"\x89PNG", "Jira serves back the PNG it was handed");

    tracker.cleanup().await;
}

/// Two ways of asking about something that does not exist, neither of which Jira treats as an error.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_an_unknown_type_and_an_unknown_entity_without_failing() {
    let unknown_type = cloud()
        .avatars()
        .get_all_system_avatars("nosuchtype")
        .send()
        .await
        .expect("an owner type Jira has never heard of is answered rather than refused");

    assert!(unknown_type.system.unwrap_or_default().is_empty(), "an unknown type owns no avatars");

    let unknown_entity = cloud()
        .avatars()
        .get_avatars(GetAvatarsRequestType::Project, "99999999")
        .send()
        .await
        .expect("a project that does not exist is answered rather than refused");

    assert!(
        !unknown_entity.system.unwrap_or_default().is_empty(),
        "an unknown entity is still offered the whole system catalogue",
    );
    assert!(unknown_entity.custom.unwrap_or_default().is_empty(), "an unknown entity has no custom avatars of its own");
}

/// The destructive path, proven through its error channel and never aimed at an avatar that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .avatars()
        .delete_avatar(DeleteAvatarRequestType::Project, "99999999", 99_999_999)
        .send()
        .await
        .expect_err("an avatar that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "a refused delete is typed: {error}");
}

/// The id of the project the suites work in, read rather than assumed.
async fn test_project_id() -> String {
    cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back by key")
        .id
        .expect("a project carries an id")
}

/// One of Jira's own project avatars, used as a fixture for the read paths.
async fn a_system_project_avatar() -> Avatar {
    cloud()
        .avatars()
        .get_all_system_avatars(GetAllSystemAvatarsRequestType::Project)
        .send()
        .await
        .expect("the site lists the project avatars it ships with")
        .system
        .unwrap_or_default()
        .into_iter()
        .next()
        .expect("Jira ships at least one project avatar")
}

/// Whether the bytes open the way one of the formats Jira serves avatars in opens.
///
/// The response carries a media type and the operation does not hand it over, so this stands in for reading the
/// header: PNG, GIF, JPEG and SVG all announce themselves in their first bytes.
fn looks_like_an_image(bytes: &[u8]) -> bool {
    let leading = &bytes[..bytes.len().min(64)];

    bytes.starts_with(b"\x89PNG")
        || bytes.starts_with(b"GIF8")
        || bytes.starts_with(b"\xff\xd8\xff")
        || leading.windows(4).any(|window| window == b"<svg")
        || leading.windows(5).any(|window| window == b"<?xml")
}

/// A valid PNG of a solid colour, built here rather than committed as a fixture.
///
/// The avatar endpoints read the image: Jira answers "not a supported image format" to anything it cannot decode, so
/// a placeholder of arbitrary bytes proves nothing. Generating one keeps the suite free of a binary fixture, and
/// keeps the size a parameter — Jira refuses an avatar smaller than the crop it is asked for.
fn png_bytes(side: u32) -> Vec<u8> {
    let side_at = usize::try_from(side).expect("a side fits an index");

    // Every row is the filter byte the format demands and then one solid-colour pixel per column.
    let mut row = vec![0u8];

    row.extend_from_slice(&[200, 60, 60].repeat(side_at));

    let scanlines = row.repeat(side_at);
    let mut header = Vec::with_capacity(13);

    header.extend_from_slice(&side.to_be_bytes());
    header.extend_from_slice(&side.to_be_bytes());
    header.extend_from_slice(&[8, 2, 0, 0, 0]);

    let mut png = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

    png.extend(png_chunk(b"IHDR", &header));
    png.extend(png_chunk(b"IDAT", &deflate_stored(&scanlines)));
    png.extend(png_chunk(b"IEND", &[]));

    png
}

fn png_chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let length = u32::try_from(data.len()).expect("a chunk is smaller than four gigabytes");
    let mut framed = Vec::with_capacity(12 + data.len());

    framed.extend_from_slice(&length.to_be_bytes());
    framed.extend_from_slice(kind);
    framed.extend_from_slice(data);

    let crc = crc32(&framed[4..]);

    framed.extend_from_slice(&crc.to_be_bytes());

    framed
}

/// A zlib stream of uncompressed blocks: a valid deflate encoding, and the only one expressible without a compressor.
fn deflate_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    let mut blocks = data.chunks(0xFFFF).peekable();

    while let Some(block) = blocks.next() {
        let length = u16::try_from(block.len()).expect("a block is at most sixty-five kilobytes");

        out.push(u8::from(blocks.peek().is_none()));
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(&(!length).to_le_bytes());
        out.extend_from_slice(block);
    }

    out.extend_from_slice(&adler32(data).to_be_bytes());

    out
}

fn adler32(data: &[u8]) -> u32 {
    let (mut low, mut high) = (1u32, 0u32);

    for byte in data {
        low = (low + u32::from(*byte)) % 65521;
        high = (high + low) % 65521;
    }

    (high << 16) | low
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;

    for byte in data {
        crc ^= u32::from(*byte);

        for _ in 0..8 {
            crc = if crc & 1 == 1 { 0xEDB8_8320 ^ (crc >> 1) } else { crc >> 1 };
        }
    }

    !crc
}
