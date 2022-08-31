use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
  OK(u32),
  BAD_REQUEST(u32),
  INTERNAL_SERVER_ERROR(u32),
  UNAUTHORIZATION(u32),
}
