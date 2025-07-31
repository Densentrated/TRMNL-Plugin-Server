use crate::utils::csv_reader;
use chrono::{Datelike, Utc};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};

#[derive(Serialize, Clone)]
pub struct VietLangResponse {
    pub word: String,
    #[serde(rename = "word-translated")]
    pub word_translated: String,
    pub sentence: String,
    #[serde(rename = "sentence-translated")]
    pub sentence_translated: String,
}

// Global cache for the current Vietnamese language data, using tokio's RwLock
lazy_static::lazy_static! {
    pub static ref CURRENT_VIET_DATA: Arc<RwLock<Option<VietLangResponse>>> = Arc::new(RwLock::new(None));
}

pub async fn run_daily_poller() {
    let mut interval = interval(Duration::from_secs(24 * 60 * 60)); // 24 hours

    // Load initial data immediately
    if let Err(e) = update_viet_lang_cache().await {
        eprintln!("Error loading initial Vietnamese data: {}", e);
    }

    loop {
        interval.tick().await;

        // Update cache with current day's data
        if let Err(e) = update_viet_lang_cache().await {
            eprintln!("Error during daily poll: {}", e);
        }
    }
}

async fn update_viet_lang_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get current date and calculate CSV row number
    let now = Utc::now();
    let year_day = now.ordinal(); // Day of year (1-365/366)
    let csv_number = ((year_day - 1) % 1000) + 1;

    println!(
        "Updating Vietnamese language cache for day {}, CSV row {}",
        year_day, csv_number
    );

    // Read from embedded CSV data using the generic util function
    let csv_row =
        csv_reader::read_embedded_csv_row("viet_lang_learn.csv", csv_number as usize).await?;

    // Convert HashMap to VietLangResponse
    let response = VietLangResponse {
        word: csv_row.get("Vietnamese").unwrap_or(&String::new()).clone(),
        word_translated: csv_row.get("English").unwrap_or(&String::new()).clone(),
        sentence: csv_row
            .get("Vietnamese_Sentence")
            .unwrap_or(&String::new())
            .clone(),
        sentence_translated: csv_row
            .get("English_Sentence")
            .unwrap_or(&String::new())
            .clone(),
    };

    // Update the global cache
    {
        let mut cache = CURRENT_VIET_DATA.write().await;
        *cache = Some(response.clone());
    }

    println!("Vietnamese language cache updated successfully");
    Ok(())
}

// Helper function to get current cached data
pub async fn get_current_viet_data() -> Option<VietLangResponse> {
    let cache = CURRENT_VIET_DATA.read().await;
    cache.clone()
}

// Initialize cache immediately (useful for tests and startup)
pub async fn initialize_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    update_viet_lang_cache().await
}
