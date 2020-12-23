use mysql_async::{Conn, FromRowError, Row};
use mysql_async::prelude::FromRow;

use crate::database::{DatabaseResult, get_from_row};
use crate::database::DatabaseError::*;
use crate::database::paging::Paging;
use crate::database::procedures::Procedure;
use crate::game::ruleset::Ruleset;

pub struct DatabaseRuleset {
    pub id: u64,
    pub user_id: u64,
    pub version: u32,
    pub ruleset: Ruleset,
}
impl DatabaseRuleset {
    pub async fn by_id(connection: &mut Conn, id: u64) -> DatabaseResult<Option<Self>>{
        Procedure::GetRulesetById.exec_first(connection, vec![
            ("id", id.into()),
        ]).await
    }
    pub async fn for_user(connection: &mut Conn, user_id: u64, paging: Paging) -> DatabaseResult<Vec<Self>>{
        Procedure::GetRulesetsForUser.exec(connection, vec![
            ("user_id", user_id.into()),
            ("limit", paging.limit.into()),
            ("offset", paging.offset.into()),
        ]).await
    }
    pub async fn delete(connection: &mut Conn, id: u64) -> DatabaseResult<()>{
        Procedure::DeleteRuleset.exec_drop(connection, vec![
            ("id", id.into()),
        ]).await
    }

    pub async fn insert_into_database(self, connection: &mut Conn) -> DatabaseResult<Self>{
        match Procedure::InsertRuleset.exec_first(connection, vec![
            ("user_id", self.user_id.into()),
            ("version", self.version.into()),
            ("ruleset", serde_json::to_string(&self.ruleset)?.into())
        ]).await?{
            None => Err(NotFound),
            Some(ruleset) => Ok(ruleset),
        }
    }
}
impl FromRow for DatabaseRuleset {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> where Self: Sized {
        let (id, row) = get_from_row(row, 0)?;
        let (user_id, row) = get_from_row(row, 1)?;
        let (version, row) = get_from_row(row, 2)?;
        let (ruleset_raw, row) = get_from_row::<String>(row, 3)?;

        let ruleset = match serde_json::from_str::<Ruleset>(&ruleset_raw){
            Ok(ruleset) => ruleset,
            Err(error) => {
                eprintln!("Error deserializing ruleset id {}: {}, text: {}", id, error, ruleset_raw);
                return Err(FromRowError(row));
            },
        };

        Ok(Self{
            id,
            user_id,
            version,
            ruleset,
        })
    }
}
