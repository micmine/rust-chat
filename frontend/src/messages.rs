use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::apiclient;

#[derive(Debug)]
pub struct Messages;

#[derive(Debug)]
pub enum Msg {}

impl Component for Messages {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        spawn_local(async move {
            apiclient::load().await;
        });
        Self
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
                        <p>{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }</p>
                    </div>
                </div>
              </>
            }
    }
}
