use mysql_async::{BoxFuture, Conn, Statement, Value};
use mysql_async::prelude::{FromRow, Queryable};

use crate::database::DatabaseResult;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Procedure {
    // Ruleset
    DeleteRuleset,
    GetRulesetById,
    GetRulesetsForUser,
    InsertRuleset,

    // User
    DeleteUser,
    GetUserByEmail,
    GetUserById,
    GetUserByUsername,
    InsertUser,
}
impl Procedure {
    pub fn get_statement<'a>(&self, connection: &'a mut Conn) -> BoxFuture<'a, Statement>{
        connection.prep(match self {
            // Ruleset
            Self::DeleteRuleset => "CALL delete_ruleset(:id)",
            Self::GetRulesetById => "CALL get_ruleset_by_id(:id)",
            Self::GetRulesetsForUser => "CALL get_rulesets_for_user(:user_id, :limit, :offset)",
            Self::InsertRuleset => "CALL insert_ruleset(:user_id, :version, :ruleset)",
            // User
            Self::DeleteUser => "CALL delete_user(:id)",
            Self::GetUserByEmail => "CALL get_user_by_email(:email)",
            Self::GetUserById => "CALL ger_user_by_id(:id)",
            Self::GetUserByUsername => "CALL get_user_by_username(:username)",
            Self::InsertUser => "CALL insert_user(:username, :email, :password_hash)",
        })
    }

    #[inline]
    pub async fn exec<T, N>(&self, connection: &mut Conn, params: Vec<(N, Value)>) -> DatabaseResult<Vec<T>>
        where T: FromRow + Send + 'static,
              N: Send,
              String: From<N>{
        let statement = self.get_statement(&mut *connection).await?;
        Self::statement_exec(&statement, connection, params).await
    }
    #[inline]
    pub async fn exec_first<T, N>(&self, connection: &mut Conn, params: Vec<(N, Value)>) -> DatabaseResult<Option<T>>
        where T: FromRow + Send + 'static,
              N: Send,
              String: From<N>{
        let statement = self.get_statement(&mut *connection).await?;
        Self::statement_exec_first(&statement, connection, params).await
    }
    #[inline]
    pub async fn exec_drop<N>(&self, connection: &mut Conn, params: Vec<(N, Value)>) -> DatabaseResult<()>
        where N: Send,
              String: From<N>{
        let statement = self.get_statement(&mut *connection).await?;
        Self::statement_exec_drop(&statement, connection, params).await
    }

    #[inline]
    pub async fn statement_exec<T, N>(statement: &Statement, connection: &mut Conn, params: Vec<(N, Value)>) -> DatabaseResult<Vec<T>>
        where T: FromRow + Send + 'static,
              N: Send,
              String: From<N>{
        Ok(connection.exec(statement, params).await?)
    }
    #[inline]
    pub async fn statement_exec_first<T, N>(statement: &Statement, connection: &mut Conn, params: Vec<(N, Value)>) -> DatabaseResult<Option<T>>
        where T: FromRow + Send + 'static,
              N: Send,
              String: From<N>{
        Ok(connection.exec_first(statement, params).await?)
    }
    #[inline]
    pub async fn statement_exec_drop<N>(statement: &Statement, connection: &mut Conn, params: Vec<(N, Value)>) -> DatabaseResult<()>
        where N: Send,
              String: From<N>{
        Ok(connection.exec_drop(statement, params).await?)
    }
}
