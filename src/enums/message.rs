#[derive(Debug)]
pub enum Message {
  MSG_CREATE_USER_SUCCESS
}

impl Message {
  pub fn new(mess: Message) -> String {
    match mess {
      Message::MSG_CREATE_USER_SUCCESS => String::from("Create User Successfully!")
    }
  }
}