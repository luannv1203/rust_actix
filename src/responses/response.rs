use serde::{Serialize, Deserialize};

use crate::enums::status::Status;

use super::pagination::Pagination;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  status: Status,
  data: T,
  message: String,
  code: Status,
}

impl<T> Response<T> {
  pub fn new(status: Status, data: T, message: String, code: Status) -> Self {
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
  status: Status,
  data: T,
  message: String,
  code: Status,
  pagination: Option<Pagination>
}

impl<T> ResponseWithPagination<T> {
  pub fn new(status: Status, data: T, message: String, code: Status, pagination: Option<Pagination>) -> Self {
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
