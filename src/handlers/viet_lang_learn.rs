use crate::tasks::viet_lang_learn_poller::{get_current_viet_data, initialize_cache};
use actix_web::{HttpResponse, Responder};

pub async fn viet_lang_learn_handler() -> impl Responder {
    // Get cached data from the poller
    match get_current_viet_data().await {
        Some(data) => {
            // Return the cached JSON data
            HttpResponse::Ok().json(data)
        }
        None => {
            // Cache is empty, try to initialize it
            if let Ok(()) = initialize_cache().await {
                // Try again after initialization
                match get_current_viet_data().await {
                    Some(data) => HttpResponse::Ok().json(data),
                    None => HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to load Vietnamese language data"
                    })),
                }
            } else {
                // Initialization failed
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Vietnamese language data not available"
                }))
            }
        }
    }
}
