use futures::{FutureExt, StreamExt};
use warp::Filter;

use crate::chat::{Username, Users};

pub mod chat;

#[tokio::main]
async fn main() {
    let echo_route = warp::path!("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let web_route = warp::path!("web" / ..)
        .and(warp::fs::dir("website/dist"));

    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let chat_route = warp::path!("chat")
        .and(warp::query())
        .and(warp::ws())
        .and(users)
        .map(|username: Username, ws: warp::ws::Ws, users| {
            ws.on_upgrade(move |socket| chat::user_connected(socket, users, username.username))
        });

    warp::serve(root.or(echo_route).or(web_route).or(chat_route)).run(([0, 0, 0, 0], 3030)).await;
}
