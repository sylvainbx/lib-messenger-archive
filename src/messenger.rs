pub mod common;
pub mod error;
pub mod messenger_plus_parser;
pub mod xml_parser;

pub trait MessengerArchive: Iterator {
    fn details(&self) -> &ArchiveDetails;
}

#[derive(Default, PartialEq, Debug)]
pub struct ArchiveDetails {
    pub file_type: FileType,
    pub first_session_id: String,
    pub last_session_id: String,
    pub recipient_id: String,
}

#[derive(Default, PartialEq, Debug)]
pub struct Message {
    pub datetime: String,
    pub timezone_offset: Option<i64>,
    pub session_id: String,
    pub sender_friendly_name: String,
    pub receiver_friendly_name: String,
    pub data: Vec<Data>,
}

#[derive(PartialEq, Debug)]
pub enum Data {
    Text(Text),
    Image(Image),
    System(String),
}

#[derive(Default, PartialEq, Debug)]
pub struct Text {
    pub style: String,
    pub content: String,
}

#[derive(Default, PartialEq, Debug)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub content: Vec<u8>,
}

#[derive(Default, PartialEq, Debug)]
pub enum FileType {
    #[default]
    XML,
    MessengerPlus,
}
