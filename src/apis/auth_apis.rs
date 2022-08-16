use actix_web::{HttpResponse, post, web::{Data, Json, self}};
use chrono::{Utc, DateTime, Duration};
use crypto::{sha2::Sha256, digest::Digest};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use crate::{repository::{mongodb_repo::MongoRepo, admin_repo::get_user_by_email}, enums::status::Status, models::response::Response, responses::{login_response::LoginResponse}};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
  email: String,
  password: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  exp: usize
}

#[post("/login")]
pub async fn login(db: Data<MongoRepo>, data: Json<LoginRequest>) -> HttpResponse {
  let admin = get_user_by_email(&db.admin, &data.email).await.unwrap();
  match admin {
    Some(x) => {
      let mut sha = Sha256::new();
      sha.input_str(&data.password);
      println!("{:?}", sha.result_str());
      if x.password.to_lowercase() == sha.result_str() {
        let mut _date: DateTime<Utc>;
        _date = Utc::now() + Duration::hours(1);
        let my_claims = Claims {
          sub: x.email,
          exp: _date.timestamp() as usize,
        };
        let token = encode(
          &Header::default(),
          &my_claims,
          &EncodingKey::from_secret("abcdefgh".as_bytes()),
        ).unwrap();
        return HttpResponse::Ok().json(
          Response::new(
            Status::new(Status::OK),
            LoginResponse::new(token),
            String::from("Login Success!"),
            Status::new(Status::OK)
          )
        )
      } else {
        return HttpResponse::Ok().json(
          Response::new(
            Status::new(Status::OK),
            None::<LoginResponse>,
            String::from("Check your user informations!"),
            Status::new(Status::INTERNAL_SERVER_ERROR),
          )
        )
      }
    }
    None => 
      return HttpResponse::Ok().json(
        Response::new(
          Status::new(Status::OK),
          None::<LoginResponse>,
          String::from("Check your user informations!"),
          Status::new(Status::INTERNAL_SERVER_ERROR),
        )
      )
  }
}

pub fn init_routes_auth(cfg: &mut web::ServiceConfig) {
  cfg.service(login);
}