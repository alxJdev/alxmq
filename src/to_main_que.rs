use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use web::Json;
use crate::AppState;
use crate::types::{ToMainQueRequest, ToMainQueResponse};

#[post("/to_main_que")]
pub async fn to_main_que_handler(_req: HttpRequest, _req_data: Json<ToMainQueRequest>, app_data: web::Data<AppState>) -> impl Responder {
    let que = match _req_data.que_id {
        1 => app_data.que_1.try_lock(),
        _ => app_data.que_0.try_lock()
    };

    let mut que_item_key = "".to_string();
    let mut success = false;

    if que.is_ok() {
        que_item_key = que.unwrap().main_que.add_to_map_and_generate_key(_req_data.message.clone());
        success = true;
    }

    let response = ToMainQueResponse {
        que_item_key,
        success
    };
    HttpResponse::Ok().json(response)
}