use anyhow::{Context, Result};
use ndarray::{Array2, Array1, Axis};
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

pub fn extract_price_vectors(price_maps: Vec<HashMap<String, Vec<f32>>>) -> Result<Array2<f32>> {
    let mut arrays: Vec<Array1<f32>> = Vec::new();

    for map in price_maps {
        for (_key, vec) in map {
            arrays.push(Array1::from(&vec[..]));
        }
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
    let array = ndarray::stack(Axis(0), &arrays).context("Failed to stack vectors into Array2")?;

    Ok(array)
}

// calculate covariance matrix for row-oriented data (each row is a variable)
pub fn calculate_covariance_matrix(
    price_maps: Vec<HashMap<String, Vec<f32>>>,
) -> Result<Array2<f32>> {
    let price_array_2d = extract_price_vectors(price_maps).unwrap();
    let covariance_matrix = price_array_2d.cov(1.).unwrap();
    Ok(covariance_matrix)
}
