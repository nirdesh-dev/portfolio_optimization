mod data;
mod helpers;
mod optimizer;

use crate::data::fetch_data_polygon::fetch_polygon_data;
use crate::helpers::calculate_covariance_matrix;
use crate::optimizer::calcalate_mvp;
use crate::optimizer::calculate_average_returns;
use crate::optimizer::calculate_efficient_frontier;
use crate::optimizer::engine::Engine;
use anyhow::{Context, Result};
use ndarray::Array1;
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() -> Result<()> {
    let symbols = vec!["AAPL", "MSFT", "GOOGL"];

    let prices = fetch_polygon_data(&symbols, "2023-06-02", "2025-06-02").await?;

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

    let cov_matrix = calculate_covariance_matrix(&prices, Engine::Cuda)?;
    println!("{:?}", cov_matrix);
    //
    let mvp_weights = calcalate_mvp(&cov_matrix);
    println!("{:?}", mvp_weights);

    let frontier = calculate_efficient_frontier(&expected_returns, &cov_matrix, 10);
    // Print results
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
