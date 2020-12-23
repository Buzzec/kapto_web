use mysql_async::{Conn, OptsBuilder, Pool};

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
                .db_name(Some(db_name + ":" + &port))
            ),
        }
    }

    pub async fn get_connection(&self) -> DatabaseResult<Conn>{
        Ok(self.pool.get_conn().await?)
    }
}

#[cfg(test)]
mod test{
    use crate::database::connection_pool::ConnectionPool;

    #[test]
    fn connection_test(){
        ConnectionPool::new().expect("Could not connect");
    }
}
