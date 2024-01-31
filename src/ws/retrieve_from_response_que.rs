use actix_web::web;
use crate::AppState;
use crate::types::{RetrieveFromResponseQueRequest, RetrieveFromResponseQueResponse};

pub fn retrieve_from_response_que_handler(app_data: web::Data<AppState>, msg: String) -> String {
    let request_result = serde_json::from_str(msg.as_str());
    let mut response = RetrieveFromResponseQueResponse{
        message: "".to_string(),
        success: false,
    };
    if request_result.is_ok() {
        let request: RetrieveFromResponseQueRequest = request_result.unwrap();
        let que = match request.que_id {
            1 => app_data.que_1.try_lock(),
            _ => app_data.que_0.try_lock()
        };
        if que.is_ok() {
            let message_result = que.unwrap().return_que.retrieve_from_map(request.key.clone());
            if message_result.is_ok() {
                response.message = message_result.unwrap().1;
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