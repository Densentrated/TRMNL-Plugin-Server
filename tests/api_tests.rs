use actix_web::{App, test, web};
use regex;
use serde_json::Value;
use std::time::Duration;
use tokio;
use trmnl_plugin_server::handlers; // Adjust the module path as needed

#[actix_web::test]
async fn test_always_passes() {
    assert_eq!(1 + 1, 2);
}

// viet_lang_learn handler tests
#[actix_web::test]
async fn test_viet_lang_learn_handler() {
    let app = test::init_service(App::new().route(
        "/viet-lang-learn",
        web::get().to(handlers::viet_lang_learn::viet_lang_learn_handler),
    ))
    .await;

    let req = test::TestRequest::get()
        .uri("/viet-lang-learn")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check that the response is successful
    assert!(
        resp.status().is_success(),
        "Response status should be successful"
    );

    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // Check that all four required parameters exist and are non-empty
    assert!(
        json.get("word").is_some(),
        "Response should contain 'word' field"
    );
    assert!(
        json.get("word-translated").is_some(),
        "Response should contain 'word-translated' field"
    );
    assert!(
        json.get("sentence").is_some(),
        "Response should contain 'sentence' field"
    );
    assert!(
        json.get("sentence-translated").is_some(),
        "Response should contain 'sentence-translated' field"
    );

    // Check that the values are strings and non-empty
    let word = json["word"].as_str().expect("word should be a string");
    let word_translated = json["word-translated"]
        .as_str()
        .expect("word-translated should be a string");
    let sentence = json["sentence"]
        .as_str()
        .expect("sentence should be a string");
    let sentence_translated = json["sentence-translated"]
        .as_str()
        .expect("sentence-translated should be a string");

    assert!(!word.is_empty(), "word should not be empty");
    assert!(
        !word_translated.is_empty(),
        "word-translated should not be empty"
    );
    assert!(!sentence.is_empty(), "sentence should not be empty");
    assert!(
        !sentence_translated.is_empty(),
        "sentence-translated should not be empty"
    );
}

// check-in handler test
// inputs a json body with a country, city, and coordinates
// outputs a json body with weather temp, weather description, time at that location
#[ignore]
#[actix_web::test]
async fn test_check_in_handler() {
    let app = test::init_service(App::new().route(
        "/check-in",
        web::post().to(handlers::check_in::check_in_handler),
    ))
    .await;

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
    assert!(
        resp.status().is_success(),
        "Response status should be successful"
    );

    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // Check that all required fields exist
    assert!(
        json.get("weather_temp").is_some(),
        "Response should contain 'weather_temp' field"
    );
    assert!(
        json.get("weather_description").is_some(),
        "Response should contain 'weather_description' field"
    );
    assert!(
        json.get("time").is_some(),
        "Response should contain 'time' field"
    );

    // Check that the values are the correct types and non-empty
    let weather_temp = json["weather_temp"]
        .as_f64()
        .expect("weather_temp should be a number");
    let weather_description = json["weather_description"]
        .as_str()
        .expect("weather_description should be a string");
    let time = json["time"].as_str().expect("time should be a string");

    assert!(
        !weather_description.is_empty(),
        "weather_description should not be empty"
    );
    assert!(!time.is_empty(), "time should not be empty");
    // weather_temp can be any number (positive, negative, or zero)
}

// BART handler test following the specification
// Input: JSON with station_name, line_name, direction (boolean), actual_times (boolean)
// Output: JSON with train_0_departure_time, train_1-3_arrival_time, next_station
#[ignore]
#[actix_web::test]
async fn test_bart_handler_basic() {
    let app =
        test::init_service(App::new().route("/BART", web::post().to(handlers::bart::bart_handler)))
            .await;

    let request_body = serde_json::json!({
        "station_name": "Walnut Creek",
        "line_name": "Yellow",
        "direction": true,
        "actual_times": false
    });

    let req = test::TestRequest::post()
        .uri("/BART")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check that the response is successful
    assert!(
        resp.status().is_success(),
        "Response status should be successful"
    );

    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // Check that all required fields exist
    assert!(
        json.get("train_0_departure_time").is_some(),
        "Response should contain 'train_0_departure_time' field"
    );
    assert!(
        json.get("train_1_arrival_time").is_some(),
        "Response should contain 'train_1_arrival_time' field"
    );
    assert!(
        json.get("train_2_arrival_time").is_some(),
        "Response should contain 'train_2_arrival_time' field"
    );
    assert!(
        json.get("train_3_arrival_time").is_some(),
        "Response should contain 'train_3_arrival_time' field"
    );
    assert!(
        json.get("next_station").is_some(),
        "Response should contain 'next_station' field"
    );

    // Validate regex pattern for time fields (e.g., "4 minutes ago", "24 minutes ago")
    let time_pattern = regex::Regex::new(r"^\d+ minutes ago$").unwrap();

    let train_0_time = json["train_0_departure_time"]
        .as_str()
        .expect("train_0_departure_time should be a string");
    assert!(
        time_pattern.is_match(train_0_time),
        "train_0_departure_time should match pattern 'X minutes ago', got: '{}'",
        train_0_time
    );

    let train_1_time = json["train_1_arrival_time"]
        .as_str()
        .expect("train_1_arrival_time should be a string");
    assert!(
        time_pattern.is_match(train_1_time),
        "train_1_arrival_time should match pattern 'X minutes ago', got: '{}'",
        train_1_time
    );

    let train_2_time = json["train_2_arrival_time"]
        .as_str()
        .expect("train_2_arrival_time should be a string");
    assert!(
        time_pattern.is_match(train_2_time),
        "train_2_arrival_time should match pattern 'X minutes ago', got: '{}'",
        train_2_time
    );

    let train_3_time = json["train_3_arrival_time"]
        .as_str()
        .expect("train_3_arrival_time should be a string");
    assert!(
        time_pattern.is_match(train_3_time),
        "train_3_arrival_time should match pattern 'X minutes ago', got: '{}'",
        train_3_time
    );

    // Check that next_station is not empty
    let next_station = json["next_station"]
        .as_str()
        .expect("next_station should be a string");
    assert!(!next_station.is_empty(), "next_station should not be empty");
}

// Test with actual_times = true (should return actual time format)
#[ignore]
#[actix_web::test]
async fn test_bart_handler_actual_times() {
    let app =
        test::init_service(App::new().route("/BART", web::post().to(handlers::bart::bart_handler)))
            .await;

    let request_body = serde_json::json!({
        "station_name": "Powell St",
        "line_name": "Blue",
        "direction": false,
        "actual_times": true
    });

    let req = test::TestRequest::post()
        .uri("/BART")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check that the response is successful
    assert!(
        resp.status().is_success(),
        "Response status should be successful"
    );

    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // When actual_times is true, times should be in actual time format (e.g., "2:45 PM")
    let actual_time_pattern = regex::Regex::new(r"^\d{1,2}:\d{2}\s(AM|PM)$").unwrap();

    let train_0_time = json["train_0_departure_time"]
        .as_str()
        .expect("train_0_departure_time should be a string");
    assert!(
        actual_time_pattern.is_match(train_0_time) || train_0_time.ends_with(" minutes ago"),
        "train_0_departure_time should match actual time pattern or minutes ago, got: '{}'",
        train_0_time
    );

    // Check that next_station is not empty
    let next_station = json["next_station"]
        .as_str()
        .expect("next_station should be a string");
    assert!(!next_station.is_empty(), "next_station should not be empty");
}

// Test with different direction values
#[ignore]
#[actix_web::test]
async fn test_bart_handler_direction_false() {
    let app =
        test::init_service(App::new().route("/BART", web::post().to(handlers::bart::bart_handler)))
            .await;

    let request_body = serde_json::json!({
        "station_name": "Embarcadero",
        "line_name": "Red",
        "direction": false,
        "actual_times": false
    });

    let req = test::TestRequest::post()
        .uri("/BART")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check that the response is successful
    assert!(
        resp.status().is_success(),
        "Response status should be successful"
    );

    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // All fields should exist
    assert!(json.get("train_0_departure_time").is_some());
    assert!(json.get("train_1_arrival_time").is_some());
    assert!(json.get("train_2_arrival_time").is_some());
    assert!(json.get("train_3_arrival_time").is_some());
    assert!(json.get("next_station").is_some());
}

// Test with invalid request body
#[ignore]
#[actix_web::test]
async fn test_bart_handler_invalid_request() {
    let app =
        test::init_service(App::new().route("/BART", web::post().to(handlers::bart::bart_handler)))
            .await;

    // Missing required fields
    let request_body = serde_json::json!({
        "station_name": "Powell St"
    });

    let req = test::TestRequest::post()
        .uri("/BART")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should return a 400 Bad Request for invalid input
    assert!(
        resp.status().is_client_error(),
        "Response should be a client error for invalid request"
    );
}

// Test response format consistency
#[ignore]
#[actix_web::test]
async fn test_bart_handler_response_format() {
    let app =
        test::init_service(App::new().route("/BART", web::post().to(handlers::bart::bart_handler)))
            .await;

    let request_body = serde_json::json!({
        "station_name": "Montgomery St",
        "line_name": "Green",
        "direction": true,
        "actual_times": false
    });

    let req = test::TestRequest::post()
        .uri("/BART")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // Validate exact field names as per specification
    let expected_fields = [
        "train_0_departure_time",
        "train_1_arrival_time",
        "train_2_arrival_time",
        "train_3_arrival_time",
        "next_station",
    ];

    for field in expected_fields.iter() {
        assert!(
            json.get(field).is_some(),
            "Response should contain '{}' field",
            field
        );

        let value = json[field]
            .as_str()
            .expect(&format!("{} should be a string", field));
        assert!(!value.is_empty(), "{} should not be empty", field);
    }

    // Ensure no extra fields are present
    assert_eq!(
        json.as_object().unwrap().len(),
        5,
        "Response should contain exactly 5 fields"
    );
}

// MBTA handler test
// input a json body with a station name
// output a json body with the four next train times, and one train time that passed
#[ignore]
#[actix_web::test]
async fn test_mbta_handler() {
    let app =
        test::init_service(App::new().route("/MBTA", web::post().to(handlers::mbta::mbta_handler)))
            .await;

    let request_body = serde_json::json!({
        "station_name": "South Station"
    });

    let req = test::TestRequest::post()
        .uri("/MBTA")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Check that the response is successful
    assert!(
        resp.status().is_success(),
        "Response status should be successful"
    );

    // Get the response body and parse as JSON
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).expect("Response should be valid JSON");

    // Check that required fields exist
    assert!(
        json.get("next_trains").is_some(),
        "Response should contain 'next_trains' field"
    );
    assert!(
        json.get("passed_train").is_some(),
        "Response should contain 'passed_train' field"
    );

    // Check that next_trains is an array with 4 elements
    let next_trains = json["next_trains"]
        .as_array()
        .expect("next_trains should be an array");
    assert_eq!(
        next_trains.len(),
        4,
        "next_trains should contain exactly 4 train times"
    );

    // Check that all train times are non-empty strings
    for (i, train_time) in next_trains.iter().enumerate() {
        let time_str = train_time
            .as_str()
            .expect(&format!("Train time {} should be a string", i));
        assert!(!time_str.is_empty(), "Train time {} should not be empty", i);
    }

    // Check that passed_train is a non-empty string
    let passed_train = json["passed_train"]
        .as_str()
        .expect("passed_train should be a string");
    assert!(!passed_train.is_empty(), "passed_train should not be empty");
}
