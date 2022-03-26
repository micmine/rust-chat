use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::apiclient::{self, MessageDTO};

#[derive(Debug)]
pub struct InputBox {
    content: String,
}

#[derive(Debug)]
pub enum Msg {
    MessageChanged(String),
    Send,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputBoxProps {
    pub user_id: usize,
    pub user_name: String,
    pub room: String,
}

impl Component for InputBox {
    type Message = Msg;
    type Properties = InputBoxProps;

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
                let message = MessageDTO {
                    room: ctx.props().room.clone(),
                    user_id: ctx.props().user_id,
                    user_name: ctx.props().user_name.clone(),
                    message: content,
                };
                spawn_local(async move {
                    apiclient::send(message).await.unwrap();
                });
                true
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
