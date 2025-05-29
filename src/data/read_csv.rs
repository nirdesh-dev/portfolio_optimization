use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    open: Option<f32>,
    high: Option<f32>,
    low: Option<f32>,
    close: Option<f32>,
    volume: Option<u64>,
    name: String,
}

pub fn read_csv() -> Result<Vec<HashMap<String, Vec<f32>>>> {
    let path = Path::new("data.csv");
    let file = File::open(&path)
        .with_context(|| format!("Failed to open CSV file at path: {:?}", path))?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut data_by_symbol_date: HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut all_dates: BTreeSet<String> = BTreeSet::new();

    for result in rdr.deserialize() {
        let record: Record = match result {
            Ok(rec) => rec,
            Err(_) => continue,
        };

        let close = match record.close {
            Some(c) => c,
            None => continue, // Can't use row without any close price
        };

        all_dates.insert(record.date.clone());
        data_by_symbol_date
            .entry(record.name.clone())
            .or_default()
            .insert(record.date.clone(), close);
    }

    let sorted_dates: Vec<String> = all_dates.into_iter().collect();
    let mut output: Vec<HashMap<String, Vec<f32>>> = Vec::new();

    for (symbol, price_by_date) in &data_by_symbol_date {
        let mut filled_prices = Vec::with_capacity(sorted_dates.len());
        let mut last_price: Option<f32> = None;

        for date in &sorted_dates {
            if let Some(&price) = price_by_date.get(date) {
                filled_prices.push(price);
                last_price = Some(price);
            } else if let Some(prev) = last_price {
                filled_prices.push(prev); // forward-fill
            } else {
                // Can't start with missing data — skip this symbol
                break;
            }
        }

        // Only keep symbol if we have complete filled data
        if filled_prices.len() == sorted_dates.len() {
            let mut map = HashMap::new();
            map.insert(symbol.clone(), filled_prices);
            output.push(map);
        }
    }

    Ok(output)
}
