use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use gtfs_realtime::FeedMessage;
use prost::Message;
use reqwest;
use std::collections::HashMap;
use chrono;

// Function to convert BART GTFS stop IDs to station names
fn get_station_name_from_gtfs_id(stop_id: &str) -> String {
    // BART GTFS stop IDs follow pattern: [LINE][NUMBER]-[PLATFORM]
    // A = Airport/Fremont line, L = Lafayette/Richmond line, M = Montgomery/Peninsula line
    // -1 = southbound/eastbound, -2 = northbound/westbound
    
    let gtfs_to_station: HashMap<&str, &str> = [
        // A line (Airport/Fremont)
        ("A10", "Embarcadero (SF)"),
        ("A20", "Montgomery St. (SF)"),
        ("A30", "Powell St. (SF)"),
        ("A40", "Civic Center (SF)"),
        ("A50", "24th St. Mission (SF)"),
        ("A60", "16th St. Mission (SF)"),
        ("A70", "Glen Park (SF)"),
        ("A80", "Balboa Park (SF)"),
        
        // L line (Lafayette/Richmond)  
        ("L10", "MacArthur (Oakland)"),
        ("L20", "19th St. Oakland"),
        ("L30", "12th St. Oakland City Center"),
        ("L40", "West Oakland"),
        
        // M line (Montgomery/Peninsula)
        ("M10", "Embarcadero (SF)"),
        ("M16", "Montgomery St. (SF)"),
        ("M20", "Powell St. (SF)"),
        ("M30", "Civic Center (SF)"),
        ("M40", "16th St. Mission (SF)"),
        ("M50", "24th St. Mission (SF)"),
        ("M60", "Glen Park (SF)"),
        ("M70", "Balboa Park (SF)"),
        ("M80", "Daly City"),
        
        // Additional common stations
        ("EMBR", "Embarcadero (SF)"),
        ("MONT", "Montgomery St. (SF)"),
        ("POWL", "Powell St. (SF)"),
        ("CIVC", "Civic Center (SF)"),
        ("16TH", "16th St. Mission (SF)"),
        ("24TH", "24th St. Mission (SF)"),
        ("GLEN", "Glen Park (SF)"),
        ("BALB", "Balboa Park (SF)"),
        ("DALY", "Daly City"),
        ("COLM", "Colma"),
        ("SSAN", "South San Francisco"),
        ("SBRN", "San Bruno"),
        ("SFIA", "San Francisco Int'l Airport"),
        ("MLBR", "Millbrae"),
        ("WOAK", "West Oakland"),
        ("12TH", "12th St. Oakland City Center"),
        ("19TH", "19th St. Oakland"),
        ("MCAR", "MacArthur (Oakland)"),
        ("ASHB", "Ashby (Berkeley)"),
        ("DBRK", "Downtown Berkeley"),
        ("NBRK", "North Berkeley"),
        ("PLZA", "El Cerrito Plaza"),
        ("DELN", "El Cerrito del Norte"),
        ("RICH", "Richmond"),
        ("ROCK", "Rockridge (Oakland)"),
        ("ORIN", "Orinda"),
        ("LAFY", "Lafayette"),
        ("WCRK", "Walnut Creek"),
        ("PHIL", "Pleasant Hill"),
        ("CONC", "Concord"),
        ("NCON", "North Concord/Martinez"),
        ("PITT", "Pittsburg/Bay Point"),
        ("PCTR", "Pittsburg Center"),
        ("ANTC", "Antioch"),
        ("LAKE", "Lake Merritt (Oakland)"),
        ("FTVL", "Fruitvale (Oakland)"),
        ("COLS", "Coliseum"),
        ("SANL", "San Leandro"),
        ("BAYF", "Bay Fair (San Leandro)"),
        ("HAYW", "Hayward"),
        ("SHAY", "South Hayward"),
        ("UCTY", "Union City"),
        ("FRMT", "Fremont"),
        ("WARM", "Warm Springs/South Fremont"),
        ("MLPT", "Milpitas"),
        ("BERY", "Berryessa / North San Jose"),
        ("CAST", "Castro Valley"),
        ("WDUB", "West Dublin"),
        ("DUBL", "Dublin/Pleasanton"),
        ("OAKL", "Oakland Int'l Airport"),
    ].iter().cloned().collect();

    // Extract the base station code (remove platform suffix)
    let base_code = if let Some(dash_pos) = stop_id.find('-') {
        &stop_id[..dash_pos]
    } else {
        stop_id
    };
    
    // Try to find the station name
    gtfs_to_station.get(base_code)
        .unwrap_or(&stop_id) // Return original ID if not found
        .to_string()
}

// expected body struct
#[derive(Serialize, Deserialize, Clone)]
pub struct BartIncomingRequest {
    pub direction: i8,
    pub station: String,
    pub line: String,
    pub show_time_not_minutes: bool,
}

#[derive(Serialize, Clone)]
pub struct BartOutgoingResponse {
    pub outbound_train: String,
    pub inbound_train_0: String,
    pub inbound_train_1: String,
    pub inbound_train_2: String,
}

pub async fn bart_handler(json_body: web::Json<Value>) -> impl Responder {
    // Store the JSON object in a variable
    let json_data = json_body.into_inner();
    
    let _incoming: BartIncomingRequest = match serde_json::from_value(json_data) {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid request body: {}", e)),
    };
    
    // get the real time information from the bart gtfs
    let bart_updates = match reqwest::get("https://api.bart.gov/gtfsrt/tripupdate.aspx").await {
        Ok(response) => response,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to fetch BART data: {}", e)),
    };
    
    let bytes = match bart_updates.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to read response: {}", e)),
    };
    
    // decode protobuf
    let bytes_decoded = match FeedMessage::decode(bytes.as_ref()) {
        Ok(decoded) => decoded,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to decode protobuf: {}", e)),
    };

    // print decoded protobuf to the console
    println!("Decoded BART GTFS-RT: {:?}", bytes_decoded);

    // Process the GTFS-RT data to extract station information
    let mut train_info = Vec::new();
    
    for entity in bytes_decoded.entity.iter() {
        if let Some(trip_update) = &entity.trip_update {
            for stop_time_update in trip_update.stop_time_update.iter() {
                if let Some(stop_id) = &stop_time_update.stop_id {
                    let station_name = get_station_name_from_gtfs_id(stop_id);
                    
                    // Get arrival time if available
                    let arrival_info = if let Some(arrival) = &stop_time_update.arrival {
                        if let Some(time) = arrival.time {
                            let dt = chrono::DateTime::from_timestamp(time, 0)
                                .unwrap_or_default();
                            format!("Arrives at {}", dt.format("%H:%M"))
                        } else {
                            "Time unknown".to_string()
                        }
                    } else {
                        "No arrival info".to_string()
                    };
                    
                    // Get delay info if available
                    let delay_info = if let Some(arrival) = &stop_time_update.arrival {
                        if let Some(delay) = arrival.delay {
                            if delay > 0 {
                                format!(" ({}s delay)", delay)
                            } else {
                                " (on time)".to_string()
                            }
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    };
                    
                    train_info.push(format!("{} - {}{}", station_name, arrival_info, delay_info));
                    
                    // Limit to avoid too much data
                    if train_info.len() >= 10 {
                        break;
                    }
                }
            }
            if train_info.len() >= 10 {
                break;
            }
        }
    }
    
    // Create response matching the expected structure
    let response = BartOutgoingResponse {
        outbound_train: train_info.get(0).unwrap_or(&"No data available".to_string()).clone(),
        inbound_train_0: train_info.get(1).unwrap_or(&"No data available".to_string()).clone(),
        inbound_train_1: train_info.get(2).unwrap_or(&"No data available".to_string()).clone(),
        inbound_train_2: train_info.get(3).unwrap_or(&"No data available".to_string()).clone(),
    };

    HttpResponse::Ok().json(response)
}

