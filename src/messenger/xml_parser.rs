use std::collections::HashMap;
use crate::messenger::{Data, FileType, Message, MessagesList, Text};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use chrono::NaiveTime;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse(path: &str) -> Result<MessagesList, Box<dyn Error>> {
  let file = File::open(&path)?;
  let file = BufReader::new(file);
  let parser = EventReader::new(file);
  let mut parents: Vec<String> = Vec::new();
  let mut list: MessagesList = MessagesList {
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
          let msg_data = msg.data.last_mut().unwrap();
          if let Data::Text(text) = msg_data {
            text.content = data;
          }
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
      list.first_session_id = attributes.get("FirstSessionID").unwrap_or(&"0".to_string()).to_string();
      list.last_session_id = attributes.get("LastSessionID").unwrap_or(&"0".to_string()).to_string();
    }
    "Message"=> {
      let mut msg = Message::default();
      msg.session_id = attributes.get("SessionID").unwrap_or(&"0".to_string()).to_string();
      handle_message_datetime(&mut msg, &attributes);

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

      msg.data.push(Data::Text(text));
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

fn handle_message_datetime(message: &mut Message, attributes: &HashMap<String, String>) {
  message.datetime = attributes.get("DateTime").unwrap_or(&"".to_string()).to_string();

  let utc_time = NaiveTime::parse_and_remainder(&message.datetime, "%Y-%m-%dT%H:%M:%S");
  if let Ok(utc_time) = utc_time {
    let local_time = NaiveTime::from_str(attributes.get("Time").unwrap_or(&"".to_string()));
    if let Ok(local_time) = local_time {
      message.timezone_offset = Some((local_time - utc_time.0).num_minutes());
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_sample_file() {
    let path = "test/alice1234.xml";
    let result = parse(path);
    let expected = MessagesList {
      file_type: FileType::XML,
      first_session_id: "1".to_string(),
      last_session_id: "1".to_string(),
      recipient_id: "alice1234".to_string(),
      messages: vec![Message {
        datetime: "2009-04-06T19:40:41.851Z".to_string(),
        timezone_offset: Some(120),
        session_id: "1".to_string(),
        sender_friendly_name: "Alice".to_string(),
        receiver_friendly_name: "Bob".to_string(),
        data: vec![Data::Text(Text {
          style: "font-family:Courier New; color:#004000; ".to_string(),
          content: "Hello!".to_string()
        })]
      },
      Message {
        datetime: "2009-04-06T20:22:05.918Z".to_string(),
        timezone_offset: Some(120),
        session_id: "1".to_string(),
        sender_friendly_name: "Bob".to_string(),
        receiver_friendly_name: "Alice".to_string(),
        data: vec![Data::Text(Text {
          style: "font-family:Courier New; color:#004000; ".to_string(),
          content: "Hi ".to_string()
        }),
        Data::Text(Text {
          style: "font-family:Arial; color:#004020; ".to_string(),
          content: "Alice!".to_string()
        })]
      }]
    };
    assert_eq!(result.unwrap(), expected);
  }
  
  #[test]
  fn parse_scrappy_file() {
    let path = "test/scrappy.xml";
    let result = parse(path);
    let expected = MessagesList {
      file_type: FileType::XML,
      first_session_id: "0".to_string(),
      last_session_id: "0".to_string(),
      recipient_id: "scrappy".to_string(),
      messages: vec![]
    };
    assert_eq!(result.unwrap(), expected);
  }
}
