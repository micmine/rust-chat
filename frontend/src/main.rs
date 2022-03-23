//extern crate sse_client;
use self::messages::Messages;
use self::inputbox::InputBox;
use yew::prelude::*;

pub mod messages;
pub mod apiclient;
pub mod inputbox;
pub mod sse;

#[derive(Debug)]
struct Main;

#[derive(Debug)]
enum Msg {}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Chat" }</h1>
                <Messages />
                <InputBox />
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}

