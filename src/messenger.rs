pub mod xml_parser;
pub mod html_parser;
pub mod error;

#[derive(Default, PartialEq, Debug)]
pub struct MessagesList {
    file_type: FileType,
    first_session_id : usize,
    last_session_id: usize,
    messages: Vec<Message>,
    recipient_id: String,
}

#[derive(Default, PartialEq, Debug)]
pub struct Message {
    local_date: String,
    local_time: String,
    utc_datetime: String,
    session_id: usize,
    sender_friendly_name: String,
    receiver_friendly_name: String,
    texts: Vec<Text>,
}
#[derive(Default, PartialEq, Debug)]
pub struct Text {
    style: String,
    content: String
}

#[derive(Default, PartialEq, Debug)]
pub enum FileType {
    #[default]
    HTML,
    XML,
}
