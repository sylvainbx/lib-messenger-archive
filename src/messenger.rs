pub mod xml_parser;
pub mod messenger_plus_parser;
pub mod error;

#[derive(Default, PartialEq, Debug)]
pub struct MessagesList {
    file_type: FileType,
    first_session_id : String,
    last_session_id: String,
    messages: Vec<Message>,
    recipient_id: String,
}

#[derive(Default, PartialEq, Debug)]
pub struct Message {
    datetime: String,
    timezone_offset: Option<i64>,
    session_id: String,
    sender_friendly_name: String,
    receiver_friendly_name: String,
    data: Vec<Data>,
}


#[derive(PartialEq, Debug)]
pub enum Data {
    Text(Text),
    Image(Image)
}

#[derive(Default, PartialEq, Debug)]
pub struct Text {
    style: String,
    content: String
}

#[derive(Default, PartialEq, Debug)]
pub struct Image {
    src: String,
    alt: String,
    content: Vec<u8>,
}

#[derive(Default, PartialEq, Debug)]
pub enum FileType {
    #[default]
    XML,
    MessengerPlus,
}
