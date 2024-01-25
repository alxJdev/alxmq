use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use crate::index::{index_handler};
use crate::to_main_que::{to_main_que_handler};
use crate::que::Que;
use crate::retrieve_from_main_que::retrieve_from_main_que_handler;
use crate::retrieve_from_response_que::retrieve_from_response_que_handler;
use crate::to_response_que::to_response_que_handler;

mod que;
mod index;
mod to_main_que;
pub mod types;
mod retrieve_from_main_que;
mod to_response_que;
mod retrieve_from_response_que;
mod error;
mod ws;

pub struct AppState {
    que_0: Mutex<Que>,
    que_1: Mutex<Que>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        que_0: Mutex::new(Que::new()),
        que_1: Mutex::new(Que::new()),
    });
    HttpServer::new( move || {
        App::new()
            .app_data(app_state.clone()
            )
            .service(index_handler)
            .service(to_main_que_handler)
            .service(retrieve_from_main_que_handler)
            .service(to_response_que_handler)
            .service(retrieve_from_response_que_handler)
    })
        .bind("127.0.0.1:42069")?
        .run().await
}