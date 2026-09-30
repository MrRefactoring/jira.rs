use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub const RESOURCE_MARKER: &str = "jrs";

pub fn run_id() -> &'static str {
    static RUN_ID: OnceLock<String> = OnceLock::new();

    RUN_ID.get_or_init(|| {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        format!("{:x}{:x}", now.as_millis(), std::process::id())
    })
}

pub fn test_name(label: &str) -> String {
    format!("[{}:{}] {label}", RESOURCE_MARKER, run_id())
}

pub fn project_key(label: &str) -> String {
    format!("JRS{}", run_suffix(label, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", 7))
}

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
