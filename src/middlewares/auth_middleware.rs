use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::apis::auth_apis::Claims;
use crate::enums::message::Message;

pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
  type Error = Error;
  type Future = Ready<Result<AuthorizationService, Error>>;

  fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
    let _auth = _req.headers().get("Authorization");
    match _auth {
      Some(_) => {
        let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
        let token = _split[1].trim();
        match decode::<Claims>(
          token,
          &DecodingKey::from_secret("abcdefgh".as_bytes()),
          &Validation::new(Algorithm::HS256),
        ) {
          Ok(_token) => ok(AuthorizationService),
          Err(_e) => err(ErrorUnauthorized(Message::new(Message::MSG_TOKEN_INVALID))),
        }
      }
      None => err(ErrorUnauthorized::<_>(Message::new(Message::MSG_TOKEN_INVALID))),
    }
  }
}
