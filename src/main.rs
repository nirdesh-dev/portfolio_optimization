mod data;
mod optimizer;

use crate::data::fetch_data::fetch_close_prices_range;
use crate::data::yahoo_periods::{Interval, Range};
use crate::optimizer::covariance_matrix::calculate_covariance_matrix;
use anyhow::Result;
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = yahoo::YahooConnector::new().unwrap();
    let symbols = vec!["AAPL", "MSFT", "GOOG"];
    let prices =
        fetch_close_prices_range(&provider, &symbols, Interval::Day1, Range::Month1).await?;
    println!("{:?}", prices);
    // let returns = calculate_simple_returns(prices);
    // let returns = calculate_expected_returns(prices);
    // println!("{:?}", returns);
    let cov_matrix = calculate_covariance_matrix(prices).unwrap();
    println!("{:?}", cov_matrix);
    Ok(())
}
