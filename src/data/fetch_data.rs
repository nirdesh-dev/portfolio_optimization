use crate::data::yahoo_periods::{Interval, Range};
use anyhow::Result;
use chrono::NaiveDate;
use std::collections::HashMap;
use yahoo_finance_api as yahoo;

pub async fn fetch_close_prices_range(
    provider: &yahoo::YahooConnector,
    symbols: &[&str],
    interval: Interval,
    range: Range,
) -> Result<Vec<HashMap<String, Vec<f32>>>> {
    let mut close_prices = Vec::new();
    for &symbol in symbols {
        let response = provider
            .get_quote_range(symbol, interval.as_str(), range.as_str())
            .await?;
        let quotes_close_prices: Vec<f32> = response
            .quotes()
            .unwrap()
            .into_iter()
            .map(|quote| quote.close as f32)
            .collect();
        let mut price_map = HashMap::new();
        price_map.insert(symbol.to_string(), quotes_close_prices);
        close_prices.push(price_map);
    }
    Ok(close_prices)
}
