use good_lp::{coin_cbc, variables, Constraint, Expression, SolverModel};
use ndarray::{Array1, Array2};
use crate::optimizer::optimize_portfolio::optimize_portfolio;

pub fn calculate_efficient_frontier(
    expected_returns: &Array1<f32>,
    cov_matrix: &Array2<f32>,
    num_points: usize,
) -> Vec<(f32, f32, Array1<f32>)> {
    // Find minimum and maximum possible returns
    let min_return = *expected_returns.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_return = *expected_returns.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    // Generate target returns
    let target_returns: Vec<f32> = (0..num_points)
        .map(|i| min_return + (max_return - min_return) * (i as f32) / ((num_points - 1) as f32))
        .collect();

    // Calculate efficient frontier points
    target_returns.into_iter()
        .map(|target| optimize_portfolio(expected_returns, cov_matrix, target))
        .collect()
}