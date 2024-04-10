use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MessengerError {
    details: String
}

impl MessengerError {
    pub fn new(msg: &str) -> MessengerError {
        MessengerError { details: msg.to_string() }
    }
}

impl fmt::Display for MessengerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MessengerError {
    fn description(&self) -> &str {
        &self.details
    }
}