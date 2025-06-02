use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use anyhow::{Result, Context};
use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct PolygonResult {
    c: f32, // closing price
}

#[derive(Debug, Deserialize)]
struct PolygonResponse {
    ticker: String,
    results: Vec<PolygonResult>,
}

pub async fn fetch_polygon_data(
    symbols: &[&str],
    start_date: &str,
    end_date: &str,
) -> Result<Vec<HashMap<String, Vec<f32>>>> {
    let client = Client::new();
    let mut all_data = Vec::new();
    let api_key = std::env::var("POLYGON_API_KEY").context("Could not load env file")?;

    let mut csv_writer = csv::Writer::from_writer(File::create("polygon_data.csv")?);
    csv_writer.write_record(&["ticker", "index", "close"])?;

    for &symbol in symbols {
        let url = format!(
            "https://api.polygon.io/v2/aggs/ticker/{}/range/1/day/{}/{}?adjusted=true&sort=asc&limit=50000&apiKey={}",
            symbol,
            start_date,
            end_date,
            api_key
        );
        println!("{}", url);

        let res = client.get(&url).send().await?.text().await?;
        let parsed: PolygonResponse = serde_json::from_str(&res)?;

        let closes: Vec<f32> = parsed.results.iter().map(|r| r.c).collect();

        for (i, &price) in closes.iter().enumerate() {
            csv_writer.write_record(&[&parsed.ticker, &i.to_string(), &price.to_string()])?;
        }

        let mut symbol_map = HashMap::new();
        symbol_map.insert(parsed.ticker, closes);
        all_data.push(symbol_map);
    }

    csv_writer.flush()?;
    Ok(all_data)
}
