use rocket::form::FromForm;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::Shutdown;
use rocket::{serde::Serialize, State};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    pub user_id: usize,
    #[field(validate = len(..20))]
    pub user_name: String,
    #[field(validate = len(..30))]
    pub message: String,
}

#[post("/post", format = "json", data = "<data>")]
pub fn post(data: Json<Message>, queue: &State<Sender<Message>>) {
    // Fails if there are no subscribers
    //let message = data
    let _res = queue.send(data.0);
}

#[get("/events")]
pub async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg).event("messages");
        }
    }
}
