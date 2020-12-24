use std::convert::Infallible;
use std::str::FromStr;
use std::sync::Arc;

use mysql_async::{Conn, OptsBuilder, Pool};
use warp::Filter;

use crate::database::DatabaseResult;
use crate::util::get_env_var_array;

pub const DATABASE_USER: [&str; 2] = ["KAPTO_WEB_DATABASE_USER", "root"];
pub const DATABASE_PASSWORD: [&str; 2] = ["KAPTO_WEB_DATABASE_PASSWORD", "coolpassword"];
pub const DATABASE_HOST: [&str; 2] = ["KAPTO_WEB_DATABASE_HOST", "192.168.55.117"];
pub const DATABASE_PORT: [&str; 2] = ["KAPTO_WEB_DATABASE_PORT", "3307"];
pub const DATABASE_NAME: [&str; 2] = ["KAPTO_WEB_DATABASE_NAME", "kapto"];

pub struct ConnectionPool{
    pool: Pool,
}
impl ConnectionPool{
    pub fn new() -> DatabaseResult<Self>{
        let user = get_env_var_array(&DATABASE_USER)?;
        let password = get_env_var_array(&DATABASE_PASSWORD)?;
        let host = get_env_var_array(&DATABASE_HOST)?;
        let port = get_env_var_array(&DATABASE_PORT)?;
        let db_name = get_env_var_array(&DATABASE_NAME)?;

        Ok(Self::connect(user, password, host, port, db_name))
    }

    fn connect(user: String, password: String, host: String, port: String, db_name: String) -> Self{
        Self{
            pool: Pool::new(OptsBuilder::default()
                .user(Some(user))
                .pass(Some(password))
                .ip_or_hostname(host)
                .tcp_port(u16::from_str(&port).unwrap())
                .db_name(Some(db_name))
            ),
        }
    }

    pub async fn get_connection(&self) -> DatabaseResult<Conn> {
        Ok(self.pool.get_conn().await?)
    }

    pub fn filter(self: Arc<Self>) -> impl Filter<Extract=(Arc<ConnectionPool>, ), Error=Infallible> + Clone {
        warp::any().map(move || self.clone())
    }
}

#[cfg(test)]
mod test {
    use mysql_async::prelude::Queryable;

    use crate::database::connection_pool::ConnectionPool;

    #[tokio::test]
    async fn connection_test() {
        let pool = ConnectionPool::new().expect("Could not connect");
        let mut connection = pool.get_connection().await.expect("Could not get connection");
        let result: i32 = connection.query_first("SELECT 100").await.expect("Could not execute query").expect("No result");
        assert_eq!(result, 100);
    }
}
