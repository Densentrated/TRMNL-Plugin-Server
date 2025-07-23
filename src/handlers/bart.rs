use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;

pub async fn bart_handler(json_body: web::Json<Value>) -> impl Responder {
    // Store the JSON object in a variable
    let _json_data = json_body.into_inner();
    
    // You can access it later like:
    // _json_data["station"], _json_data["direction"], etc.
    
    HttpResponse::Ok().body("BART Handler")
}

