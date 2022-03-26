use gloo_events::EventListener;
use web_sys::EventSource;
use yew::prelude::*;

use crate::apiclient::Message;

#[derive(Debug)]
pub struct Messages {
    es: EventSource,
    _listener: Option<EventListener>,
}

#[derive(Debug)]
pub enum Msg {
    NewMessage(Result<Message, serde_json::Error>),
}

impl Component for Messages {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb = ctx
            .link()
            .callback(|bufstr: String| Msg::NewMessage(serde_json::from_str(&bufstr)));

        let (es, listener) = crate::apiclient::load(cb);

        Self {
            es,
            _listener: Some(listener),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewMessage(messages) => {
                match messages {
                    Ok(message) => {
                        log::info!("{:?}", message);
                    }
                    Err(_) => {
                        log::error!("Got invlaid message");
                    }
                }
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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

    fn destroy(&mut self, ctx: &Context<Self>) {
        self.es.close();
        self._listener = None;
    }
}
