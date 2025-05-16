mod data;
mod optimizer;

use crate::data::fetch_data::fetch_close_prices_range;
use crate::data::yahoo_periods::{Interval, Range};
use crate::optimizer::calculate_returns::calculate_average_returns;
use crate::optimizer::covariance_matrix::calculate_covariance_matrix;
use crate::optimizer::efficient_frontier::calculate_efficient_frontier;
use crate::optimizer::minimum_variance_portfolio::calcalate_mvp;
use anyhow::{Context, Result};
use ndarray::Array1;
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Yahoo Finance API connector
    let provider = yahoo::YahooConnector::new().context("Failed to create Yahoo connector")?;

    let symbols = vec!["AAPL", "MSFT", "GOOG"];

    let prices = fetch_close_prices_range(&provider, &symbols, Interval::Day1, Range::Month1)
        .await
        .context("Failed to fetch price data")?;

    println!("{:?}", prices);

    let mean_daily_returns =
        calculate_average_returns(&prices).context("Failed to calculate expected returns")?;
    println!("{:?}", mean_daily_returns);

    // Convert returns to ndarray for optimization
    let expected_returns = {
        let mut price_array = Vec::with_capacity(mean_daily_returns.len());

        for (symbol, price) in &mean_daily_returns {
            println!("{}: {:.4}%", symbol, price * 100.0);
            price_array.push(*price);
        }

        Array1::from_vec(price_array)
    };

    let cov_matrix = calculate_covariance_matrix(&prices)?;
    println!("{:?}", cov_matrix);

    let mvp_weights = calcalate_mvp(&cov_matrix);
    println!("{:?}", mvp_weights);

    let frontier = calculate_efficient_frontier(&expected_returns, &cov_matrix, 10);
    // Print results
    for (ret, var, weights) in frontier {
        println!(
            "Return: {:.4}, Risk: {:.4}, Weights: {:?}",
            ret,
            var.sqrt(),
            weights
        );
    }

    Ok(())
}
