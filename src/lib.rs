mod messenger;

use std::fs::File;
use std::error::Error;
use messenger::error::MsnMessengerError;
use crate::messenger::MessagesList;

/// Read the file at the given path and try to interpret it as a valid MSN Messenger 
/// conversation archive.
/// 
/// The resulting structure is a MessagesList
pub fn parse<'a>(path: String) -> Result<MessagesList, Box<dyn Error>> {
    let file = File::open(&path)?;
    if path.ends_with(".xml") {
        messenger::xml_parser::parse(file, &path)
    } else if path.ends_with(".html") {
        messenger::html_parser::parse(file, &path)
    } else {
        Err(Box::new(MsnMessengerError::new("Invalid messenger archive. Expected format: .html or .xml")))
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
