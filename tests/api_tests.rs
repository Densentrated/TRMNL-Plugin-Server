use actix_web::{test, App};
use TRMNL_Plugin_Server::handlers; // Adjust the module path as needed

#[actix_web::test]
async fn test_viet_language_learning_endpoint() {
    let app = test::init_service(
        App::new().route("/viet-language-learning", web::get().to(handlers::viet_lang_learn::viet_lang_learn_handler))
    ).await;

    let req = test::TestRequest::get().uri("/viet-language-learning").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["param1"].as_str().unwrap_or("").is_empty() == false);
    assert!(body["param2"].as_str().unwrap_or("").is_empty() == false);
}