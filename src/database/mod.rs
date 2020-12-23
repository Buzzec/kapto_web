use std::error::Error;
use std::ffi::OsString;
use std::fmt;

use bitflags::_core::fmt::{Debug, Formatter};
use mysql_async::{FromRowError, Row};
use mysql_async::prelude::FromValue;

pub mod connection_pool;
pub mod paging;
pub mod procedures;
pub mod ruleset;
pub mod user;

pub type DatabaseResult<T> = Result<T, DatabaseError>;
#[derive(Debug)]
pub enum DatabaseError{
    NotFound,
    EnvironmentVariableError(OsString),
    MysqlError(mysql_async::Error),
    SerdeJsonError(serde_json::Error),
}
impl From<OsString> for DatabaseError{
    fn from(from: OsString) -> Self {
        Self::EnvironmentVariableError(from)
    }
}
impl From<mysql_async::Error> for DatabaseError{
    fn from(from: mysql_async::Error) -> Self {
        Self::MysqlError(from)
    }
}
impl From<serde_json::Error> for DatabaseError{
    fn from(from: serde_json::Error) -> Self {
        Self::SerdeJsonError(from)
    }
}
impl fmt::Display for DatabaseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}
impl Error for DatabaseError{
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            Self::NotFound => None,
            Self::EnvironmentVariableError(_) => None,
            Self::MysqlError(error) => Some(error),
            Self::SerdeJsonError(error) => Some(error),
        }
    }
}

fn get_from_row<T: FromValue>(row: Row, index: usize) -> Result<(T, Row), FromRowError>{
    match row.get(index){
        None => Err(FromRowError(row)),
        Some(val) => Ok((val, row)),
    }
}

