use std::sync::Arc;

use crate::api::generic::{handle_request, Request};
use crate::database::connection_pool::ConnectionPool;

pub mod generic;
pub mod token;
pub mod user;

pub async fn api_handler(request: Request, pool: Arc<ConnectionPool>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&handle_request(request, pool).await))
}
