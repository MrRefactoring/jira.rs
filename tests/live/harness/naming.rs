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
/// Built through [`run_suffix`], so two runs an hour apart do not land on the same key.
pub fn project_key(label: &str) -> String {
    format!("JRS{}", run_suffix(label, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", 7))
}

/// `length` characters drawn from `alphabet`, standing for this run and the label together.
///
/// The whole run id is hashed rather than trimmed. Its leading characters are the high digits of a millisecond clock
/// and barely move within an hour, so a key built from the first few of them repeats between runs — and whatever is
/// being named answers that the key is already taken, which reads as leaked state rather than as a naming bug.
pub fn run_suffix(label: &str, alphabet: &[u8], length: usize) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;

    for byte in format!("{}{label}", run_id()).bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }

    (0..length)
        .map(|position| {
            let index = (hash >> (position * 8)) as usize % alphabet.len();

            char::from(alphabet[index])
        })
        .collect()
}
