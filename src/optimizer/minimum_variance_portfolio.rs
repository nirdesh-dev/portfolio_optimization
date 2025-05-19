use ndarray::{Array1, Array2};
use ndarray_linalg::Inverse;

#[allow(unused)]
pub fn calcalate_mvp(cov_matrix: &Array2<f32>) -> Array1<f32> {
    let inv_cov = cov_matrix.inv().unwrap();
    let ones = Array1::ones(cov_matrix.nrows());
    inv_cov.dot(&ones) / ones.dot(&inv_cov.dot(&ones))
}
