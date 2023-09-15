use std::{
    future::{ready, Ready},
};

use crate::{
    token::{self},
};
use actix_web::{dev::Payload};
use actix_web::{FromRequest, HttpMessage, HttpRequest};

#[derive(Debug)]
pub struct JwtMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(
        req: &HttpRequest,
        _: &mut Payload,
    ) -> std::future::Ready<Result<JwtMiddleware, actix_web::Error>> {
        let auth_header = req.headers().get("Authorization");
        let token = match auth_header {
            Some(t) => t.to_str().unwrap().to_string(),
            None => "".to_string(),
        };
        let token = token.replace("Bearer ", "");
        let token_data = token::decode_token(token);
        match token_data {
            Ok(token_data) => {
                let user_id = token_data.claims.sub.parse::<uuid::Uuid>().unwrap();
                req.extensions_mut()
                    .insert::<uuid::Uuid>(user_id.to_owned());

                return ready(Ok(JwtMiddleware { user_id }));
            }
            Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e))),
        }
    }
}
