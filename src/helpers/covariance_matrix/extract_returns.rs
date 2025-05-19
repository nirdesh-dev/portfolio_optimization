use std::collections::HashMap;
use anyhow::Context;
use ndarray::{Array1, Axis};
use crate::helpers::covariance_matrix::errors::CovarianceError;
use crate::helpers::covariance_matrix::models::PriceData;
use crate::optimizer::calculate_returns::calculate_simple_returns;
use crate::optimizer::engine::Engine;

/// Extracts return data from price maps into a 2D array
///
/// # Arguments
/// * `price_maps` - A slice of price maps to extract returns from
///
/// # Returns
/// * A 2D array where each row represents returns for one symbol
pub fn extract_returns(price_maps: &[HashMap<String, Vec<f32>>], engine: &Engine) ->
anyhow::Result<PriceData> {
    // First, compute returns for all price series
    let returns_map =
        calculate_simple_returns(price_maps).context("Failed to calculate simple returns")?;

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

    match engine {
        Engine::CPU => {
            // Stack rows to form a 2D array for CPU processing
            let stacked = ndarray::stack(
                Axis(0),
                &arrays.iter().map(|a| a.view()).collect::<Vec<_>>(),
            )
                .context("Failed to stack vectors into Array2")?;

            Ok(PriceData::Matrix(stacked))
        }
        Engine::CUDA => {
            // Create flattened array for GPU processing
            let n_assets = arrays.len();
            let n_samples = expected_len;
            let mut flattened_data = Vec::with_capacity(n_assets * n_samples);

            for asset_returns in &arrays {
                flattened_data.extend(asset_returns.iter());
            }

            Ok(PriceData::Flattened {
                data: Array1::from(flattened_data),
                n_assets,
                n_samples,
            })
        }
    }
}