pub enum Status {
  OK,
  BadRequest,
  InternalServerError,
  // Unauthorization,
}

pub fn status_number(status: Status) -> u16 {
  match status {
    Status::OK => 200,
    Status::BadRequest => 400,
    Status::InternalServerError => 500,
    // Status::Unauthorization => 401
  }
}