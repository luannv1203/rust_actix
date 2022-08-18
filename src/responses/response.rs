use serde::{Serialize, Deserialize};

use super::pagination::Pagination;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  status: u16,
  data: T,
  message: String,
  code: u16,
}

impl<T> Response<T> {
  pub fn new(status: u16, data: T, message: String, code: u16) -> Self {
    Response {
      status: status,
      data: data,
      message: message,
      code: code,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWithPagination<T> {
  status: u16,
  data: T,
  message: String,
  code: u16,
  pagination: Option<Pagination>
}

impl<T> ResponseWithPagination<T> {
  pub fn new(status: u16, data: T, message: String, code: u16, pagination: Option<Pagination>) -> Self {
    ResponseWithPagination {
      status: status,
      data: data,
      message: message,
      code: code,
      pagination: pagination
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoResponseWithPagination<T> {
  pub data: Vec<T>,
  pub pagination: Pagination
}
