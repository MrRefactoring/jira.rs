//! Whether a delivery really came from your Jira site.

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::core::{Error, Result};

/// How a signature was computed. Jira sends `sha256`; nothing else is accepted.
const ALGORITHM: &str = "sha256";

/// Whether `body` carries a signature that `secret` produces.
///
/// A webhook registered with a secret arrives signed: Jira computes HMAC-SHA256 over the exact bytes of the request
/// body and sends the digest as `X-Hub-Signature: sha256=<hex>`. Recomputing it is the only thing that distinguishes
/// a delivery from Jira from a POST anyone on the internet can make to the same URL.
///
/// `body` must be the bytes that arrived. Re-serializing a parsed value produces a different byte sequence for the
/// same data — key order, whitespace and number formatting are not preserved — and the signature will not match.
/// Every framework has a way to keep the raw body; use it.
///
/// Answers `Ok(false)` for every way a delivery can fail to be trustworthy: no header, an algorithm other than
/// `sha256`, a digest that is not hexadecimal, a digest of the right shape and the wrong value. A handler's response
/// to all four is the same, and telling them apart to the caller would tell them apart to whoever is probing the
/// endpoint.
///
/// Errors only on a mistake of yours. An empty secret would verify every body ever sent, so it is a programming
/// error rather than a failed check.
///
/// The comparison is [`hmac`]'s own, which is constant-time: one that stopped at the first differing byte would let
/// an attacker who can send many deliveries and measure the replies recover a valid signature a byte at a time.
///
/// ```
/// use jira::webhooks::verify_signature;
///
/// // Test case 2 of RFC 4231.
/// let signed = "sha256=5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";
///
/// assert!(verify_signature(b"what do ya want for nothing?", "Jefe", Some(signed))?);
/// assert!(!verify_signature(b"a different body", "Jefe", Some(signed))?);
/// assert!(!verify_signature(b"what do ya want for nothing?", "Jefe", None)?);
/// # Ok::<(), jira::Error>(())
/// ```
pub fn verify_signature(body: &[u8], secret: &str, signature: Option<&str>) -> Result<bool> {
    if secret.is_empty() {
        return Err(Error::Config("the webhook secret is empty, which would verify every body ever sent".to_owned()));
    }

    let Some(signature) = signature else { return Ok(false) };
    let Some((algorithm, digest)) = signature.split_once('=') else { return Ok(false) };

    if algorithm != ALGORITHM {
        return Ok(false);
    }

    let Some(sent) = from_hex(digest) else { return Ok(false) };

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| Error::Config("the webhook secret cannot be used as an HMAC key".to_owned()))?;

    mac.update(body);

    Ok(mac.verify_slice(&sent).is_ok())
}

/// The bytes a hexadecimal digest spells, or `None` if it does not spell one.
///
/// The digits are checked before they are read. [`u8::from_str_radix`] accepts a leading sign, so `+a` would parse
/// as ten and `sha256=+abc` would be taken for a digest it is not.
fn from_hex(hex: &str) -> Option<Vec<u8>> {
    if hex.is_empty() || !hex.len().is_multiple_of(2) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }

    hex.as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok()).collect()
}
