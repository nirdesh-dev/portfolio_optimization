use anyhow::Result;
use std::collections::HashMap;

/// Calculates simple returns from a sequence of prices
///
/// # Arguments
/// * `price_maps` - Vector of price maps, where each map links symbols to their price sequences
///
/// # Returns
/// * A HashMap mapping symbols to their corresponding return sequences
pub fn calculate_simple_returns(
    price_maps: &[HashMap<String, Vec<f32>>],
) -> Result<HashMap<String, Vec<f32>>> {
    let mut result = HashMap::new();

    for price_map in price_maps {
        for (symbol, prices) in price_map {
            if prices.len() < 2 {
                continue;
            }
            let returns: Vec<f32> = prices
                .windows(2)
                .map(|window| (window[1] - window[0]) / window[0])
                .collect();
            result.insert(symbol.clone(), returns);
        }
    }
    Ok(result)
}

/// Calculates expected (average) returns from price data
///
/// # Arguments
/// * `price_maps` - Vector of price maps, where each map links symbols to their price sequences
///
/// # Returns
/// * A HashMap mapping symbols to their expected returns
#[allow(dead_code)]
pub fn calculate_expected_returns(
    price_maps: &[HashMap<String, Vec<f32>>],
) -> Result<HashMap<String, f32>> {
    let mut result = HashMap::new();
    let pct_daily_returns = calculate_simple_returns(price_maps).unwrap();
    for (symbol, prices) in pct_daily_returns {
        let exp_returns: f32 = prices.iter().sum::<f32>() / prices.len() as f32;
        result.insert(symbol, exp_returns);
    }
    Ok(result)
}
