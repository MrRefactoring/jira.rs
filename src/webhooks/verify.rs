use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::core::{Error, Result};

const ALGORITHM: &str = "sha256";

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

fn from_hex(hex: &str) -> Option<Vec<u8>> {
    if hex.is_empty() || !hex.len().is_multiple_of(2) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }

    hex.as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).ok()?, 16).ok()).collect()
}
