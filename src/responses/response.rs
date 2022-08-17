use serde::{Serialize, Deserialize};

use super::pagination::Pagination;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  status: u16,
  data: T,
  message: String,
  code: u16,
  pagination: Option<Pagination>
}

impl<T> Response<T> {
  pub fn new(status: u16, data: T, message: String, code: u16, pagination: Option<Pagination>) -> Self {
    Response {
      status: status,
      data: data,
      message: message,
      code: code,
      pagination: pagination
    }
  }
}