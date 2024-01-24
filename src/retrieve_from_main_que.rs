use actix_web::{HttpRequest, HttpResponse, post, Responder, web};
use actix_web::web::Json;
use crate::AppState;
use crate::types::{QueObject, RetrieveFromMainQueRequest, RetrieveFromMainQueResponse};

#[post("/retrieve_from_main_que")]
pub async fn retrieve_from_main_que_handler(_req: HttpRequest, _req_data: Json<RetrieveFromMainQueRequest>, app_data: web::Data<AppState>) -> impl Responder {
    let que = match _req_data.que_id {
        1 => app_data.que_1.try_lock(),
        _ => app_data.que_0.try_lock()
    };

    let mut que_items: Vec<QueObject> = Vec::new();
    let mut success = false;

    if que.is_ok() {
        que_items = que.unwrap().main_que.retrieve_all_from_map();
        if que_items.len() > 0 {
            success = true;
        }
    }

    let response = RetrieveFromMainQueResponse {
        que_items,
        success
    };
    HttpResponse::Ok().json(response)
}