use actix_web::web;
use crate::AppState;
use crate::types::{RetrieveFromMainQueRequest, RetrieveFromMainQueResponse};

pub fn retrieve_from_main_que_handler(app_data: web::Data<AppState>, msg: String) -> String {
    let request_result = serde_json::from_str(msg.as_str());
    let mut response = RetrieveFromMainQueResponse {
        que_items: vec![],
        success: false,
    };
    if request_result.is_ok() {
        let request: RetrieveFromMainQueRequest = request_result.unwrap();
        let que = match request.que_id {
            1 => app_data.que_1.try_lock(),
            _ => app_data.que_0.try_lock()
        };
        if que.is_ok() {
            response.que_items = que.unwrap().main_que.retrieve_all_from_map();
            if response.que_items.len() > 0 {
                response.success = true;
            }
        }
    }

    let return_result = serde_json::to_string(&response);
    if return_result.is_ok() {
        return  return_result.unwrap();
    }
    else {
        panic!();
    }
}