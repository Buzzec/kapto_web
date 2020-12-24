use std::sync::Arc;

use serde::Deserialize;
use warp::Filter;

use crate::api::api_handler;
use crate::database::connection_pool::ConnectionPool;

pub mod api;
pub mod database;
pub mod game;

pub mod chat;
pub mod util;

#[tokio::main]
async fn main() {
    // let echo_route = warp::path!("echo")
    //     .and(warp::ws())
    //     .map(|ws: warp::ws::Ws| {
    //         ws.on_upgrade(|websocket| {
    //             let (tx, rx) = websocket.split();
    //             rx.forward(tx).map(|result| {
    //                 if let Err(e) = result {
    //                     eprintln!("websocket error: {:?}", e);
    //                 }
    //             })
    //         })
    //     });


    // let users = Users::default();
    // let users = warp::any().map(move || users.clone());

    // let chat_route = warp::path!("chat")
    //     .and(warp::query())
    //     .and(warp::ws())
    //     .and(users)
    //     .map(|username: Username, ws: warp::ws::Ws, users| {
    //         ws.on_upgrade(move |socket| chat::user_connected(socket, users, username.username))
    //     });

    let pool = Arc::new(ConnectionPool::new().expect("Could not connect to database"));
    let api_route = warp::path!("api")
        .and(warp::post())
        .and(json_body())
        .and(ConnectionPool::filter(pool.clone()))
        .and_then(api_handler);

    let web_route = warp::fs::dir("website/dist");

    warp::serve(
        api_route
            .or(web_route)
    ).run(([0, 0, 0, 0], 3030)).await;
}

fn json_body<T>() -> impl Filter<Extract=(T, ), Error=warp::Rejection> + Clone
    where for<'de> T: Deserialize<'de> + Send {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}
