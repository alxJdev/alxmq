use actix_web::web;
use crate::AppState;
use crate::types::{ToResponseQueRequest, ToResponseQueResponse};

pub fn to_response_que_handler(app_data: web::Data<AppState>, msg: String) -> String {
    let request_result = serde_json::from_str(msg.as_str());
    let mut response = ToResponseQueResponse {
        success: false,
    };

    if request_result.is_ok() {
        let request: ToResponseQueRequest = request_result.unwrap();
        let que = match request.que_id {
            1 => app_data.que_1.try_lock(),
            _ => app_data.que_0.try_lock()
        };
        if que.is_ok() {
            que.unwrap().return_que.add_to_map(request.key.clone(), request.message.clone());
            response.success = true;
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