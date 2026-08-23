use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Marker embedded in every resource name the live suites create.
///
/// Distinct from the marker `jira.js` uses, so the two suites can run against one site without sweeping up each
/// other's fixtures.
pub const RESOURCE_MARKER: &str = "jrs";

/// A stable id for this test process, so concurrent or repeated runs never collide.
pub fn run_id() -> &'static str {
    static RUN_ID: OnceLock<String> = OnceLock::new();

    RUN_ID.get_or_init(|| {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        // Two clocks rather than a random number: the process id separates runs started in the same millisecond, and
        // neither needs a dependency.
        format!("{:x}{:x}", now.as_millis(), std::process::id())
    })
}

/// A human-readable, run-scoped resource name, e.g. `[jrs:18f2a1c] my issue`.
pub fn test_name(label: &str) -> String {
    format!("[{}:{}] {label}", RESOURCE_MARKER, run_id())
}

/// A valid, unique Jira project key: uppercase letters and digits, starting with a letter, at most 10 characters.
///
/// Jira is stricter here than it looks — over ten characters, or a leading digit, and project creation fails with a
/// validation error rather than a truncated key.
///
/// The run id is hashed rather than truncated. Its leading characters are the high digits of a millisecond clock and
/// barely move within an hour, so a key built from the first few of them collides with the one the last run used —
/// and Jira answers "another project uses this project key", which reads as a leaked fixture rather than as a naming
/// bug. Hashing spreads the whole run id, and the label, across every character.
pub fn project_key(label: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;

    for byte in format!("{}{label}", run_id()).bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }

    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let suffix: String = (0..7)
        .map(|position| {
            let index = (hash >> (position * 8)) as usize % ALPHABET.len();

            char::from(ALPHABET[index])
        })
        .collect();

    format!("JRS{suffix}")
}
