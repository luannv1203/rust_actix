use std::{env};
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{Collection, Client};

use crate::models::{user_model::User, admin_model::Admin};

pub struct MongoRepo {
  pub user: Collection<User>,
  pub admin: Collection<Admin>
}

impl MongoRepo {
  pub async fn init() -> Self {
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
      Ok(v) => v.to_string(),
      Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri).await.unwrap();
    println!("Mongodb Connected!");
    let db = client.database("luannv");
    let user = db.collection("users");
    let admin = db.collection("admins");

    MongoRepo {
      user: user,
      admin: admin
    }
  }
}
