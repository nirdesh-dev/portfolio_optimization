mod data;
mod helpers;
mod optimizer;

use crate::data::fetch_data::fetch_close_prices_range;
use crate::data::yahoo_periods::{Interval, Range};
use crate::helpers::covariance_matrix::calculate_covariance_matrix::calculate_covariance_matrix;
use crate::optimizer::calculate_returns::calculate_average_returns;
use crate::optimizer::engine::Engine;
use anyhow::{Context, Result};
use tokio::time::Instant;
use yahoo_finance_api as yahoo;
use crate::data::read_csv::read_csv;
use ndarray::{Array1};
use crate::helpers::filter_map::filter_price_map;
use crate::optimizer::efficient_frontier::calculate_efficient_frontier;
use crate::optimizer::minimum_variance_portfolio::calcalate_mvp;

#[tokio::main]
async fn main() -> Result<()> {
    //
    let symbols = vec!["AAPL", "MSFT", "GOOGL"];
    //
    // let prices = fetch_close_prices_range(&provider, &symbols, Interval::Day1, Range::Month1)
    //     .await
    //     .context("Failed to fetch price data")?;

    let price_map = read_csv()?;

    let filtered_map = filter_price_map(&symbols, price_map);
    println!("{:?}", filtered_map);

    // for (i, map) in price_map.iter().enumerate() {
    //     for (symbol, prices) in map {
    //         println!("Symbol: {:?}, price_len: {:?}", symbol, prices.len())
    //     }
    // }

    // println!("{:?}", prices);
    //
    // let mean_daily_returns =
    //     calculate_average_returns(&filtered_map).context("Failed to calculate expected returns")?;
    // println!("Mean daily returns: {:?}", mean_daily_returns);
    //
    // // Convert returns to ndarray for optimization
    // let expected_returns = {
    //     let mut price_array = Vec::with_capacity(mean_daily_returns.len());
    //
    //     for (symbol, price) in &mean_daily_returns {
    //         println!("{}: {:.4}%", symbol, price * 100.0);
    //         price_array.push(*price);
    //     }
    //
    //     Array1::from_vec(price_array)
    // };

    // let start = Instant::now();
    // let cov_matrix_cpu = calculate_covariance_matrix(&prices, Engine::Cpu)?;
    // let cpu_time = start.elapsed();
    // println!("Covariance Matrix From CPU: {:?}", cov_matrix_cpu);
    // println!("CPU Time: {:?}", cpu_time);
    //
    // let start = Instant::now();
    // let cov_matrix_gpu = calculate_covariance_matrix(&filtered_map, Engine::Cuda)?;
    // let gpu_time = start.elapsed();
    // println!("Covariance Matrix From GPU: {:?}", cov_matrix_gpu);
    // println!("GPU Time: {:?}", gpu_time);
    //
    // let mvp_weights = calcalate_mvp(&cov_matrix_gpu);
    // println!("{:?}", mvp_weights);
    //
    // let frontier = calculate_efficient_frontier(&expected_returns, &cov_matrix_gpu, 10);
    // // Print results
    // for (ret, var, weights) in frontier {
    //     println!(
    //         "Return: {:.4}, Risk: {:.4}, Weights: {:?}",
    //         ret,
    //         var.sqrt(),
    //         weights
    //     );
    // }

    Ok(())
}
