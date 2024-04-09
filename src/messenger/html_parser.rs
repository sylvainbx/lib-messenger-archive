use std::error::Error;
use std::fs::File;
use std::path::Path;
use crate::messenger::MessagesList;

pub fn parse(file: File, path: &str) -> Result<MessagesList, Box<dyn Error>> {
    Ok(MessagesList {
        recipient_id: Path::new(path).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_string(),
        ..MessagesList::default()
    })
}