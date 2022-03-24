use std::rc::Rc;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::EventSource;
use yew::Callback::Callback;

use crate::sse::EventSourceService;
use serde_json::Result;

//use sse_client::EventSource;

pub async fn send(message: &str) -> Result<()> {
    // TODO: validate

    let body = Message::new(message);
    let body = serde_json::to_string(&body)?;

    // create request
    let resp = Request::post(&format!("/api/chat/post",).to_string())
        .header("Token", "asd")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    println!("{:?}", resp);

    Ok(())
}

pub async fn load() {
    fn on_message(msg: String) {
        log::info!("{:?}", msg);
    }
    let callback = Callback {
        cb: Rc::new(on_message),
        passive: Some(false),
    };
    let _conection = EventSourceService::connect("/api/chat/events", callback);

    //log::info!("active: {}", _conection.event_source.ready_state());

    //let event_source = EventSource::new("/api/chat/events").unwrap();

    //event_source.on_message(|message| {
        //println!("New message event {:?}", message);
    //});
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub room: String,
    pub username: String,
    pub message: String,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            room: String::from("room"),
            username: String::from("room"),
            message: String::from("room"),
        }
    }
}

impl Message {
    fn new(message: &str) -> Self {
        Self {
            room: String::from("a"),
            username: String::from("a"),
            message: String::from(message),
        }
    }
}
