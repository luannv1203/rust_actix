use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
  pub token: String,
}

impl LoginResponse {
  pub fn new(token: String) -> Self {
    return LoginResponse {
      token: token
    }
  }
}
