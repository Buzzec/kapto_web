use mysql_async::{FromRowError, Row};
use mysql_async::chrono::NaiveDate;
use mysql_async::prelude::FromRow;
use rand::{RngCore, thread_rng};
use rand::rngs::ThreadRng;

use crate::api::token::AuthToken;
use crate::database::{DatabaseError, DatabaseResult, get_from_row};
use crate::database::connection_pool::ConnectionPool;
use crate::database::procedures::Procedure;

const TOKEN_LENGTH: usize = 128 / 8;

#[derive(Clone, Debug)]
pub struct DatabaseToken {
    pub id: u64,
    pub user_id: u64,
    pub token: Vec<u8>,
    pub expires: Option<NaiveDate>,
}
impl DatabaseToken {
    pub fn new(user_id: u64) -> Self {
        Self {
            id: Default::default(),
            user_id,
            token: Self::random_token(&mut thread_rng(), TOKEN_LENGTH),
            expires: None,
        }
    }
    fn random_token(random: &mut ThreadRng, size: usize) -> Vec<u8> {
        let mut out = vec![Default::default(); size];
        random.fill_bytes(&mut out);
        out
    }

    pub async fn by_id(id: u64, pool: &ConnectionPool) -> DatabaseResult<Option<Self>> {
        Procedure::GetTokenById.exec_first(&mut pool.get_connection().await?, vec![
            ("id", id.into()),
        ]).await
    }

    pub async fn create_token(self, pool: &ConnectionPool) -> DatabaseResult<Self> {
        match Procedure::CreateToken.exec_first(&mut pool.get_connection().await?, vec![
            ("user_id", self.user_id.into())
        ]).await? {
            Some(value) => Ok(value),
            None => Err(DatabaseError::NotFound),
        }
    }

    pub async fn verify_token(token: AuthToken, pool: &ConnectionPool) -> DatabaseResult<Self> {
        match Self::by_id(token.id, pool).await? {
            None => Err(DatabaseError::TokenFailed),
            Some(retrieved) => if retrieved.token == token.token { Ok(retrieved) } else { Err(DatabaseError::TokenFailed) }
        }
    }
}
impl FromRow for DatabaseToken {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> where
        Self: Sized {
        let (id, row) = get_from_row(row, 0)?;
        let (user_id, row) = get_from_row(row, 1)?;
        let (token, row) = get_from_row(row, 2)?;
        let (expires, _) = get_from_row(row, 3)?;
        Ok(Self {
            id,
            user_id,
            token,
            expires,
        })
    }
}
