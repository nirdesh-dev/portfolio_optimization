mod data;

use yahoo_finance_api as yahoo;
use anyhow::Result;
use crate::data::fetch_data::fetch_quotes_range;
use crate::data::yahoo_periods::{Interval, Range};

#[tokio::main]
async fn main() -> Result<()>{
    let provider = yahoo::YahooConnector::new().unwrap();
    let symbols = vec!["AAPL", "MSFT", "GOOG"];
    let prices = fetch_quotes_range(
        &provider,
        &symbols,
        Interval::Day1,
        Range::Month1
    ).await?;
    println!("Prices: {:?}", prices);
    Ok(())
}
