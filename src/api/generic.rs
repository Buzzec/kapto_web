use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::api::token::AuthToken;
use crate::api::user::{handle_user_request, UserRequest, UserResponse};
use crate::database::connection_pool::ConnectionPool;

pub type Request = TokenPacket<RequestData>;
pub type Response = TokenPacket<ResponseData>;
impl<E> From<E> for Response where E: ReadableError {
    fn from(from: E) -> Self {
        Self {
            token: None,
            data: from.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPacket<T> {
    pub token: Option<AuthToken>,
    pub data: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RequestData {
    Ping(i64),
    User(UserRequest),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResponseData {
    Pong(i64),
    GenericSuccess,
    GenericError {
        data: String,
        error_text: String,
    },
    User(UserResponse),
}
impl<E> From<E> for ResponseData where E: ReadableError {
    fn from(from: E) -> Self {
        let error_text = from.read();
        let data = match serde_json::to_string(&from) {
            Ok(data) => data,
            Err(error) => return Self::GenericError {
                data: "".to_string(),
                error_text: format!("Full failure, cause: {}, json error: {}", error_text, error),
            }
        };
        Self::GenericError { data, error_text }
    }
}

pub trait ReadableError: Debug + Serialize {
    fn read(&self) -> String;
}

pub async fn handle_request(request: Request, pool: Arc<ConnectionPool>) -> Response {
    match request.data {
        RequestData::Ping(value) => Response {
            token: None,
            data: ResponseData::Pong(value),
        },
        RequestData::User(user_request) => handle_user_request(request.token, user_request, pool).await,
    }
}
