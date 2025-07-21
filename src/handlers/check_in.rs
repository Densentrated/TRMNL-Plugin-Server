use actix_web::{HttpResponse, Responder};

pub async fn check_in_handler() -> impl Responder {
    HttpResponse::Ok().body("Check-In Handler")
}
