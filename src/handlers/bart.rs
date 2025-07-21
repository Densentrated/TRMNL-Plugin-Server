use actix_web::{HttpResponse, Responder};

pub async fn bart_handler() -> impl Responder {
    HttpResponse::Ok().body("BART Handler")
}
