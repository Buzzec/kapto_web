use serde::{Deserialize, Serialize};

use crate::database::token::DatabaseToken;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthToken {
    pub id: u64,
    pub token: Vec<u8>,
}
impl From<DatabaseToken> for AuthToken {
    fn from(from: DatabaseToken) -> Self {
        Self {
            id: from.id,
            token: from.token,
        }
    }
}
