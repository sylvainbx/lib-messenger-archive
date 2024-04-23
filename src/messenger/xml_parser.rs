use crate::messenger::{common, Data, Message, MessagesList, Text};
use chrono::NaiveTime;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent;

pub struct XmlParser {
    list: MessagesList,
    reader: EventReader<BufReader<File>>,
    parents: Vec<String>,
}

impl XmlParser {
    pub fn new(path: &str) -> Self {
        XmlParser {
            list: MessagesList {
                recipient_id: Path::new(path)
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string(),
                ..MessagesList::default()
            },
            reader: common::get_parser(path).expect("Invalid file provided"),
            parents: vec![],
        }
    }

    pub fn archive(&self) -> &MessagesList {
        &self.list
    }


    fn parse_node(&mut self, name: &str, message: &mut Message, attributes: &Vec<OwnedAttribute>) {
        let attributes = common::parse_attributes(attributes);

        match name {
            "Log" => {
                self.list.first_session_id = attributes.get("FirstSessionID").unwrap_or(&"0").to_string();
                self.list.last_session_id = attributes.get("LastSessionID").unwrap_or(&"0").to_string();
            }
            "Message" => {
                message.session_id = attributes.get("SessionID").unwrap_or(&"0").to_string();
                self.handle_message_datetime(message, &attributes);
            }
            "User" => {
                if self.parents.contains(&"From".to_string()) {
                    message.sender_friendly_name =
                        attributes.get("FriendlyName").unwrap_or(&"").to_string();
                } else if self.parents.contains(&"To".to_string()) {
                    message.receiver_friendly_name =
                        attributes.get("FriendlyName").unwrap_or(&"").to_string();
                }
            }
            "Text" => {
                let text = Text {
                    style: attributes.get("Style").unwrap_or(&"").to_string(),
                    ..Text::default()
                };

                message.data.push(Data::Text(text));
            }
            _ => {}
        }
    }

    fn handle_message_datetime(&mut self, message: &mut Message, attributes: &HashMap<&str, &str>) {
        message.datetime = attributes.get("DateTime").unwrap_or(&"").to_string();

        let utc_time = NaiveTime::parse_and_remainder(&message.datetime, "%Y-%m-%dT%H:%M:%S");
        if let Ok(utc_time) = utc_time {
            let local_time = NaiveTime::from_str(attributes.get("Time").unwrap_or(&""));
            if let Ok(local_time) = local_time {
                message.timezone_offset = Some((local_time - utc_time.0).num_minutes());
            }
        }
    }
}

impl Iterator for XmlParser {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        let mut message = Message::default();
        loop {
            let e = self.reader.next();
            match e {
                Ok(XmlEvent::StartElement {
                       name, attributes, ..
                   }) => {
                    self.parse_node(&name.local_name, &mut message, &attributes);
                    self.parents.push(name.local_name.clone());
                }
                Ok(XmlEvent::Characters(data)) => {
                    if self.parents.ends_with(&["Message".to_string(), "Text".to_string()]) {
                        let msg_data = message.data.last_mut()?;
                        if let Data::Text(text) = msg_data {
                            text.content = data;
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    self.parents.pop();
                    if name.local_name.eq("Message") {
                        return Some(message);
                    }
                }
                Ok(XmlEvent::EndDocument) => {
                    return None;
                }
                Err(e) => panic!("Something went wrong: {}", e),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messenger::FileType;

    #[test]
    fn parse_sample_file() {
        let path = "test/alice1234.xml";
        let mut parser = XmlParser::new(path);
        let expected = MessagesList {
            file_type: FileType::XML,
            first_session_id: "1".to_string(),
            last_session_id: "1".to_string(),
            recipient_id: "alice1234".to_string(),
        };
        let messages = vec![
            Message {
                datetime: "2009-04-06T19:40:41.851Z".to_string(),
                timezone_offset: Some(120),
                session_id: "1".to_string(),
                sender_friendly_name: "Alice".to_string(),
                receiver_friendly_name: "Bob".to_string(),
                data: vec![Data::Text(Text {
                    style: "font-family:Courier New; color:#004000; ".to_string(),
                    content: "Hello!".to_string(),
                })],
            },
            Message {
                datetime: "2009-04-06T20:22:05.918Z".to_string(),
                timezone_offset: Some(120),
                session_id: "1".to_string(),
                sender_friendly_name: "Bob".to_string(),
                receiver_friendly_name: "Alice".to_string(),
                data: vec![
                    Data::Text(Text {
                        style: "font-family:Courier New; color:#004000; ".to_string(),
                        content: "Hi ".to_string(),
                    }),
                    Data::Text(Text {
                        style: "font-family:Arial; color:#004020; ".to_string(),
                        content: "Alice!".to_string(),
                    }),
                ],
            },
        ];
        assert_eq!(parser.next().as_ref(), Some(&messages[0]));
        assert_eq!(parser.next().as_ref(), Some(&messages[1]));
        assert_eq!(parser.next(), None);
        assert_eq!(parser.archive(), &expected);
    }

    #[test]
    fn parse_scrappy_file() {
        let path = "test/scrappy.xml";
        let mut parser = XmlParser::new(path);
        let expected = MessagesList {
            file_type: FileType::XML,
            first_session_id: "0".to_string(),
            last_session_id: "0".to_string(),
            recipient_id: "scrappy".to_string(),
        };
        assert_eq!(parser.next(), None);
        assert_eq!(parser.archive(), &expected);
        
    }
}
