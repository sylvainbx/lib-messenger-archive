mod messenger;

use std::error::Error;
use messenger::error::MessengerError;
use crate::messenger::MessagesList;

/// Read the file at the given path and try to interpret it as a valid MSN Messenger / WLM
/// conversation archive.
///
/// Archives generated by the Messenger Plus! plugin are supported too.
/// 
/// The resulting structure is a MessagesList
pub fn parse<'a>(path: String) -> Result<MessagesList, Box<dyn Error>> {
    if path.ends_with(".xml") {
        messenger::xml_parser::parse(&path)
    } else if path.ends_with(".html") {
        messenger::messenger_plus_parser::parse(&path)
    } else {
        Err(Box::new(MessengerError::new("Invalid messenger archive. Expected format: .html or .xml")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_parse_crazy_file() {
        let result = parse("archive.zip".to_string());
        assert!(result.is_err());
    }
}
