use std::collections::HashMap;
use crate::messenger::{FileType, Message, MessagesList, Text};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse(file: File, path: &str) -> Result<MessagesList, Box<dyn Error>> {
  let file = BufReader::new(file);
  let parser = EventReader::new(file);
  let mut parents: Vec<String> = Vec::new();
  let mut list: MessagesList = MessagesList {
    file_type: FileType::XML,
    recipient_id: Path::new(path).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_string(),
    ..MessagesList::default()
  };

  for e in parser {
    match e {
      Ok(XmlEvent::StartElement { name, attributes, .. }) => {
        parse_node(&name.local_name, &parents, &attributes, &mut list)?;
        parents.push(name.local_name.clone());
      }
      Ok(XmlEvent::Characters(data)) => {
        if parents.ends_with(&["Message".to_string(), "Text".to_string()]) {
          let msg = list.messages.last_mut().unwrap();
          let text = msg.texts.last_mut().unwrap();

          text.content = data;
        }
      }
      Ok(XmlEvent::EndElement { .. }) => {
        parents.pop();
      }
      Err(e) => {
        return Err(Box::new(e))
      }
      _ => {}
    }
  }
  Ok(list)
}

fn parse_node(name: &str, parents: &Vec<String>, attributes: &Vec<OwnedAttribute>, list: &mut MessagesList) -> Result<(), Box<dyn Error>>{
  let attributes = parse_attributes(attributes);

  match name {
    "Log" => {
      list.first_session_id = attributes.get("FirstSessionID").unwrap_or(&"0".to_string()).parse::<usize>()?;
      list.last_session_id = attributes.get("LastSessionID").unwrap_or(&"0".to_string()).parse::<usize>()?;
    }
    "Message"=> {
      let mut msg = Message::default();
      msg.local_date = attributes.get("Date").unwrap_or(&"".to_string()).to_string();
      msg.local_time = attributes.get("Time").unwrap_or(&"".to_string()).to_string();
      msg.utc_datetime = attributes.get("DateTime").unwrap_or(&"".to_string()).to_string();
      msg.session_id = attributes.get("SessionID").unwrap_or(&"0".to_string()).parse::<usize>()?;

      list.messages.push(msg);
    }
    "User" => {
      let msg = list.messages.last_mut().unwrap();
      if parents.contains(&"From".to_string()) {
        msg.sender_friendly_name = attributes.get("FriendlyName").unwrap_or(&"".to_string()).to_string();
      } else if parents.contains(&"To".to_string()) {
        msg.receiver_friendly_name = attributes.get("FriendlyName").unwrap_or(&"".to_string()).to_string();
      }
    }
    "Text" => {
      let msg = list.messages.last_mut().unwrap();

      let text = Text {
        style: attributes.get("Style").unwrap_or(&"".to_string()).to_string(),
        ..Text::default()
      };

      msg.texts.push(text);

    }
    _ => { }
  }
  Ok(())
}

fn parse_attributes(attributes: &Vec<OwnedAttribute>) -> HashMap<String, String> {
  let mut hash: HashMap<String, String> = HashMap::new();
  for attribute in attributes {
    hash.insert(attribute.name.local_name.clone(), attribute.value.clone());
  }
  hash
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_sample_file() {
    let path = "test/user1234.xml";
    let file = File::open(path).unwrap();
    let result = parse(file, path);
    let expected = MessagesList {
      file_type: FileType::XML,
      first_session_id: 1,
      last_session_id: 1,
      recipient_id: "user1234".to_string(),
      messages: vec![Message {
        local_date: "06/04/2009".to_string(),
        local_time: "21:40:41".to_string(),
        utc_datetime: "2009-04-06T19:40:41.851Z".to_string(),
        session_id: 1,
        sender_friendly_name: "Alice".to_string(),
        receiver_friendly_name: "Bob".to_string(),
        texts: vec![Text {
          style: "font-family:Courier New; color:#004000; ".to_string(),
          content: "Hello!".to_string()
        }]
      },
      Message {
        local_date: "06/04/2009".to_string(),
        local_time: "22:22:05".to_string(),
        utc_datetime: "2009-04-06T20:22:05.918Z".to_string(),
        session_id: 1,
        sender_friendly_name: "Bob".to_string(),
        receiver_friendly_name: "Alice".to_string(),
        texts: vec![Text {
          style: "font-family:Courier New; color:#004000; ".to_string(),
          content: "Hi ".to_string()
        },
        Text {
          style: "font-family:Arial; color:#004020; ".to_string(),
          content: "Alice!".to_string()
        }]
      }]
    };
    assert_eq!(result.unwrap(), expected);
  }
  
  #[test]
  fn parse_scrappy_file() {
    let path = "test/scrappy.xml";
    let file = File::open(path).unwrap();
    let result = parse(file, path);
    let expected = MessagesList {
      file_type: FileType::XML,
      first_session_id: 0,
      last_session_id: 0,
      recipient_id: "scrappy".to_string(),
      messages: vec![]
    };
    assert_eq!(result.unwrap(), expected);
  }
}
