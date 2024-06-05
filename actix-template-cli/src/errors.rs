use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultError {
    message: String,
}

impl DefaultError {
    pub fn new(message: &str) -> Self {
        DefaultError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for DefaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DefaultError {}

impl From<Box<dyn Error>> for DefaultError {
    fn from(error: Box<dyn Error>) -> Self {
        DefaultError {
            message: error.to_string(),
        }
    }
}

impl<T> From<actix_web_starter_client::apis::Error<T>> for DefaultError {
    fn from(error: actix_web_starter_client::apis::Error<T>) -> Self {
        DefaultError {
            message: error.to_string(),
        }
    }
}

impl From<inquire::InquireError> for DefaultError {
    fn from(error: inquire::InquireError) -> Self {
        DefaultError {
            message: error.to_string(),
        }
    }
}
