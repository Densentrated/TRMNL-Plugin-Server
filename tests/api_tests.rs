use actix_web::{test, web, App};
use trmnl_plugin_server::handlers; // Adjust the module path as needed
use serde_json::Value;

#[actix_web::test]
async fn test_always_passes() {
    assert_eq!(1 + 1, 2);
}

// viet_lang_learn handler tests
#[actix_web::test]
async fn test_viet_lang_learn_handler() {
    let app = test::init_service(
        App::new()
            .route("/viet-lang-learn", web::get().to(handlers::viet_lang_learn::viet_lang_learn_handler))
    ).await;

    let req = test::TestRequest::get()
        .uri("/viet-lang-learn")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Check that the response is successful
    assert!(resp.status().is_success(), "Response status should be successful");
    
    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Check that all four required parameters exist and are non-empty
    assert!(json.get("word").is_some(), "Response should contain 'word' field");
    assert!(json.get("word-translated").is_some(), "Response should contain 'word-translated' field");
    assert!(json.get("sentence").is_some(), "Response should contain 'sentence' field");
    assert!(json.get("sentence-translated").is_some(), "Response should contain 'sentence-translated' field");
    
    // Check that the values are strings and non-empty
    let word = json["word"].as_str().expect("word should be a string");
    let word_translated = json["word-translated"].as_str().expect("word-translated should be a string");
    let sentence = json["sentence"].as_str().expect("sentence should be a string");
    let sentence_translated = json["sentence-translated"].as_str().expect("sentence-translated should be a string");
    
    assert!(!word.is_empty(), "word should not be empty");
    assert!(!word_translated.is_empty(), "word-translated should not be empty");
    assert!(!sentence.is_empty(), "sentence should not be empty");
    assert!(!sentence_translated.is_empty(), "sentence-translated should not be empty");
}

// check-in handler test
// inputs a json body with a country, city, and coordinates
// outputs a json body with weather temp, weather description, time at that location
#[actix_web::test]
async fn test_check_in_handler() {
    let app = test::init_service(
        App::new()
            .route("/check-in", web::post().to(handlers::check_in::check_in_handler))
    ).await;

    let request_body = serde_json::json!({
        "country": "United States",
        "city": "San Francisco",
        "coordinates": {
            "lat": 37.7749,
            "lon": -122.4194
        }
    });

    let req = test::TestRequest::post()
        .uri("/check-in")
        .set_json(&request_body)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Check that the response is successful
    assert!(resp.status().is_success(), "Response status should be successful");
    
    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Check that all required fields exist
    assert!(json.get("weather_temp").is_some(), "Response should contain 'weather_temp' field");
    assert!(json.get("weather_description").is_some(), "Response should contain 'weather_description' field");
    assert!(json.get("time").is_some(), "Response should contain 'time' field");
    
    // Check that the values are the correct types and non-empty
    let weather_temp = json["weather_temp"].as_f64().expect("weather_temp should be a number");
    let weather_description = json["weather_description"].as_str().expect("weather_description should be a string");
    let time = json["time"].as_str().expect("time should be a string");
    
    assert!(!weather_description.is_empty(), "weather_description should not be empty");
    assert!(!time.is_empty(), "time should not be empty");
    // weather_temp can be any number (positive, negative, or zero)
}

// BART handler test
// input a json body with a station name, 
// output a json body with the four next train times and one train time that passed
#[actix_web::test]
async fn test_bart_handler() {
    let app = test::init_service(
        App::new()
            .route("/BART", web::post().to(handlers::bart::bart_handler))
    ).await;

    let request_body = serde_json::json!({
        "station_name": "Powell St"
    });

    let req = test::TestRequest::post()
        .uri("/BART")
        .set_json(&request_body)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Check that the response is successful
    assert!(resp.status().is_success(), "Response status should be successful");
    
    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Check that required fields exist
    assert!(json.get("next_trains").is_some(), "Response should contain 'next_trains' field");
    assert!(json.get("passed_train").is_some(), "Response should contain 'passed_train' field");
    
    // Check that next_trains is an array with 4 elements
    let next_trains = json["next_trains"].as_array().expect("next_trains should be an array");
    assert_eq!(next_trains.len(), 4, "next_trains should contain exactly 4 train times");
    
    // Check that all train times are non-empty strings
    for (i, train_time) in next_trains.iter().enumerate() {
        let time_str = train_time.as_str().expect(&format!("Train time {} should be a string", i));
        assert!(!time_str.is_empty(), "Train time {} should not be empty", i);
    }
    
    // Check that passed_train is a non-empty string
    let passed_train = json["passed_train"].as_str().expect("passed_train should be a string");
    assert!(!passed_train.is_empty(), "passed_train should not be empty");
}

// MBTA handler test
// input a json body with a station name
// output a json body with the four next train times, and one train time that passed
#[actix_web::test]
async fn test_mbta_handler() {
    let app = test::init_service(
        App::new()
            .route("/MBTA", web::post().to(handlers::mbta::mbta_handler))
    ).await;

    let request_body = serde_json::json!({
        "station_name": "South Station"
    });

    let req = test::TestRequest::post()
        .uri("/MBTA")
        .set_json(&request_body)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Check that the response is successful
    assert!(resp.status().is_success(), "Response status should be successful");
    
    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Check that required fields exist
    assert!(json.get("next_trains").is_some(), "Response should contain 'next_trains' field");
    assert!(json.get("passed_train").is_some(), "Response should contain 'passed_train' field");
    
    // Check that next_trains is an array with 4 elements
    let next_trains = json["next_trains"].as_array().expect("next_trains should be an array");
    assert_eq!(next_trains.len(), 4, "next_trains should contain exactly 4 train times");
    
    // Check that all train times are non-empty strings
    for (i, train_time) in next_trains.iter().enumerate() {
        let time_str = train_time.as_str().expect(&format!("Train time {} should be a string", i));
        assert!(!time_str.is_empty(), "Train time {} should not be empty", i);
    }
    
    // Check that passed_train is a non-empty string
    let passed_train = json["passed_train"].as_str().expect("passed_train should be a string");
    assert!(!passed_train.is_empty(), "passed_train should not be empty");
}