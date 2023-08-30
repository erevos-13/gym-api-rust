use core::fmt;

use chrono::Utc;
use hmac::{Hmac, Mac};
use jsonwebtoken::errors::ErrorKind;

use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
const TOP_SECRET: &[u8] = b"68a43845-7980-4301-abe5-1df0cee9ef92";

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}
impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    gym: String,
    exp: usize,
    role: u8,
}

pub fn signing(uid: String) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.clone(),
        role: Role::User as u8,
        exp: expiration as usize,
        gym: String::from("test gym"),
    };
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TOP_SECRET),
    ) {
        Ok(token) => token,
        Err(_) => "error".to_string(),
    };
    Ok(token)
}
