use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::RwLock;
use warp::filters::ws::{Message, WebSocket};

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
pub type Users = Arc<RwLock<HashMap<usize, UnboundedSender<Result<Message, warp::Error>>>>>;

#[derive(Deserialize)]
pub struct Username {
    pub username: String
}

pub async fn user_connected(ws: WebSocket, users: Users, username: String) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    eprintln!("new chat user: {}#{}", username, my_id);

    let (user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = unbounded_channel();
    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    users.write().await.insert(my_id, tx);
    send_message(format!("<{}#{} Connected>", username, my_id), None, &users).await;

    while let Some(result) = user_ws_rx.next().await {
        let message = match result {
            Ok(message) => message,
            Err(error) => {
                eprintln!("websocket error (uid={}): {}", my_id, error);
                break;
            }
        };
        user_message(my_id, &username, message, &users).await;
    }
    user_disconnected(my_id, username, users).await;
}

async fn user_message(my_id: usize, username: &str, message: Message, users: &Users) {
    let message = match message.to_str() {
        Ok(message) => message,
        Err(_) => return,
    };

    let new_message = format!("<{}#{}>: {}", username, my_id, message);
    send_message(new_message, Some(&[my_id]), users).await;
}

async fn send_message(message: String, ignore_ids: Option<&[usize]>, users: &Users) {
    for (uid, tx) in users.read().await.iter() {
        if let Some(ids) = ignore_ids {
            if ids.contains(uid) {
                continue;
            }
        }
        if let Err(_disconnected) = tx.send(Ok(Message::text(message.clone()))) {};
    }
}

async fn user_disconnected(my_id: usize, username: String, users: Users) {
    eprintln!("good bye user: {}#{}", username, my_id);
    send_message(format!("<{}#{} Disconnected>", username, my_id), None, &users).await;

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}
