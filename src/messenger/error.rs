use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MsnMessengerError {
    details: String
}

impl MsnMessengerError {
    pub fn new(msg: &str) -> MsnMessengerError {
        MsnMessengerError { details: msg.to_string() }
    }
}

impl fmt::Display for MsnMessengerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MsnMessengerError {
    fn description(&self) -> &str {
        &self.details
    }
}