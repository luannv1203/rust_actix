use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, doc}, Collection};
use std::{fmt::Error};

use crate::models::user_model::User;


pub async fn create_user_repo(db: &Collection<User>, new_user: User) -> Result<InsertOneResult, Error> {
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
    Ok(user)
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
