use mysql_async::{FromRowError, Row};
use mysql_async::prelude::FromRow;
use serde::{Deserialize, Serialize};

use crate::database::{DatabaseResult, get_from_row};
use crate::database::connection_pool::ConnectionPool;
use crate::database::DatabaseError::NotFound;
use crate::database::procedures::Procedure::*;

pub struct DatabaseUser{
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
}
impl DatabaseUser {
    pub async fn delete(pool: &ConnectionPool, id: u64) -> DatabaseResult<()> {
        DeleteUser.exec_drop(&mut pool.get_connection().await?, vec![
            ("id", id.into()),
        ]).await
    }
    pub async fn by_email(pool: &ConnectionPool, email: &str) -> DatabaseResult<Option<Self>> {
        GetUserByEmail.exec_first(&mut pool.get_connection().await?, vec![
            ("email", email.into()),
        ]).await
    }
    pub async fn by_id(pool: &ConnectionPool, id: u64) -> DatabaseResult<Option<Self>> {
        GetUserById.exec_first(&mut pool.get_connection().await?, vec![
            ("id", id.into()),
        ]).await
    }
    pub async fn by_username(pool: &ConnectionPool, username: &str) -> DatabaseResult<Option<Self>> {
        GetUserByUsername.exec_first(&mut pool.get_connection().await?, vec![
            ("username", username.into()),
        ]).await
    }
    pub async fn by_user_identification(pool: &ConnectionPool, identification: UserIdentification) -> DatabaseResult<Option<Self>> {
        match identification {
            UserIdentification::Id(id) => Self::by_id(pool, id).await,
            UserIdentification::Username(username) => Self::by_username(pool, &username).await,
            UserIdentification::Email(email) => Self::by_email(pool, &email).await,
        }
    }

    pub async fn insert_into_database(self, pool: &ConnectionPool) -> DatabaseResult<Self> {
        match InsertUser.exec_first::<Self, _>(&mut pool.get_connection().await?, vec![
            ("username", self.username.into()),
            ("email", self.email.into()),
            ("password_hash", self.password_hash.into())
        ]).await? {
            None => Err(NotFound),
            Some(user) => Ok(user),
        }
    }
}
impl FromRow for DatabaseUser{
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized {
        let (id, row) = get_from_row(row, 0)?;
        let (username, row) = get_from_row(row, 1)?;
        let (email, row) = get_from_row(row, 2)?;
        let (password_hash, row) = get_from_row(row, 3)?;
        let (is_admin, _) = get_from_row(row, 4)?;

        Ok(Self {
            id,
            username,
            email,
            password_hash,
            is_admin,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserIdentification {
    Id(u64),
    Username(String),
    Email(String),
}
impl PartialEq<DatabaseUser> for UserIdentification {
    fn eq(&self, other: &DatabaseUser) -> bool {
        match self {
            UserIdentification::Id(id) => id.eq(&other.id),
            UserIdentification::Username(username) => username.eq(&other.username),
            UserIdentification::Email(email) => email.eq(&other.email),
        }
    }
}
