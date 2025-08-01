use actix_web::{web, App, HttpServer};
mod handlers;
mod tasks;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the daily poller in the background
    tokio::spawn(tasks::viet_lang_learn_poller::run_daily_poller());
    
    // start the server
    HttpServer::new(|| {
        App::new()
            .route("/viet-lang-learn", web::get().to(handlers::viet_lang_learn::viet_lang_learn_handler))
            .route("/BART", web::post().to(handlers::bart::bart_handler))
            .route("/MBTA", web::post().to(handlers::mbta::mbta_handler))
            .route("/check-in", web::post().to(handlers::check_in::check_in_handler))
    })
    .bind("0.0.0.0:22991")?
    .run()
    .await
}