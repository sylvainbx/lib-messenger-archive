use std::error::Error;
use std::fs::File;
use std::path::Path;
use crate::messenger::{Data, FileType, Message, MessagesList, Text, Image};

pub fn parse(path: &str) -> Result<MessagesList, Box<dyn Error>> {
    let _file = File::open(&path)?;
    Ok(MessagesList {
        recipient_id: Path::new(path).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_string(),
        file_type: FileType::MessengerPlus,
        ..MessagesList::default()
    })
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;

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
                            style: "font-family:Courier New;color:#004000; ".to_string(),
                            content: "Hello Alice!".to_string()
                        }),
                       Data::Text(Text {
                           style: "font-family:Courier New;color:#004000; ".to_string(),
                           content: "How are you?".to_string()
                       })
                    ]
                },
                Message {
                    datetime: "2009-08-05T19:30".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Alice".to_string(),
                    receiver_friendly_name: "Bob".to_string(),
                    data: vec![
                        Data::Text(Text {
                            style: "font-family:Segoe UI;".to_string(),
                            content: "I'm fine, thank you!".to_string()
                        }),
                        Data::Text(Text {
                            style: "font-family:Segoe UI;".to_string(),
                            content: "What about you?".to_string()
                        }),
                       Data::Text(Text {
                           style: "font-family:Segoe UI;".to_string(),
                           content: "?Have you called John about this weekend?".to_string()
                       })
                    ]
                },
                Message {
                    datetime: "2009-08-05T19:30".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Bob".to_string(),
                    receiver_friendly_name: "Alice".to_string(),
                    data: vec![
                        Data::Text(Text {
                            style: "font-family:Courier New;color:#004000; ".to_string(),
                            content: "Yes!".to_string()
                        }),
                       Data::Text(Text {
                           style: "font-family:Courier New;color:#004000; ".to_string(),
                           content: ">He should have called you...".to_string()
                       })
                    ]
                },
                Message {
                    datetime: "2009-08-05T19:31".to_string(),
                    timezone_offset: None,
                    session_id: "Session_2009-08-05T19-30-21".to_string(),
                    sender_friendly_name: "Alice".to_string(),
                    receiver_friendly_name: "Bob".to_string(),
                    data: vec![
                        Data::Text(Text {
                            style: "font-family:Segoe UI;".to_string(),
                            content: "He didn't!".to_string()
                        })
                    ]
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
                            style: "font-family:Courier New;color:#004000; ".to_string(),
                            content: "Maybe you can call him?".to_string()
                        })
                    ]
                },
            ]
        };
        assert_eq!(result.unwrap(), expected);
    }
}