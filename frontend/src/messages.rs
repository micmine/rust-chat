use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventSource, MessageEvent};
use yew::prelude::*;

use crate::apiclient::Message;

#[derive(Debug)]
pub struct Messages {
    es: EventSource,
    _listener: Option<EventListener>,
}

#[derive(Debug)]
pub enum Msg {
    EsReady(Result<Message, serde_json::Error>),
}

impl Component for Messages {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let es = EventSource::new("/api/chat/events")
            .map_err(|js_value: JsValue| {
                let err: js_sys::Error = js_value.dyn_into().unwrap();
                err
            })
            .unwrap();

        let cb = ctx.link().callback(|bufstr: String| {
            log::info!("{}", &bufstr);
            Msg::EsReady(serde_json::from_str(&bufstr))
        });
        let listener = EventListener::new(&es, "messages_sse", move |event: &Event| {
            let event = event.dyn_ref::<MessageEvent>().unwrap();
            let text = event.data().as_string().unwrap();
            log::info!("Got message {}", &text);
            cb.emit(text);
        });

        Self {
            es,
            _listener: Some(listener),
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        self.es.close();
        self._listener = None;
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::EsReady(asd) => {
                log::debug!("{:?}", asd);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::debug!("{:?}", self.es.onmessage());
        html! {
          <>
              <div class="chat-container chat-push">
                <div class="chat-message chat-message-right">
                    <a href="">{ "micmine" }</a>
                    <p>{ "Hey" }</p>
                </div>

                <div class="chat-message chat-message-left">
                    <a href="">{ "jony" }</a>
                    <p>{ "Hey" }</p>
                </div>

                <div class="chat-message chat-message-right">
                    <a href="">{ "micmine" }</a>
                    <p>{ "how are you doing ?" }</p>
                </div>
                <div class="chat-message chat-message-left">
                    <a href="">{ "susanne" }</a>
                    <p>{ "Lorem sunt in culpa qui officia deserunt mollit anim id est laborum." }</p>
                </div>
            </div>
          </>
        }
    }
}
