pub mod common;
pub mod messenger_plus_parser;
pub mod xml_parser;

pub trait MessengerArchive: Iterator {
    fn details(&self) -> Option<&ArchiveDetails>;
}

/// Some general details about the current archive.
#[derive(Default, PartialEq, Debug)]
pub struct ArchiveDetails {
    /// Standard archive or Messenger Plus!
    pub file_type: FileType,
    /// An archive may be composed of multiple conversation sessions, this is the ID of the first
    /// session.
    pub first_session_id: String,
    /// The ID of the last session (see `first_session_id`).
    pub last_session_id: String,
    /// The ID of the user exchanging with the one owning the archive.
    pub recipient_id: String,
}

/// A message, exchanged by two messenger's users.
#[derive(Default, PartialEq, Debug)]
pub struct Message {
    /// The date and time of the message. May be more or less precise, depending on what was saved
    /// in the archive: the minutes should always be present but the seconds may not be.
    pub datetime: String,
    /// The minutes difference between UTC and the local timezone, may be negative.
    /// The `Option` may be `None` if the timezone is unknown.
    pub timezone_offset: Option<i64>,
    /// An archive may be composed of multiple conversation sessions, this is the ID of the current
    /// session.
    pub session_id: String,
    /// The sending user may use a different nickname for each message, this is his nickname for the
    /// current message.
    pub sender_friendly_name: String,
    /// The receiving user may use a different nickname for each message, this is his nickname for
    /// the current message.
    pub receiver_friendly_name: String,
    /// The body of the message. May be composed of multiple parts (e.g. an image and a text).
    pub data: Vec<Data>,
}

/// The various type of parts of the message body.
#[derive(PartialEq, Debug)]
pub enum Data {
    /// Some formatted text.
    Text(Text),
    /// An image.
    Image(Image),
    /// Messenger Plus! plugin has also saved a message when the session's user was disconnected.
    /// These kind of messages are saved as `System`.
    System(String),
}

/// A part of a message body, containing some formatted text.
#[derive(Default, PartialEq, Debug)]
pub struct Text {
    /// A CSS-like string indicating the style of the text.
    pub style: String,
    /// The text.
    pub content: String,
}

/// A part of a message body, containing an image.
#[derive(Default, PartialEq, Debug)]
pub struct Image {
    /// The path to the image file.
    pub src: String,
    /// The alternative text if the image cannot be shown.
    pub alt: String,
    /// A buffer containing the image data.
    pub content: Vec<u8>,
}

/// Indicates the type of archive
#[derive(Default, PartialEq, Debug)]
pub enum FileType {
    #[default]
    /// A standard Windows Live Messenger XML archive.
    XML,
    /// A Messenger PLus! plugin HTML archive.
    MessengerPlus,
}
