#![allow(dead_code)]
mod tests;

use std::collections::HashMap;
use std::collections::hash_map::Values;
use std::error::Error;
use std::hash::Hash;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

#[derive(Debug)]
struct Record {
    time_period: String,
    station: String,
    entries: HashMap<TimeOfDay, i32>,
    exits: HashMap<TimeOfDay, i32>,
    latitude: f64,
    longitude: f64,
}

/// The data is initialised/preprocessed once through `data_preprocessing` and
/// can then be reused to answer many queries. You can see below that queries
/// take in `&Data` immutable references.
pub struct Data {
    // TODO: You can, but don't have to, add any additional state that you would like to share between query answers here.
    records: Vec<Record>,
}

/// Preprocess the raw `CSVRecord` structs.
pub fn data_preprocessing() -> Result<Data, Box<dyn Error>> {
    // TODO: If you want to (optionally) add some more state to `Data`, you need
    // to initialise that state here.
    let path = Path::new("trains.csv");

    let csv_records: Vec<CSVRecord> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    let records: Vec<Record> = csv_records
        .into_iter()
        .map(|csv_record| convert_csvrecord_to_record(&csv_record))
        .collect();
    Ok(Data { records })
}

/// What is the north-most station?
// pub fn find_north_most_station(data: &Data) -> Option<String> {
//     let mut station_lat: HashMap<String, f64> = HashMap::new();
//     for record in data.records.iter().clone() {
//         station_lat.insert(record.station.clone(), record.latitude);
//     }
//     let station = station_lat
//         .iter()
//         .max_by(|a, b| a.1.total_cmp(b.1))
//         .map(|(name, _)| name.clone());

//     station
// }

pub fn find_north_most_station(data: &Data) -> Option<String> {

    let station = data.records
        .iter()
        .max_by(|a, b| a.latitude.total_cmp(&b.latitude))
        .map(|record| record.station.clone());

    station
}

/// What is the south-most station?
pub fn find_south_most_station(data: &Data) -> Option<String> {
    let station = data.records
        .iter()
        .min_by(|a, b| a.latitude.total_cmp(&b.latitude))
        .map(|record| record.station.clone());
    station
}

/// What is the east-most station?
pub fn find_east_most_station(data: &Data) -> Option<String> {
    let station = data.records
        .iter()
        .max_by(|a, b| a.latitude.total_cmp(&b.longitude))
        .map(|record| record.station.clone());
    station
}

/// What is the west-most station?
pub fn find_west_most_station(data: &Data) -> Option<String> {
    let station = data.records
        .iter()
        .min_by(|a, b| a.latitude.total_cmp(&b.longitude))
        .map(|record| record.station.clone());
    station
}

/// Return the names of the most and least used (total entries + exits) stations on the NSW network at each time of day, in total over all of the years.
pub fn most_least_used_stations(data: &Data, time_of_day: TimeOfDay) -> Option<(String, String)> {
    let mut station_usage: HashMap<&str, i32> = HashMap::new();

    for record in data.records.iter() {
        let entry_num = record.entries.get(&time_of_day).copied();
        let exit_num = record.exits.get(&time_of_day).copied();
        
        // add two options
        let sum = match (entry_num, exit_num) {
            (Some(e), Some(x)) => Some(e + x),
            _ => entry_num.or(exit_num), // Option::or returns the first Some it finds
        };

        if let Some(total) = sum {
            *station_usage.entry(&record.station).or_insert(0) += total;
        }
    }

    let most_station = station_usage
        .iter()
        .max_by_key(|(_k, v)| *v)
        .map(|(k, _v)| k.to_string());
    let least_station = station_usage
        .iter()
        .min_by_key(|(_k, v)| *v)
        .map(|(k, _v)| k.to_string());

    Some((least_station?, most_station?))
}

// TODO: if you think the Vec return type is inefficient/unsuitable, ask your tutor about more flexible alternatives (hint: iterators).
/// Allow a user to search for a station, and show it's busiest times of day.
pub fn search_station_busiest_times_of_day(
    data: &Data,
    station_name: &str,
) -> Option<Vec<(TimeOfDay, i32)>> {
    // TODO: implement
    todo!()
}

/// Allow a user to search for a station, if it exists, and show it's busiest year.
pub fn search_station_busiest_year(data: &Data, station_name: &str) -> Option<String> {
    // TODO: implement
    todo!()
}

/// Which station had its yearly utilisation (total entries + exits) increase the most from 2016 (inclusive) to 2020 (inclusive)?
pub fn find_largest_yearly_utilisation_increase(data: &Data) -> Option<String> {
    // TODO: implement
    todo!()
}

/// Which station had the biggest percentage change in utilisation (total entries + exits) from 2019 to 2020?
pub fn find_biggest_percentage_change(data: &Data) -> Option<String> {
    // TODO: implement
    todo!()
}

/// Find the names of the two closest from each other.
pub fn find_two_closest_stations(data: &Data) -> Option<(String, String)> {
    // TODO: implement
    todo!()
}

/// Find the names of the two furthest away from each other.
pub fn find_two_furthest_stations(data: &Data) -> Option<(String, String)> {
    // TODO: implement
    todo!()
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: You don't have to add anything here, but if you want to test your
    // query implementations outside of the unit tests, you can do that here.
    let data = data_preprocessing()?;
    let station = find_north_most_station(&data).unwrap();

    println!("The most north station is {}", station);


    Ok(())
}

#[derive(Deserialize, Debug)]
// a raw deserialized row from trains.csv
struct CSVRecord {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>, // some stations have - (no data) instead a number

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
// HashMap keys
pub enum TimeOfDay {
    Morning,
    Midday,
    Evening,
    Midnight,
    Total,
}

/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn convert_csvrecord_to_record(csv_record: &CSVRecord) -> Record {
    let mut record = Record {
        time_period: csv_record.time_period.clone(),
        station: csv_record.station.clone(),
        entries: HashMap::new(),
        exits: HashMap::new(),
        latitude: csv_record.latitude,
        longitude: csv_record.longitude,
    };

    if let Some(e) = csv_record.entries_morning {
        record.entries.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_record.entries_midday {
        record.entries.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_record.entries_evening {
        record.entries.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_record.entries_midnight {
        record.entries.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_record.entries_total {
        record.entries.insert(TimeOfDay::Total, e);
    }

    if let Some(e) = csv_record.exits_morning {
        record.exits.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_record.exits_midday {
        record.exits.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_record.exits_evening {
        record.exits.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_record.exits_midnight {
        record.exits.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_record.exits_total {
        record.exits.insert(TimeOfDay::Total, e);
    }

    record
}
