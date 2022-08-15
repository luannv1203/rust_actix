use serde::{Serialize, Deserialize};

use crate::models::user_model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
  pub id: String,
  pub name: String,
  pub location: String,
  pub title: String
}

impl UserResponse {
  pub fn new(user: User) -> Self {
    return UserResponse {
      id: user.id.unwrap().to_hex(),
      name: user.name,
      location: user.location,
      title: user.title
    }
  }
}