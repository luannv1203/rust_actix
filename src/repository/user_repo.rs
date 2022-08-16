use mongodb::{bson::{oid::ObjectId, doc, Bson}, Collection, results::{UpdateResult, DeleteResult}};
use std::{fmt::Error};
use futures::stream::TryStreamExt;

use crate::{models::user_model::User, responses::user_response::UserResponse};


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

pub async fn get_list_user_repo(db: &Collection<User>) -> Result<Vec<UserResponse>, Error> {
  let mut cursors = db.find(None, None).await.ok().expect("Get Failed!");
  let mut users: Vec<UserResponse> = Vec::new();
  while let Some(user) = cursors
    .try_next()
    .await
    .ok()
    .expect("Get List User Failed!")
  {
    users.push(UserResponse::new(user))
  }
  Ok(users)
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
