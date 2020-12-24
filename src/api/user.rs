use std::sync::Arc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::api::generic::{ReadableError, Response, ResponseData};
use crate::api::token::AuthToken;
use crate::database::account_util::{check_email, check_username, EmailError, hash_password, PasswordError, UsernameError, verify_password};
use crate::database::connection_pool::ConnectionPool;
use crate::database::DatabaseError;
use crate::database::token::DatabaseToken;
use crate::database::user::{DatabaseUser, UserIdentification};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserRequest {
    Register {
        username: String,
        email: String,
        password: String,
    },
    Login {
        username: String,
        password: String,
    },
    DeleteUser {
        username: String,
        password: String,
    },
    GetUserInfo(UserIdentification),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserResponse {
    PartialUser {
        id: u64,
        username: String,
        is_admin: bool,
    },
    FullUser {
        id: u64,
        username: String,
        email: String,
        is_admin: bool,
    },
}
impl UserResponse {
    fn partial_user(user: DatabaseUser) -> Self {
        Self::PartialUser {
            id: user.id,
            username: user.username,
            is_admin: user.is_admin,
        }
    }

    fn full_user(user: DatabaseUser) -> Self {
        Self::FullUser {
            id: user.id,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin,
        }
    }
}

pub async fn handle_user_request(token: Option<AuthToken>, request: UserRequest, pool: Arc<ConnectionPool>) -> Response {
    match try_handle_user_request(token, request, pool).await {
        Ok(response) => response,
        Err(error) => Response::from(error),
    }
}
async fn try_handle_user_request(token: Option<AuthToken>, request: UserRequest, pool: Arc<ConnectionPool>) -> UserResult<Response> {
    match request {
        UserRequest::Register { username, email, password } => {
            check_username(&username)?;
            check_email(&email)?;
            let user_username_search = DatabaseUser::by_username(&pool, &username);
            let user_email_search = DatabaseUser::by_email(&pool, &email);
            if user_username_search.await?.is_some() {
                return Err(UserError::UsernameTaken(username));
            }
            if user_email_search.await?.is_some() {
                return Err(UserError::EmailTaken(email));
            }
            let hash = hash_password(&password)?;

            let user = DatabaseUser {
                id: 0,
                username,
                email,
                password_hash: hash,
                is_admin: false,
            }.insert_into_database(&pool).await?;

            Ok(Response {
                token: None,
                data: ResponseData::User(UserResponse::full_user(user)),
            })
        },
        UserRequest::Login { username, password } => {
            let time1 = SystemTime::now();
            let user = verify_user_password(&username, &password, &pool).await?;
            let token = DatabaseToken::new(user.id).create_token(&pool).await?;
            let out = Ok(Response {
                token: Some(token.into()),
                data: ResponseData::User(UserResponse::full_user(user)),
            });
            println!("Login Time: {:?}", SystemTime::now().duration_since(time1));
            out
        },
        UserRequest::DeleteUser { username, password } => {
            let user = verify_user_password(&username, &password, &pool).await?;
            DatabaseUser::delete(&pool, user.id).await?;
            Ok(Response {
                token: None,
                data: ResponseData::GenericSuccess,
            })
        },
        UserRequest::GetUserInfo(identification) => {
            let user = DatabaseUser::by_user_identification(&pool, identification);
            let token = match token {
                None => None,
                Some(token) => Some(DatabaseToken::verify_token(token, &pool).await?),
            };
            let requested = match user.await? {
                None => return Err(UserError::UserNotFound),
                Some(user) => user,
            };
            let requester = if let Some(token_inner) = &token {
                DatabaseUser::by_id(&pool, token_inner.user_id).await?
            } else {
                None
            };
            if let Some(requester) = requester {
                if requester.is_admin || requester.id == requested.id {
                    return Ok(Response {
                        token: None,
                        data: ResponseData::User(UserResponse::full_user(requested)),
                    })
                }
            }
            Ok(Response {
                token: None,
                data: ResponseData::User(UserResponse::partial_user(requested)),
            })
        },
    }
}

async fn verify_user_password(username: &str, password: &str, pool: &ConnectionPool) -> UserResult<DatabaseUser> {
    let user = match DatabaseUser::by_username(pool, username).await? {
        None => return Err(UserError::LoginError),
        Some(user) => user,
    };
    verify_password(&password, &user.password_hash)?;
    Ok(user)
}

type UserResult<T> = Result<T, UserError>;
#[derive(Debug, Serialize)]
enum UserError {
    UserNotFound,
    UsernameTaken(String),
    EmailTaken(String),
    LoginError,
    DatabaseError(DatabaseError),
    UsernameError(UsernameError),
    EmailError(EmailError),
    PasswordError(PasswordError),
}
impl From<DatabaseError> for UserError {
    fn from(from: DatabaseError) -> Self {
        Self::DatabaseError(from)
    }
}
impl From<UsernameError> for UserError {
    fn from(from: UsernameError) -> Self {
        Self::UsernameError(from)
    }
}
impl From<EmailError> for UserError {
    fn from(from: EmailError) -> Self {
        Self::EmailError(from)
    }
}
impl From<PasswordError> for UserError {
    fn from(from: PasswordError) -> Self {
        Self::PasswordError(from)
    }
}
impl ReadableError for UserError {
    fn read(&self) -> String {
        match self {
            Self::UserNotFound => "User was not found".to_string(),
            Self::UsernameTaken(username) => format!("Username taken: {}", username),
            Self::EmailTaken(email) => format!("Email taken: {}", email),
            Self::LoginError => "Error logging in".to_string(),
            Self::DatabaseError(error) => error.read(),
            Self::UsernameError(error) => error.read(),
            Self::EmailError(error) => error.read(),
            Self::PasswordError(error) => error.read(),
        }
    }
}
