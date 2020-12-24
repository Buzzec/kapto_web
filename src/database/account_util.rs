use pbkdf2::{CheckError, pbkdf2_check, pbkdf2_simple};
use serde::Serialize;

use crate::api::generic::ReadableError;

pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~-";
pub const WORK_FACTOR: u8 = 10;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_USERNAME_LENGTH: usize = 64;
pub const MAX_EMAIL_LENGTH: usize = 128;

pub fn hash_password(password: &str) -> PasswordResult<String> {
    if password.len() < MIN_PASSWORD_LENGTH {
        Err(PasswordError::TooShort { length: password.len(), min_length: MIN_PASSWORD_LENGTH })
    } else {
        Ok(pbkdf2_simple(&password, 1 << WORK_FACTOR)?)
    }
}
pub fn verify_password(password: &str, hash: &str) -> PasswordResult<()> {
    Ok(pbkdf2_check(password, hash)?)
}
pub type PasswordResult<T> = Result<T, PasswordError>;
#[derive(Debug, Serialize)]
pub enum PasswordError {
    RandError(String),
    CheckError(String),
    TooShort { length: usize, min_length: usize },
}
impl From<rand_core::Error> for PasswordError {
    fn from(from: rand_core::Error) -> Self {
        Self::RandError(format!("{:?}", from))
    }
}
impl From<CheckError> for PasswordError {
    fn from(from: CheckError) -> Self {
        Self::CheckError(format!("{:?}", from))
    }
}
impl ReadableError for PasswordError {
    fn read(&self) -> String {
        match self {
            PasswordError::RandError(error) => format!("Random lib error: {}", error),
            PasswordError::CheckError(error) => format!("Password check failed: {}", error),
            PasswordError::TooShort { length, min_length } => format!("Password too short, min length={}, length={}", min_length, length),
        }
    }
}

pub fn check_username(username: &str) -> UsernameResult<()> {
    if username.len() > MAX_USERNAME_LENGTH {
        Err(UsernameError::TooLong { max_length: MAX_USERNAME_LENGTH, length: username.len() })
    } else {
        Ok(())
    }
}
pub type UsernameResult<T> = Result<T, UsernameError>;
#[derive(Debug, Serialize)]
pub enum UsernameError {
    TooLong { max_length: usize, length: usize },
    ContainsInvalidCharacter(char),
}
impl ReadableError for UsernameError {
    fn read(&self) -> String {
        match self {
            UsernameError::TooLong { max_length, length } => format!("username is too long, length={}, max length={}", length, max_length),
            UsernameError::ContainsInvalidCharacter(char) => format!("username contains invalid character: {}", char),
        }
    }
}

pub fn check_email(email: &str) -> EmailResult<()> {
    for char in email.chars() {
        if char == '@' {
            return Ok(());
        }
    }
    Err(EmailError::MissingAt)
}
pub type EmailResult<T> = Result<T, EmailError>;
#[derive(Debug, Serialize)]
pub enum EmailError {
    MissingAt,
}
impl ReadableError for EmailError {
    fn read(&self) -> String {
        match self {
            EmailError::MissingAt => "Email is missing the @ sign".to_string(),
        }
    }
}
