use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
  pub page: i64,
  pub size: i64,
  pub total_records: i64,
  pub total_pages: i64
}
