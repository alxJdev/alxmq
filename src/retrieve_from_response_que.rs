use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use actix_web::web::Json;
use crate::AppState;
use crate::types::{RetrieveFromResponseQueRequest, RetrieveFromResponseQueResponse};

#[post("/retrieve_from_response_que")]
pub async fn retrieve_from_response_que_handler(_req: HttpRequest, _req_data: Json<RetrieveFromResponseQueRequest>, app_data: web::Data<AppState>) -> impl Responder {
    let que = match _req_data.que_id {
        1 => app_data.que_1.try_lock(),
        _ => app_data.que_0.try_lock()
    };

    let mut success = false;
    let mut message = ("".to_string(), "".to_string());

    if que.is_ok() {
        let message_result = que.unwrap().return_que.retrieve_from_map(_req_data.key.clone());
        if message_result.is_ok() {
            message = message_result.unwrap();
            success = true;
        }
    }

    let response = RetrieveFromResponseQueResponse {
        message: message.1,
        success
    };
    HttpResponse::Ok().json(response)
}