#[allow(non_camel_case_types)]
pub enum Status {
  OK,
  BAD_REQUEST,
  INTERNAL_SERVER_ERROR,
  // Unauthorization,
}
impl Status {
  pub fn new(self) -> u16 {
    match self {
      Status::OK => 200,
      Status::BAD_REQUEST => 400,
      Status::INTERNAL_SERVER_ERROR => 500,
      // Status::Unauthorization => 401
    }
  }
}