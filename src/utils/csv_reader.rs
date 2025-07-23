use include_dir::{include_dir, Dir};
use std::collections::HashMap;

// Embed the storage directory at compile time
static STORAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/storage");

pub async fn read_embedded_csv_row(filename: &str, row_number: usize) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
    // Get embedded CSV file content
    let csv_file = STORAGE_DIR.get_file(filename)
        .ok_or("CSV file not found in embedded storage")?;
    
    let content = csv_file.contents_utf8()
        .ok_or("CSV file is not valid UTF-8")?;
    
    // Parse CSV in a blocking task to avoid blocking the async runtime
    let response = tokio::task::spawn_blocking(move || -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        let mut records: Vec<HashMap<String, String>> = Vec::new();
        
        // Parse all records as string maps
        for result in reader.deserialize() {
            let record: HashMap<String, String> = result?;
            records.push(record);
        }
        
        if records.is_empty() {
            return Err("No data found in CSV file".into());
        }
        
        // Get the specific row (with wraparound if row_number exceeds available rows)
        let index = (row_number - 1) % records.len();
        let row = &records[index];
        
        Ok(row.clone())
    }).await??;
    
    Ok(response)
}
