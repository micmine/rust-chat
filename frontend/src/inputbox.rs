use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::apiclient;

#[derive(Debug)]
pub struct InputBox {
    content: String,
}

#[derive(Debug)]
pub enum Msg {
    MessageChanged(String),
    Send,
}

impl Component for InputBox {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageChanged(message) => {
                self.content = message;
                false
            }
            Msg::Send => {
                log::info!("Send message, {}", self.content);
                let content = self.content.clone();
                spawn_local(async move {
                    apiclient::send(&content).await;
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_change = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::MessageChanged(input.value()))
        });

        let on_click = link.callback(|_e: MouseEvent| Msg::Send);

        html! {
            <>
                <input type="string" name="message" placeholder="..." onchange={on_change}/>
                <button onclick={on_click} >{ "Send" }</button>
            </>
        }
    }
}
