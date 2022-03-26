use crate::inputbox::InputBoxProps;
use crate::messages::MessagesProps;
use js_sys::Math::random;

//extern crate sse_client;
use self::messages::Messages;
use self::inputbox::InputBox;
use yew::prelude::*;

pub mod messages;
pub mod apiclient;
pub mod inputbox;

#[derive(Debug)]
struct Main {
    user_id: usize,
    user_name: String,
    room: String
}

#[derive(Debug)]
enum Msg {}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let user_id: usize = random().to_bits() as usize;
        log::info!("UserId: {}", user_id);
        Self {
            user_id,
            user_name: String::from(""),
            room: String::from("")
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let messages_props = MessagesProps {
            user_id: self.user_id.clone()
        };
        let input_box_props = InputBoxProps {
            user_id: self.user_id.clone(),
            user_name: self.user_name.clone(),
            room: self.room.clone(),
        };
        html! {
            <>
                <h1>{ "Chat" }</h1>
                <Messages ..messages_props />
                <InputBox ..input_box_props />
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}

