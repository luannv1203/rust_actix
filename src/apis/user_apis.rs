use actix_web::{post, web::{Data, Json, self}, HttpResponse, get};
use serde::{Serialize, Deserialize};

use crate::{
  repository::{
    mongodb_repo::MongoRepo,
    user_repo::{create_user_repo, get_user_repo}
  },
  models::{
    user_model::User,
    response::Response
  },
  enums::status::{status_number, Status}
};
#[derive(Debug, Serialize, Deserialize)]
struct UserResponse {
  id: String,
  name: String,
  location: String,
  title: String
}

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_doc: Json<User>) -> HttpResponse {
  let data = User {
    id: None,
    name: new_doc.name.to_owned(),
    location: new_doc.location.to_owned(),
    title: new_doc.title.to_owned()
  };
  let user_data = create_user_repo(&&db.user, data).await;
  match user_data {
    Ok(user) => HttpResponse::Ok().json(
      Response::new(
        status_number(Status::OK),
        user,
        String::from("Create user success!"),
        200
      )
    ),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
  }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: web::Path<String>) -> HttpResponse {
  let id = path.into_inner();
  if id.is_empty() {
    return HttpResponse::Ok().json(
      Response::new(
        status_number(Status::OK),
        None::<User>,
        String::new(),
        status_number(Status::BadRequest)
      )
    )
  }
  let user_detail = get_user_repo(&&db.user, &id).await;
  match user_detail {
    Ok(user) => HttpResponse::Ok().json(
      Response::new(
        status_number(Status::OK),
        UserResponse {
          id: user.id.unwrap().to_hex(),
          name: user.name,
          location: user.location,
          title: user.title,
        },
        String::new(),
        200
      )
    ),
    Err(err) => HttpResponse::InternalServerError().json(
      Response::new(
        status_number(Status::InternalServerError),
        None::<User>,
        err.to_string(),
        200
      )
    ),
  }
}
