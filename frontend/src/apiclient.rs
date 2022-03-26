use gloo_events::EventListener;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{EventSource, MessageEvent};

use serde_json::Result;
use yew::prelude::*;

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

pub fn load(cb: Callback<String>) -> (EventSource, EventListener) {
    let es = EventSource::new("/api/chat/events")
        .map_err(|js_value: JsValue| {
            let err: js_sys::Error = js_value.dyn_into().unwrap();
            err
        })
        .unwrap();

    let listener = EventListener::new(&es, "messages", move |event: &Event| {
        let event = event.dyn_ref::<MessageEvent>().unwrap();
        let text = event.data().as_string().unwrap();
        cb.emit(text);
    });

    (es, listener)
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
