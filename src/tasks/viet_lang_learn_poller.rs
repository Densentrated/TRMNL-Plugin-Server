use tokio::time::{interval, Duration};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use include_dir::{include_dir, Dir};

// Embed the storage directory at compile time
static STORAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/storage");

#[derive(Debug, Deserialize, Clone)]
struct CsvRow {
    #[serde(rename = "Vietnamese")]
    vietnamese: String,
    #[serde(rename = "English")]
    english: String,
    #[serde(rename = "Vietnamese_Sentence")]
    vietnamese_sentence: String,
    #[serde(rename = "English_Sentence")]
    english_sentence: String,
}

#[derive(Serialize, Clone)]
pub struct VietLangResponse {
    pub word: String,
    #[serde(rename = "word-translated")]
    pub word_translated: String,
    pub sentence: String,
    #[serde(rename = "sentence-translated")]
    pub sentence_translated: String,
}

// Global cache for the current Vietnamese language data
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

    println!("Updating Vietnamese language cache for day {}, CSV row {}", year_day, csv_number);

    // Read from embedded CSV data
    let response = read_embedded_csv_row(csv_number as usize).await?;

    // Update the global cache
    {
        let mut cache = CURRENT_VIET_DATA.write().unwrap();
        *cache = Some(response.clone());
    }

    println!("Vietnamese language cache updated successfully");
    Ok(())
}

async fn read_embedded_csv_row(row_number: usize) -> Result<VietLangResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Get embedded CSV file content
    let csv_file = STORAGE_DIR.get_file("viet_lang_learn.csv")
        .ok_or("CSV file not found in embedded storage")?;
    
    let content = csv_file.contents_utf8()
        .ok_or("CSV file is not valid UTF-8")?;
    
    // Parse CSV in a blocking task to avoid blocking the async runtime
    let response = tokio::task::spawn_blocking(move || -> Result<VietLangResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        let mut records: Vec<CsvRow> = Vec::new();
        
        // Parse all records
        for result in reader.deserialize() {
            let record: CsvRow = result?;
            records.push(record);
        }
        
        if records.is_empty() {
            return Err("No data found in CSV file".into());
        }
        
        // Get the specific row (with wraparound if row_number exceeds available rows)
        let index = (row_number - 1) % records.len();
        let row = &records[index];
        
        Ok(VietLangResponse {
            word: row.vietnamese.clone(),
            word_translated: row.english.clone(),
            sentence: row.vietnamese_sentence.clone(),
            sentence_translated: row.english_sentence.clone(),
        })
    }).await??;
    
    Ok(response)
}

// Helper function to get current cached data
pub fn get_current_viet_data() -> Option<VietLangResponse> {
    let cache = CURRENT_VIET_DATA.read().unwrap();
    cache.clone()
}

// Initialize cache immediately (useful for tests and startup)
pub async fn initialize_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    update_viet_lang_cache().await
}