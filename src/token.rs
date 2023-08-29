use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

const TOP_SECRET: &[u8] = b"68a43845-7980-4301-abe5-1df0cee9ef92";

pub fn signing(sub: String, value: String) -> Result<String, Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(TOP_SECRET).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("username", value);
    match claims.sign_with_key(&key) {
        Ok(token) => Ok(token),
        Err(e) => Err(e),
    }
}
