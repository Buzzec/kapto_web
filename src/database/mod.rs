use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fmt::{Debug, Formatter};

use mysql_async::{FromRowError, Row};
use mysql_async::prelude::FromValue;
use serde::{Deserialize, Serialize};

use crate::api::generic::ReadableError;

pub mod connection_pool;
pub mod paging;
pub mod account_util;
pub mod procedures;
pub mod ruleset;
pub mod token;
pub mod user;

pub type DatabaseResult<T> = Result<T, DatabaseError>;
#[derive(Debug, Serialize, Deserialize)]
pub enum DatabaseError {
    NotFound,
    EnvironmentVariableError(OsString),
    MysqlError(String),
    SerdeJsonError(String),
    TokenFailed,
}
impl From<OsString> for DatabaseError{
    fn from(from: OsString) -> Self {
        Self::EnvironmentVariableError(from)
    }
}
impl From<mysql_async::Error> for DatabaseError{
    fn from(from: mysql_async::Error) -> Self {
        Self::MysqlError(format!("{:?}", from))
    }
}
impl From<serde_json::Error> for DatabaseError {
    fn from(from: serde_json::Error) -> Self {
        Self::SerdeJsonError(format!("{:?}", from))
    }
}
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for DatabaseError {}
impl ReadableError for DatabaseError {
    fn read(&self) -> String {
        match self {
            DatabaseError::NotFound => "Data was attempted to be inserted but was not found afterward".to_string(),
            DatabaseError::EnvironmentVariableError(string) => format!("OsString could not be converted to utf-8: {:?}", string),
            DatabaseError::MysqlError(error) => format!("Database Error: {}", error),
            DatabaseError::SerdeJsonError(error) => format!("Serialization Error: {}", error),
            Self::TokenFailed => "Token verification failed".to_string(),
        }
    }
}

fn get_from_row<T: FromValue>(row: Row, index: usize) -> Result<(T, Row), FromRowError> {
    match row.get(index) {
        None => Err(FromRowError(row)),
        Some(val) => Ok((val, row)),
    }
}

