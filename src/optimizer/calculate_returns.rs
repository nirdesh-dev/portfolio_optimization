use anyhow::Result;
use std::collections::HashMap;

pub fn calculate_simple_returns(
    price_maps: Vec<HashMap<String, Vec<f32>>>,
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
            result.insert(symbol, returns);
        }
    }
    Ok(result)
}

#[allow(dead_code)]
pub fn calculate_expected_returns(
    price_maps: Vec<HashMap<String, Vec<f32>>>,
) -> Result<HashMap<String, f32>> {
    let mut result = HashMap::new();
    let pct_daily_returns = calculate_simple_returns(price_maps);
    for daily_return in pct_daily_returns {
        for (symbol, prices) in daily_return {
            let exp_returns: f32 = prices.iter().sum::<f32>() / prices.len() as f32;
            result.insert(symbol, exp_returns);
        }
    }
    Ok(result)
}
