use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use actix_web::web::Json;
use crate::AppState;
use crate::types::{ToResponseQueRequest, ToResponseQueResponse};

#[post("/to_response_que")]
pub async fn to_response_que_handler(_req: HttpRequest, _req_data: Json<ToResponseQueRequest>, app_data: web::Data<AppState>) -> impl Responder {
    let que = match _req_data.que_id {
        1 => app_data.que_1.try_lock(),
        _ => app_data.que_0.try_lock()
    };

    let mut success = false;

    if que.is_ok() {
        que.unwrap().return_que.add_to_map(_req_data.key.clone(), _req_data.message.clone());
        success = true;
    }

    let response = ToResponseQueResponse {
        success,
    };
    HttpResponse::Ok().json(response)
}