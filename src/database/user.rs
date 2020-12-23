use mysql_async::{Conn, FromRowError, Row};
use mysql_async::prelude::FromRow;

use crate::database::{DatabaseResult, get_from_row};
use crate::database::DatabaseError::NotFound;
use crate::database::procedures::Procedure::*;

pub struct DatabaseUser{
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
}
impl DatabaseUser{
    pub async fn delete(connection: &mut Conn, id: u64) -> DatabaseResult<()>{
        DeleteUser.exec_drop(connection, vec![
            ("id", id.into()),
        ]).await
    }
    pub async fn by_email(connection: &mut Conn, email: &str) -> DatabaseResult<Option<Self>>{
        GetUserByEmail.exec_first(connection, vec![
            ("email", email.into()),
        ]).await
    }
    pub async fn by_id(connection: &mut Conn, id: u64) -> DatabaseResult<Option<Self>>{
        GetUserById.exec_first(connection, vec![
            ("id", id.into()),
        ]).await
    }
    pub async fn by_username(connection: &mut Conn, username: &str) -> DatabaseResult<Option<Self>>{
        GetUserByUsername.exec_first(connection, vec![
            ("username", username.into()),
        ]).await
    }

    pub async fn insert_into_database(self, connection: &mut Conn) -> DatabaseResult<Self>{
        match InsertUser.exec_first::<Self, _>(connection, vec![
            ("username", self.username.into()),
            ("email", self.email.into()),
            ("password_hash", self.password_hash.into())
        ]).await?{
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

        Ok(Self{
            id,
            username,
            email,
            password_hash,
            is_admin
        })
    }
}
