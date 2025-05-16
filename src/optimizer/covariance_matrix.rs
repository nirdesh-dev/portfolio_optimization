use crate::optimizer::calculate_returns::calculate_simple_returns;
use anyhow::{Context, Result};
use ndarray::{Array1, Array2, Axis};
use ndarray_stats::CorrelationExt;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CovarianceError {
    #[error("Input matrix must have at least 2 observations")]
    InsufficientObservations,
    #[error("All samples must have equal length")]
    UnevenSampleLength,
}

/// Extracts return data from price maps into a 2D array
///
/// # Arguments
/// * `price_maps` - A slice of price maps to extract returns from
///
/// # Returns
/// * A 2D array where each row represents returns for one symbol
pub fn extract_returns(price_maps: &[HashMap<String, Vec<f32>>]) -> Result<Array2<f32>> {
    // First, compute returns for all price series
    let returns_map = calculate_simple_returns(&price_maps)?;

    // Pre-allocate with capacity to avoid reallocations
    let mut arrays = Vec::with_capacity(returns_map.len());

    // Extract returns vectors and convert to Array1
    for (_symbol, returns) in returns_map {
        arrays.push(Array1::from(returns));
    }

    // Must have at least two vectors to compute covariance/correlation
    if arrays.len() < 2 {
        return Err(CovarianceError::InsufficientObservations)
            .context("Need at least 2 rows for analysis");
    }

    let expected_len = arrays[0].len();
    if !arrays.iter().all(|v| v.len() == expected_len) {
        return Err(CovarianceError::UnevenSampleLength)
            .context("Each sample (row) must have the same length");
    }
    // Stack rows to form a 2D array
    let stacked = ndarray::stack(
        Axis(0),
        &arrays.iter().map(|a| a.view()).collect::<Vec<_>>(),
    )
        .context("Failed to stack vectors into Array2")?;

    Ok(stacked)
}

/// Calculates the covariance matrix from price data
///
/// # Arguments
/// * `price_maps` - A slice of price maps to calculate covariance from
///
/// # Returns
/// * A square covariance matrix
pub fn calculate_covariance_matrix(
    price_maps: &[HashMap<String, Vec<f32>>],
) -> Result<Array2<f32>> {
    let price_array_2d = extract_returns(price_maps)
        .context("Failed to extract returns for covariance calculation")?;

    let covariance_matrix = price_array_2d.cov(1.)
        .context("Failed to calculate covariance matrix")?;

    Ok(covariance_matrix)
}
