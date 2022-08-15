pub enum Status {
  OK,
  BadRequest,
  InternalServerError,
  // Unauthorization,
}
impl Status {
  pub fn new(status: Status) -> u16 {
    match status {
      Status::OK => 200,
      Status::BadRequest => 400,
      Status::InternalServerError => 500,
      // Status::Unauthorization => 401
    }
  }
}