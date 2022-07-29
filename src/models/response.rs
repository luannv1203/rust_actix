use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  status: u16,
  data: T,
  message: String,
  code: u16
}

impl<T> Response<T> {
  pub fn new(status: u16, data: T, message: String, code: u16) -> Self {
    Response {
      status: status,
      data: data,
      message: message,
      code: code
    }
  }
}