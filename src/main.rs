use actix_web::{web, App, HttpServer, Responder};
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/viet-lang-learn", web::post().to(handlers::viet_lang_learn::viet_lang_learn_handler))
            .route("/BART", web::post().to(handlers::bart::bart_handler))
            .route("/MBTA", web::post().to(handlers::mbta::mbta_handler))
            .route("/check-in", web::post().to(handlers::check_in::check_in_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}