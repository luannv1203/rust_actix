use actix_web::{post, web::{Data, Json, self}, HttpResponse, get, put, delete};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

use crate::{
  repository::{
    mongodb_repo::MongoRepo,
    user_repo::{create_user_repo, get_user_repo, get_list_user_repo, update_user_repo, delete_user_repo}
  },
  models::{
    user_model::User,
  },
  enums::{status::{Status}, message::Message}, responses::{user_response::UserResponse, response::{Response, ResponseWithPagination}},
};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
  pub id: Option<String>,
  pub name: Option<String>,
  pub size: Option<i64>,
  pub page: Option<i64>
}

#[post("/")]
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
        Status::new(Status::OK),
        user,
        Message::new(Message::MSG_CREATE_USER_SUCCESS),
        200,
      )
    ),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string())
  }
}

#[get("/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: web::Path<String>) -> HttpResponse {
  let id = path.into_inner();
  if id.is_empty() {
    return HttpResponse::Ok().json(
      Response::new(
        Status::new(Status::OK),
        None::<User>,
        String::new(),
        Status::new(Status::BAD_REQUEST),
      )
    )
  }
  let user_detail = get_user_repo(&&db.user, &id).await;
  match user_detail {
    Ok(user) => HttpResponse::Ok().json(
      Response::new(
        Status::new(Status::OK),
        UserResponse::new(user),
        String::new(),
        200,
      )
    ),
    Err(err) => HttpResponse::InternalServerError().json(
      Response::new(
        Status::new(Status::INTERNAL_SERVER_ERROR),
        None::<User>,
        err.to_string(),
        200,
      )
    ),
  }
}

#[get("/list")]
pub async fn get_list_user(
  // _: AuthorizationService,
  db: Data<MongoRepo>,
  query: web::Query<QueryParams>,
) -> HttpResponse {
  let users_repo = get_list_user_repo(&db.user, query).await;
  match users_repo {
    Ok(users_res) => HttpResponse::Ok().json(
      ResponseWithPagination::new(
        Status::new(Status::OK),
        users_res.data,
        String::new(),
        Status::new(Status::OK),
        Some(users_res.pagination)
      )
    ),
    Err(err) => HttpResponse::InternalServerError().json(
      Response::new(
        Status::new(Status::INTERNAL_SERVER_ERROR),
        None::<Vec<UserResponse>>,
        err.to_string(),
        200,
      )
    )
  }
}

#[put("/{id}")]
pub async fn update_user(db: Data<MongoRepo>, path: web::Path<String>, new_user: Json<User>) -> HttpResponse {
  let id = path.into_inner();
  if id.is_empty() {
    return HttpResponse::Ok().json(
      Response::new(
        Status::new(Status::OK),
        None::<UserResponse>,
        String::new(),
        Status::new(Status::BAD_REQUEST),
      )
    )
  };

  let data = User {
    id: Some(ObjectId::parse_str(&id).unwrap()),
    name: new_user.name.to_owned(),
    location: new_user.location.to_owned(),
    title: new_user.title.to_owned(),
  };

  let update_result = update_user_repo(&db.user, &id, data).await;
  match update_result {
    Ok(update) => {
      if update.matched_count == 1 {
        let updated_user_info = get_user_repo(&&db.user, &id).await;
        return match updated_user_info {
            Ok(user) => HttpResponse::Ok().json(
              Response::new(
                Status::new(Status::OK),
                UserResponse::new(user),
                Message::new(Message::MSG_UPDATE_USER_SUCCESS),
                200,
              )
            ),
            Err(err) => HttpResponse::InternalServerError().json(
              Response::new(
                Status::new(Status::INTERNAL_SERVER_ERROR),
                None::<User>,
                err.to_string(),
                200,
              )
            ),
        };
      } else {
        return HttpResponse::NotFound().body("No user found with specified ID");
      }
    }
    Err(err) => HttpResponse::InternalServerError().json(
      Response::new(
        Status::new(Status::INTERNAL_SERVER_ERROR),
        None::<User>,
        err.to_string(),
        200,
      )
    ),
  }
}

#[delete("/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: web::Path<String>) -> HttpResponse {
  let id = path.into_inner();
  if id.is_empty() {
    return HttpResponse::Ok().json(
      Response::new(
        Status::new(Status::OK),
        None::<User>,
        String::new(),
        Status::new(Status::BAD_REQUEST),
      )
    )
  }
  let result = delete_user_repo(&db.user, &id).await;
  match result {
    Ok(res) => {
      if res.deleted_count == 1 {
        return HttpResponse::Ok().json(
          Response::new(
            Status::new(Status::OK),
            None::<User>,
            String::from("User successfully deleted!"),
            Status::new(Status::OK),
          )
        )
      } else {
        return HttpResponse::Ok().json(
          Response::new(
            Status::new(Status::INTERNAL_SERVER_ERROR),
            None::<User>,
            String::from("User with specified ID not found!"),
            Status::new(Status::INTERNAL_SERVER_ERROR),
          )
        )
      }
    }
    Err(err) => HttpResponse::InternalServerError().json(
      Response::new(
        Status::new(Status::INTERNAL_SERVER_ERROR),
        None::<User>,
        err.to_string(),
        Status::new(Status::INTERNAL_SERVER_ERROR),
      )
    )
  }
}

pub fn init_routes_user(cfg: &mut web::ServiceConfig) {
  cfg.service(create_user);
  cfg.service(get_list_user);
  cfg.service(get_user);
  cfg.service(update_user);
  cfg.service(delete_user);
}
