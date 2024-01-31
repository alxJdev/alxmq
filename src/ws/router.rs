use actix_web::web;
use crate::AppState;
use crate::ws::index::index_handler;
use crate::ws::retrieve_from_main_que::retrieve_from_main_que_handler;
use crate::ws::retrieve_from_response_que::retrieve_from_response_que_handler;
use crate::ws::to_main_que::to_main_que_handler;
use crate::ws::to_response_que::to_response_que_handler;
use crate::ws::types::RouterMessage;

pub fn ws_router(app_data: web::Data<AppState>, msg: String) -> String {
    let request_object_result = serde_json::from_str(msg.as_str());
    if request_object_result.is_err() {
        return "fuck".to_string();
    }
    let request_object: RouterMessage = request_object_result.unwrap();
    return match request_object.route {
        1 => retrieve_from_main_que_handler(app_data, msg),
        2 => retrieve_from_response_que_handler(app_data, msg),
        3 => to_main_que_handler(app_data, msg),
        4 => to_response_que_handler(app_data, msg),
        _ => index_handler(),
    }
}