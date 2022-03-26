use gloo_events::EventListener;
use web_sys::EventSource;
use yew::prelude::*;

use crate::apiclient::MessageDTO;

#[derive(Debug)]
pub enum MessageType {
    Error,
    Own,
    Other
}

#[derive(Debug)]
pub struct Message {
    msg_type: MessageType,
    author: String,
    message: String,
}

impl Message {
    fn from(message: MessageDTO, local_user_id: usize) -> Self {
        let msg_type = match message.user_id == local_user_id {
            true => MessageType::Own,
            false => MessageType::Other,
        };

        Self {
            msg_type,
            message: message.message,
            author: message.user_name
        }
    }
}

#[derive(Debug)]
pub struct Messages {
    messages: Vec<Message>,
    es: EventSource,
    _listener: Option<EventListener>,
}

#[derive(Debug)]
pub enum Msg {
    NewMessage(Result<MessageDTO, serde_json::Error>),
}

#[derive(Properties, PartialEq)]
pub struct MessagesProps {
    pub user_id: usize
}

impl Component for Messages {
    type Message = Msg;
    type Properties = MessagesProps;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = ctx
            .link()
            .callback(|bufstr: String| Msg::NewMessage(serde_json::from_str(&bufstr)));

        let (es, listener) = crate::apiclient::load(cb);

        Self {
            messages: Vec::new(),
            es,
            _listener: Some(listener),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewMessage(messages) => {
                match messages {
                    Ok(message_dto) => {
                        log::info!("{:?}", message_dto);
                        self.messages.push(Message::from(message_dto, _ctx.props().user_id));
                    }
                    Err(_) => {
                        log::error!("Got invlaid message");
                    }
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
              <div class="chat-container chat-push">
                {
                    self.messages.iter().map(|message| {
                        let class = match &message.msg_type {
                            MessageType::Error => "",
                            MessageType::Own => "chat-message-right",
                            MessageType::Other => "chat-message-left",
                        };
                        html!{
                            <div class={classes!("chat-message", class)}>
                              <a href="">{ &message.author }</a>
                              <p>{ &message.message }</p>
                            </div>
                        }
                    }).collect::<Html>()
                }
              </div>
            </>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.es.close();
        self._listener = None;
    }
}
