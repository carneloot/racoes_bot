use std::fmt::{Display, Formatter};
use crate::db;

#[derive(Debug)]
pub enum Error {
    Database(db::Error),
    TeloxideRequest(teloxide::RequestError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Database(ref err) => write!(f, "Database error {}", err),
            Error::TeloxideRequest(ref err) => write!(f, "Telegram request error {}", err),
        }
    }
}

impl From<db::Error> for Error {
    fn from(err: db::Error) -> Self {
        Self::Database(err)
    }
}

impl From<teloxide::RequestError> for Error {
    fn from(value: teloxide::RequestError) -> Self {
        Self::TeloxideRequest(value)
    }
}
