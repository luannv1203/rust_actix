use mongodb::{Collection, bson::{uuid::Error, doc}};

use crate::models::admin_model::Admin;

pub async fn get_user_by_email(col: &Collection<Admin>, email: &String) -> Result<Option<Admin>, Error> {
  let filter = doc! {"email": email};
  let admin = col.find_one(filter, None).await.ok().expect("Not found");
  Ok(admin)
}