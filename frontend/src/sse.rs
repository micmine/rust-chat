use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventSource, MessageEvent};
use yew::prelude::*;

pub struct EventSourceTask {
    pub event_source: EventSource,
    cb: Closure<dyn FnMut(MessageEvent) -> ()>,
}

impl EventSourceTask {
    fn is_active(&self) -> bool {
        self.event_source.ready_state() == EventSource::OPEN
    }
}

pub struct EventSourceService {}

impl EventSourceService {
    pub fn new() -> Self {
        EventSourceService {}
    }

    pub async fn connect(url: &str, callback: Callback<String>) -> EventSourceTask {
        log::debug!("Connect to {}", &url);
        let event_source = EventSource::new(url).unwrap();
        let cb = Closure::wrap(Box::new(move |event: MessageEvent| {
            log::info!("Got message");
            let text = event.data().as_string();
            match text {
                Some(text) => {
                    let content = text.to_owned().clone();
                    callback.emit(content);
                }
                None => {
                    log::error!("Unable to parse incoming messsage as string");
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        event_source.set_onmessage(Some(cb.as_ref().unchecked_ref()));

        EventSourceTask { event_source, cb }
    }
}

//impl Task for EventSourceTask {
//fn is_active(&self) -> bool {
//self.event_source.ready_state() == EventSource::OPEN
//}
//}

impl Drop for EventSourceTask {
    fn drop(&mut self) {
        self.event_source.close();
    }
}
