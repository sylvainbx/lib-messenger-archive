use crate::messenger::common::parse_attributes;
use crate::messenger::{common, Data, FileType, Image, Message, MessagesList, Text};
use chrono::{NaiveDateTime, NaiveTime, Timelike};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

pub fn parse(path: &str) -> Result<MessagesList, Box<dyn Error>> {
    let parser = common::get_parser(path)?;
    let path = Path::new(path);
    let dir = path
        .parent()
        .expect("The file must be somewhere in a directory");

    let mut parents: (String, Vec<OwnedAttribute>) = ("".to_string(), vec![]);
    let mut session = MsgPlusSession::default();
    let mut list = MessagesList {
        recipient_id: path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string(),
        file_type: FileType::MessengerPlus,
        ..MessagesList::default()
    };

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                parse_node(
                    &name.local_name,
                    &parents,
                    &attributes,
                    &mut session,
                    &mut list,
                    dir,
                )?;
                parents.0 = format!("{}.{}", parents.0, name.local_name);
                parents.1 = attributes;
            }
            Ok(XmlEvent::Characters(data)) => {
                parse_text(&data, &parents, &mut list, &mut session)?;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                let new_selector = match parents.0.rfind('.') {
                    Some(pos) => &parents.0[0..pos],
                    None => "",
                };
                parents.0 = new_selector.to_string();
            }
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
    }
    list.last_session_id = session.id.clone();
    Ok(list)
}

#[derive(Default)]
struct MsgPlusSession {
    date: NaiveDateTime,
    id: String,
    owner: String,
    recipient: String,
    message_style: String,
}

fn parse_node(
    name: &str,
    parents: &(String, Vec<OwnedAttribute>),
    attributes: &Vec<OwnedAttribute>,
    session: &mut MsgPlusSession,
    list: &mut MessagesList,
    dir: &Path,
) -> Result<(), Box<dyn Error>> {
    let attributes = parse_attributes(attributes);
    match name {
        "div" => {
            if parents.0.ends_with("html.body")
                && attributes
                    .iter()
                    .any(|(attr, val)| attr.eq(&"class") && val.eq(&"mplsession"))
            {
                if let Some(id) = attributes.get("id") {
                    session.id = id.to_string();
                    session.date = NaiveDateTime::parse_from_str(id, "Session_%Y-%m-%dT%H-%M-%S")?;
                    if list.first_session_id.is_empty() {
                        list.first_session_id = id.to_string();
                    }
                }
            }
        }
        "td" => {
            if parents.0.ends_with("html.body.div.table.tbody.tr")
                && attributes.iter().any(|(attr, ..)| attr.eq(&"style"))
            {
                if let Some(style) = attributes.get("style") {
                    session.message_style =
                        html_escape::decode_html_entities(style).trim().to_string()
                }
            }
        }
        "tr" => {
            if parents.0.ends_with("html.body.div.table.tbody") {
                let mut msg = Message {
                    session_id: session.id.to_string(),
                    ..Message::default()
                };
                if attributes
                    .iter()
                    .any(|(attr, val)| attr.eq(&"class") && val.eq(&"msgplus"))
                {
                    msg.data = vec![Data::System("".to_string())];
                }
                list.messages.push(msg);
            }
        }
        "img" => {
            if parents.0.ends_with("html.body.div.table.tbody.tr.td")
                && attributes.iter().any(|(attr, ..)| attr.eq(&"src"))
            {
                let msg = list.messages.last_mut().unwrap();
                let mut img = Image::default();

                if let Some(alt) = attributes.get("alt") {
                    img.alt = alt.trim().to_string();
                }
                if let Some(src) = attributes.get("src") {
                    img.src = src.trim().to_string();
                    let mut buffer = Vec::new();
                    File::open(dir.join(src))?.read_to_end(&mut buffer)?;
                    img.content = buffer;
                }
                msg.data.push(Data::Image(img));
            }
        }
        _ => {}
    }
    Ok(())
}

fn parse_text(
    data: &str,
    parents: &(String, Vec<OwnedAttribute>),
    list: &mut MessagesList,
    session: &mut MsgPlusSession,
) -> Result<(), Box<dyn Error>> {
    match parents.0.as_str() {
        ".html.body.div.ul.li" => {
            let attributes = parse_attributes(&parents.1);
            if attributes
                .iter()
                .any(|(attr, val)| attr.eq(&"class") && val.eq(&"in"))
            {
                session.owner = data.trim().to_string();
            } else {
                session.recipient = data.trim().to_string();
            }
        }
        ".html.body.div.table.tbody.tr.th.span" => {
            if list.messages.len() == 1 {
                let datetime = NaiveDateTime::new(
                    session.date.date(),
                    NaiveTime::parse_from_str(
                        format!("{}:{}", data, session.date.second()).as_str(),
                        "(%H:%M):%S",
                    )?,
                );
                let msg = list.messages.last_mut().unwrap();
                msg.datetime = datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
            } else {
                let datetime = NaiveDateTime::new(
                    session.date.date(),
                    NaiveTime::parse_from_str(data, "(%H:%M)")?,
                );
                let msg = list.messages.last_mut().unwrap();
                msg.datetime = datetime.format("%Y-%m-%dT%H:%M").to_string();
            };
        }
        ".html.body.div.table.tbody.tr.th" => {
            let msg = list.messages.last_mut().unwrap();
            if data.matches(&session.owner).count() > 0 {
                msg.sender_friendly_name = session.owner.to_string();
                msg.receiver_friendly_name = session.recipient.to_string();
            } else {
                msg.sender_friendly_name = session.recipient.to_string();
                msg.receiver_friendly_name = session.owner.to_string();
            }
        }
        ".html.body.div.table.tbody.tr.td" => {
            let msg = list.messages.last_mut().unwrap();
            let attributes = parse_attributes(&parents.1);
            if let Some(Data::System(_)) = msg.data.first() {
                msg.data.push(Data::System(data.to_string()));
                msg.data.swap_remove(0);
            } else {
                let mut txt = Text {
                    content: data.to_string(),
                    ..Text::default()
                };
                match attributes.get("style") {
                    None => {
                        txt.style = session.message_style.clone();
                    }
                    Some(style) => {
                        txt.style = style.trim().to_string();
                    }
                };
                msg.data.push(Data::Text(txt));
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn parse_sample_file() {
        let path = "test/alice@example.com.html";
        let result = parse(path);

        let mut f = File::open("test/Images/MsgPlus_Img0663.png").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let expected = MessagesList {
            file_type: FileType::MessengerPlus,
            recipient_id: "alice@example.com".to_string(),
            first_session_id: "Session_2009-08-05T19-30-21".to_string(),
            last_session_id: "Session_2009-08-05T19-30-21".to_string(),
            messages: vec![
                Message {
                    datetime: "2009-08-05T19:30:21".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Bob".to_string(),
                    receiver_friendly_name: "Alice".to_string(),
                    data: vec![
                        Data::Text(Text {
                            style: "font-family:\"Courier New\";color:#004000;".to_string(),
                            content: "Hello Alice!".to_string(),
                        }),
                        Data::Text(Text {
                            style: "font-family:\"Courier New\";color:#004000;".to_string(),
                            content: "How are you?".to_string(),
                        }),
                    ],
                },
                Message {
                    datetime: "2009-08-05T19:30".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Alice".to_string(),
                    receiver_friendly_name: "Bob".to_string(),
                    data: vec![
                        Data::Text(Text {
                            style: "font-family:\"Segoe UI\";".to_string(),
                            content: "I'm fine, thank you!".to_string(),
                        }),
                        Data::Text(Text {
                            style: "font-family:\"Segoe UI\";".to_string(),
                            content: "What about you?".to_string(),
                        }),
                        Data::Text(Text {
                            style: "font-family:\"Segoe UI\";".to_string(),
                            content: "Have you called John about this weekend?".to_string(),
                        }),
                    ],
                },
                Message {
                    datetime: "2009-08-05T19:31".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Bob".to_string(),
                    receiver_friendly_name: "Alice".to_string(),
                    data: vec![
                        Data::Text(Text {
                            style: "font-family:\"Courier New\";color:#004000;".to_string(),
                            content: "Yes!".to_string(),
                        }),
                        Data::Text(Text {
                            style: "font-family:\"Courier New\";color:#004000;".to_string(),
                            content: "He should have called you...".to_string(),
                        }),
                    ],
                },
                Message {
                    datetime: "2009-08-05T19:31".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Alice".to_string(),
                    receiver_friendly_name: "Bob".to_string(),
                    data: vec![Data::Text(Text {
                        style: "font-family:\"Segoe UI\";".to_string(),
                        content: "He didn't!".to_string(),
                    })],
                },
                Message {
                    datetime: "2009-08-05T19:35".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Bob".to_string(),
                    receiver_friendly_name: "Alice".to_string(),
                    data: vec![
                        Data::Image(Image {
                            src: "./Images/MsgPlus_Img0663.png".to_string(),
                            alt: ":)".to_string(),
                            content: buffer,
                        }),
                        Data::Text(Text {
                            style: "font-family:\"Courier New\";color:#004000;".to_string(),
                            content: "Maybe you can call him?".to_string(),
                        }),
                    ],
                },
                Message {
                    datetime: "2009-08-05T19:44".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "".to_string(),
                    receiver_friendly_name: "".to_string(),
                    data: vec![Data::System("Alice is now offline".to_string())],
                },
            ],
        };
        assert_eq!(result.unwrap(), expected);
    }
}
