use actix_web::{HttpResponse, post, web::{Data, Json, self}};
use chrono::{Utc, DateTime, Duration};
use crypto::{sha2::{Sha256}, digest::Digest};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use crate::{repository::{mongodb_repo::MongoRepo, admin_repo::get_user_by_email}, enums::status::Status, responses::{login_response::LoginResponse, response::Response}};
use std::{env};
extern crate dotenv;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
  email: String,
  password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize
}

#[post("/login")]
pub async fn login(db: Data<MongoRepo>, data: Json<LoginRequest>) -> HttpResponse {
  let admin = get_user_by_email(&db.admin, &data.email).await.unwrap();
  let secet_key = env::var("SECRET_KEY").unwrap();
  match admin {
    Some(x) => {
      let mut sha = Sha256::new();
      // let mut rsa = 
      sha.input_str(&data.password);
      if x.password.to_lowercase() == sha.result_str().to_lowercase() {
        let mut _date: DateTime<Utc>;
        _date = Utc::now() + Duration::hours(1);
        let my_claims = Claims {
          sub: x.email,
          exp: _date.timestamp() as usize,
        };
        let token = encode(
          &Header::default(),
          &my_claims,
          &EncodingKey::from_secret(secet_key.as_bytes()),
        ).unwrap();
        return HttpResponse::Ok().json(
          Response::new(
            Status::OK(200),
            LoginResponse::new(token),
            String::from("Login Success!"),
            Status::OK(200),
          )
        )
      } else {
        return HttpResponse::Ok().json(
          Response::new(
            Status::OK(200),
            None::<LoginResponse>,
            String::from("Check your user informations!"),
            Status::INTERNAL_SERVER_ERROR(500),
          )
        )
      }
    }
    None => 
      return HttpResponse::Ok().json(
        Response::new(
          Status::OK(200),
          None::<LoginResponse>,
          String::from("Check your user informations!"),
          Status::INTERNAL_SERVER_ERROR(500),
        )
      )
  }
}

pub fn init_routes_auth(cfg: &mut web::ServiceConfig) {
  cfg.service(login);
}
