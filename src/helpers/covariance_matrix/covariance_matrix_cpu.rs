use anyhow::{Context, Result};
use ndarray::{Array2};
use ndarray_stats::CorrelationExt;

/// Calculates the covariance matrix from price data
///
/// # Arguments
/// * `price_maps` - A slice of price maps to calculate covariance from
///
/// # Returns
/// * A square covariance matrix
pub fn calculate_covariance_matrix_cpu(
    price_array: Array2<f32>,
) -> Result<Array2<f32>> {
    let covariance_matrix = price_array
        .cov(1.)
        .context("Failed to calculate covariance matrix")?;

    Ok(covariance_matrix)
}
