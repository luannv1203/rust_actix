#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Message {
  MSG_CREATE_USER_SUCCESS,
  MSG_UPDATE_USER_SUCCESS
}

impl Message {
  pub fn new(self) -> String {
    match self {
      Message::MSG_CREATE_USER_SUCCESS => String::from("Create User Successfully!"),
      Message::MSG_UPDATE_USER_SUCCESS => String::from("Update User Successfully!"),
    }
  }
}