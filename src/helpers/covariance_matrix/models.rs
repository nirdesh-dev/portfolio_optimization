use ndarray::{Array1, Array2};

/// Enum representing the different return formats based on engine type
#[derive(Debug)]
pub enum PriceData {
    /// 2D array where each row represents returns for one symbol - used for CPU
    Matrix(Array2<f32>),
    /// Flattened array with all returns concatenated - used for GPU
    Flattened {
        data: Array1<f32>,
        n_assets: usize,
        n_samples: usize,
    },
}
