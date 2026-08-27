//! Ported from jira.js/tests/live/cloud/projectAvatars.test.ts.
//!
//! `create_project_avatar` takes image bytes, which the specification describes as an object of arbitrary keys — the
//! shape that made this endpoint unusable before. Adding an avatar to a project is a write, but a contained one: it
//! lands in the project's list of custom avatars and is deleted again here. `update_project_avatar`, which would
//! select one as the project's displayed avatar, is deliberately not called — that changes what everyone sees.

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, poll_until};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn stores_an_avatar_for_the_project_from_image_bytes() {
    let mut tracker = ResourceTracker::new();

    let avatar = cloud()
        .project_avatars()
        .create_project_avatar(TEST_PROJECT_KEY, png_bytes(48))
        // Jira reads the declared media type rather than sniffing the bytes, and refuses an upload without one as
        // "not a supported image format".
        .content_type("image/png")
        .size(48)
        .x(0)
        .y(0)
        .send()
        .await
        .expect("the project takes an avatar built from image bytes");

    assert!(!avatar.id.is_empty(), "a stored avatar carries an id");
    assert_eq!(avatar.is_system_avatar, Some(false), "an uploaded avatar is not one of Jira's own");

    let avatar_id: i64 = avatar.id.parse().expect("an avatar id is a number");

    tracker.defer(move || async move {
        cloud().project_avatars().delete_project_avatar(TEST_PROJECT_KEY, avatar_id).send().await
    });

    let avatars = poll_until("the upload to be offered to the project as a custom avatar", || async {
        let avatars = cloud()
            .project_avatars()
            .get_all_project_avatars(TEST_PROJECT_KEY)
            .send()
            .await
            .expect("the project lists the avatars available to it");

        let listed = avatars.custom.clone().unwrap_or_default().iter().any(|candidate| candidate.id == avatar.id);

        listed.then_some(avatars)
    })
    .await;

    assert!(!avatars.system.unwrap_or_default().is_empty(), "a project always has Jira's own avatars to choose from");

    tracker.cleanup().await;
}

/// The endpoint decodes what it is given, so bytes that are not an image are refused rather than stored.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_bytes_that_are_not_an_image() {
    let error = cloud()
        .project_avatars()
        .create_project_avatar(TEST_PROJECT_KEY, vec![1u8, 2, 3, 4])
        .content_type("image/png")
        .size(48)
        .send()
        .await
        .expect_err("four arbitrary bytes are not an image Jira can crop");

    assert_eq!(error.status(), Some(400), "{error}");
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
