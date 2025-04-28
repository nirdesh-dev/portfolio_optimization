use anyhow::Result;
use yahoo_finance_api as yahoo;
use chrono::NaiveDate;
use crate::data::yahoo_periods::{Interval, Range};

pub async fn fetch_quotes_range(
    provider: &yahoo::YahooConnector,
    symbols: &[&str], interval: Interval, range: Range
) -> Result<Vec<Vec<f32>>> {

    let mut close_prices = Vec::new();
    for &symbol in symbols {
        let response = provider.get_quote_range(
            symbol,
            interval.as_str(),
            range.as_str()
        ).await?;
        let quotes_close_prices: Vec<f32> = response.quotes().unwrap()
            .into_iter()
            .map(|quote| quote.close as f32)
            .collect();
        close_prices.push(quotes_close_prices);
    }
    Ok(close_prices)
}
