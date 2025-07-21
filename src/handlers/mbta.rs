use actix_web::{HttpResponse, Responder};

pub async fn mbta_handler() -> impl Responder {
    HttpResponse::Ok().body("MBTA Handler")
}
