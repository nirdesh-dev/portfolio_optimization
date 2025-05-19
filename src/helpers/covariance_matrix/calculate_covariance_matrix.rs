use anyhow::{Context, Result};
use ndarray::{Array2};
use std::collections::HashMap;
use crate::optimizer::engine::Engine;
use super::extract_returns::extract_returns;
use super::models::PriceData;
use super::covariance_matrix_cpu::calculate_covariance_matrix_cpu;
use super::covariance_matrix_cuda::{calculate_covariance_matrix_cuda};

/// Calculates the covariance matrix from price data
///
/// # Arguments
/// * `price_maps` - A slice of price maps to calculate covariance from
///
/// # Returns
/// * A square covariance matrix
pub fn calculate_covariance_matrix(
    price_maps: &[HashMap<String, Vec<f32>>],
    engine: Engine,
) -> Result<Array2<f32>> {
    let price_data = extract_returns(price_maps, &engine)
        .context("Failed to extract returns for covariance calculation")?;
    match engine {
        Engine::CPU => {
            if let PriceData::Matrix(price_array_2d) = price_data {
                let covariance_matrix = calculate_covariance_matrix_cpu(price_array_2d)?;
                Ok(covariance_matrix)
            } else {
                unreachable!("CPU engine should always return Matrix data")
            }
        }
        Engine::CUDA => {
            if let PriceData::Flattened { data, n_assets, n_samples } = price_data {
                let covariance_matrix = calculate_covariance_matrix_cuda(data, n_assets,
                                                                         n_samples)?;
                Ok(covariance_matrix)
            } else {
                unreachable!("CUDA engine should always return Flattened data")
            }
        }
    }
}
