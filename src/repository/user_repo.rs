use actix_web::web;
use mongodb::{bson::{oid::ObjectId, doc, Bson}, Collection, results::{UpdateResult, DeleteResult}, options::FindOptions};
use std::{fmt::Error};
use futures::stream::TryStreamExt;

use crate::{models::user_model::User, responses::{user_response::{UserResponse}, pagination::Pagination, response::RepoResponseWithPagination}, apis::user_apis::QueryParams};


pub async fn create_user_repo(db: &Collection<User>, new_user: User) -> Result<Bson, Error> {
  let new_doc = User{
    id: None,
    name: new_user.name,
    location: new_user.location,
    title: new_user.title
  };
  let user = db
    .insert_one(new_doc, None)
    .await
    .ok()
    .expect("Error creating user");
    Ok(user.inserted_id)
}

pub async fn get_user_repo(db: &Collection<User>, id: &String) -> Result<User, Error> {
  let obj_id = ObjectId::parse_str(id).unwrap();
  let filter = doc! {"_id": obj_id};
  let user_detail = db
    .find_one(filter, None)
    .await
    .ok()
    .expect("Error getting user's detail");
  Ok(user_detail.unwrap())
}

fn string_query_like(param: &Option<String>) -> String {
  let mut str = String::new();
  str.push_str(".*");
  str.push_str(param.as_ref().unwrap_or(&String::new()));
  str.push_str(".*");
  str
}

pub async fn get_list_user_repo(db: &Collection<User>, query: web::Query<QueryParams>) -> Result<RepoResponseWithPagination<UserResponse>, Error> {
  let name = string_query_like(&query.name);
  let title = string_query_like(&query.title);
  let filter = doc! {"name": {"$regex": name}, "title": {"$regex": title} };
  let v = filter.clone();

  let limit = query.size.unwrap_or(10);
  let page = query.page.unwrap_or(1);

  let find_options = FindOptions::builder()
    .limit(limit)
    .skip(u64::try_from((page - 1) * limit).unwrap())
    .build();
  let mut cursors = db
    .find(filter, find_options)
    .await.ok()
    .expect("Get Failed!");
  let mut users: Vec<UserResponse> = Vec::new();
  while let Some(user) = cursors
    .try_next()
    .await
    .ok()
    .expect("Get List User Failed!")
  {
    users.push(UserResponse::new(user))
  }

  let total_record = db.count_documents(v, None).await.unwrap();
  let total;
  if (total_record as i64) % limit > 0 {
    total = (((total_record as i64) / limit) as i64) + 1
  } else {
    total = ((total_record as i64) / limit) as i64
  }

  let pagination = Pagination {
    page: page,
    size: limit,
    total_records: total_record as i64,
    total_pages: total,
  };
  Ok(RepoResponseWithPagination {
    data: users,
    pagination: pagination
  })
}

pub async fn update_user_repo(db: &Collection<User>, id: &String, new_user: User) -> Result<UpdateResult, Error> {
  let obj_id = ObjectId::parse_str(id).unwrap();
  let filter = doc! {"_id": obj_id};
  let new_doc = doc! {
    "$set":
      {
        "id": new_user.id,
        "name": new_user.name,
        "location": new_user.location,
        "title": new_user.title
      },
  };
  let updated_doc = db
    .update_one(filter, new_doc, None)
    .await
    .ok()
    .expect("Error updating user");
  Ok(updated_doc)
}

pub async fn delete_user_repo(db: &Collection<User>, id: &String) -> Result<DeleteResult, Error> {
  let obj_id = ObjectId::parse_str(id).unwrap();
  let filter = doc! {"_id": obj_id};
  let user = db
    .delete_one(filter, None)
    .await
    .ok()
    .expect("Error Deleted Fail!");
  Ok(user)
}
