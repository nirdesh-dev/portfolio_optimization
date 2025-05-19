use thiserror::Error;

#[derive(Error, Debug)]
pub enum CovarianceError {
    #[error("Input matrix must have at least 2 observations")]
    InsufficientObservations,
    #[error("All samples must have equal length")]
    UnevenSampleLength,
}