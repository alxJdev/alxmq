pub mod types;
mod retrieve_from_main_que;
mod router;
mod index;
mod retrieve_from_response_que;
mod to_main_que;
mod to_response_que;

use actix_web::{Error, get, HttpRequest, HttpResponse, web};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use crate::AppState;
use crate::ws::router::ws_router;

struct WsObject {
    app_state: web::Data<AppState>,
}
impl Actor for WsObject {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for WsObject {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Text(msg)) => ctx.text(ws_router(self.app_state.clone(), msg.to_string())),
            _ => (),
        }
    }
}

#[get("/ws")]
pub async fn ws_handler(_req: HttpRequest, stream: web::Payload, app_data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let response = ws::start(WsObject{app_state: app_data}, &_req, stream);
    response
}