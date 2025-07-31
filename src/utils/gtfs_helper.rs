use std::collections::HashMap;

pub fn get_gtfs_id_from_station_name_primary(station_name: &str) -> Option<String> {
    // Create mapping: station name -> GTFS ID
    // BART GTFS stop IDs follow pattern: [LINE][NUMBER]-[PLATFORM]
    // A = Airport/Fremont line, L = Lafayette/Richmond line, M = Montgomery/Peninsula line
    // -1 = southbound/eastbound, -2 = northbound/westbound
    
    let station_to_gtfs: HashMap<&str, &str> = [
        // A line (Airport/Fremont)
        ("Embarcadero (SF)", "A10"),
        ("Montgomery St. (SF)", "A20"),
        ("Powell St. (SF)", "A30"),
        ("Civic Center (SF)", "A40"),
        ("24th St. Mission (SF)", "A50"),
        ("16th St. Mission (SF)", "A60"),
        ("Glen Park (SF)", "A70"),
        ("Balboa Park (SF)", "A80"),
        
        // L line (Lafayette/Richmond)  
        ("MacArthur (Oakland)", "L10"),
        ("19th St. Oakland", "L20"),
        ("12th St. Oakland City Center", "L30"),
        ("West Oakland", "L40"),
        
        // M line (Montgomery/Peninsula)
        ("Embarcadero (SF)", "M10"),
        ("Montgomery St. (SF)", "M16"),
        ("Powell St. (SF)", "M20"),
        ("Civic Center (SF)", "M30"),
        ("16th St. Mission (SF)", "M40"),
        ("24th St. Mission (SF)", "M50"),
        ("Glen Park (SF)", "M60"),
        ("Balboa Park (SF)", "M70"),
        ("Daly City", "M80"),
        
        // Additional common stations
        ("Embarcadero (SF)", "EMBR"),
        ("Montgomery St. (SF)", "MONT"),
        ("Powell St. (SF)", "POWL"),
        ("Civic Center (SF)", "CIVC"),
        ("16th St. Mission (SF)", "16TH"),
        ("24th St. Mission (SF)", "24TH"),
        ("Glen Park (SF)", "GLEN"),
        ("Balboa Park (SF)", "BALB"),
        ("Daly City", "DALY"),
        ("Colma", "COLM"),
        ("South San Francisco", "SSAN"),
        ("San Bruno", "SBRN"),
        ("San Francisco Int'l Airport", "SFIA"),
        ("Millbrae", "MLBR"),
        ("West Oakland", "WOAK"),
        ("12th St. Oakland City Center", "12TH"),
        ("19th St. Oakland", "19TH"),
        ("MacArthur (Oakland)", "MCAR"),
        ("Ashby (Berkeley)", "ASHB"),
        ("Downtown Berkeley", "DBRK"),
        ("North Berkeley", "NBRK"),
        ("El Cerrito Plaza", "PLZA"),
        ("El Cerrito del Norte", "DELN"),
        ("Richmond", "RICH"),
        ("Rockridge (Oakland)", "ROCK"),
        ("Orinda", "ORIN"),
        ("Lafayette", "LAFY"),
        ("Walnut Creek", "WCRK"),
        ("Pleasant Hill", "PHIL"),
        ("Concord", "CONC"),
        ("North Concord/Martinez", "NCON"),
        ("Pittsburg/Bay Point", "PITT"),
        ("Pittsburg Center", "PCTR"),
        ("Antioch", "ANTC"),
        ("Lake Merritt (Oakland)", "LAKE"),
        ("Fruitvale (Oakland)", "FTVL"),
        ("Coliseum", "COLS"),
        ("San Leandro", "SANL"),
        ("Bay Fair (San Leandro)", "BAYF"),
        ("Hayward", "HAYW"),
        ("South Hayward", "SHAY"),
        ("Union City", "UCTY"),
        ("Fremont", "FRMT"),
        ("Warm Springs/South Fremont", "WARM"),
        ("Milpitas", "MLPT"),
        ("Berryessa / North San Jose", "BERY"),
        ("Castro Valley", "CAST"),
        ("West Dublin", "WDUB"),
        ("Dublin/Pleasanton", "DUBL"),
        ("Oakland Int'l Airport", "OAKL"),
    ].iter().cloned().collect();

    // Try exact match first
    if let Some(id) = station_to_gtfs.get(station_name) {
        return Some(id.to_string());
    }
    
    // Try case-insensitive match
    let lower_name = station_name.to_lowercase();
    for (name, id) in station_to_gtfs.iter() {
        if name.to_lowercase() == lower_name {
            return Some(id.to_string());
        }
    }
    
    // Try partial match (contains)
    for (name, id) in station_to_gtfs.iter() {
        if name.to_lowercase().contains(&lower_name) || lower_name.contains(&name.to_lowercase()) {
            return Some(id.to_string());
        }
    }
    
    None
}

pub fn get_gtfs_id_from_station_name(station_name: &str) -> Option<String> {
    // Create reverse mapping: station name -> GTFS ID
    let station_to_gtfs: HashMap<&str, &str> = [
        // A line (Airport/Fremont)
        ("Embarcadero (SF)", "A10"),
        ("Montgomery St. (SF)", "A20"),
        ("Powell St. (SF)", "A30"),
        ("Civic Center (SF)", "A40"),
        ("24th St. Mission (SF)", "A50"),
        ("16th St. Mission (SF)", "A60"),
        ("Glen Park (SF)", "A70"),
        ("Balboa Park (SF)", "A80"),
        
        // L line (Lafayette/Richmond)  
        ("MacArthur (Oakland)", "L10"),
        ("19th St. Oakland", "L20"),
        ("12th St. Oakland City Center", "L30"),
        ("West Oakland", "L40"),
        
        // M line (Montgomery/Peninsula) - prefer M line for SF stations
        ("Embarcadero", "M10"),
        ("Montgomery St.", "M16"),
        ("Powell St.", "M20"),
        ("Civic Center", "M30"),
        ("16th St. Mission", "M40"),
        ("24th St. Mission", "M50"),
        ("Glen Park", "M60"),
        ("Balboa Park", "M70"),
        ("Daly City", "M80"),
        
        // Additional stations
        ("Colma", "COLM"),
        ("South San Francisco", "SSAN"),
        ("San Bruno", "SBRN"),
        ("San Francisco Int'l Airport", "SFIA"),
        ("Millbrae", "MLBR"),
        ("Ashby (Berkeley)", "ASHB"),
        ("Downtown Berkeley", "DBRK"),
        ("North Berkeley", "NBRK"),
        ("El Cerrito Plaza", "PLZA"),
        ("El Cerrito del Norte", "DELN"),
        ("Richmond", "RICH"),
        ("Rockridge (Oakland)", "ROCK"),
        ("Orinda", "ORIN"),
        ("Lafayette", "LAFY"),
        ("Walnut Creek", "WCRK"),
        ("Pleasant Hill", "PHIL"),
        ("Concord", "CONC"),
        ("North Concord/Martinez", "NCON"),
        ("Pittsburg/Bay Point", "PITT"),
        ("Pittsburg Center", "PCTR"),
        ("Antioch", "ANTC"),
        ("Lake Merritt (Oakland)", "LAKE"),
        ("Fruitvale (Oakland)", "FTVL"),
        ("Coliseum", "COLS"),
        ("San Leandro", "SANL"),
        ("Bay Fair (San Leandro)", "BAYF"),
        ("Hayward", "HAYW"),
        ("South Hayward", "SHAY"),
        ("Union City", "UCTY"),
        ("Fremont", "FRMT"),
        ("Warm Springs/South Fremont", "WARM"),
        ("Milpitas", "MLPT"),
        ("Berryessa / North San Jose", "BERY"),
        ("Castro Valley", "CAST"),
        ("West Dublin", "WDUB"),
        ("Dublin/Pleasanton", "DUBL"),
        ("Oakland Int'l Airport", "OAKL"),
    ].iter().cloned().collect();
    
    // Try exact match first
    if let Some(id) = station_to_gtfs.get(station_name) {
        return Some(id.to_string());
    }
    
    // Try case-insensitive match
    let lower_name = station_name.to_lowercase();
    for (name, id) in station_to_gtfs.iter() {
        if name.to_lowercase() == lower_name {
            return Some(id.to_string());
        }
    }
    
    // Try partial match (contains)
    for (name, id) in station_to_gtfs.iter() {
        if name.to_lowercase().contains(&lower_name) || lower_name.contains(&name.to_lowercase()) {
            return Some(id.to_string());
        }
    }
    
    None
}