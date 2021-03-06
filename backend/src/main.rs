mod chat;

#[macro_use]
extern crate rocket;

use chat::Message;
use crate::chat::{post, events};
use rocket::tokio::sync::broadcast::channel;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/api/chat", routes![post, events])
}
