use actix_web::{HttpResponse, Responder};

pub async fn viet_lang_learn_handler() -> impl Responder {
    HttpResponse::Ok().body("Viet Lang Learn Handler")
}
