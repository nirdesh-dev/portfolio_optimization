use anyhow::{Context, Result};
use cudarc::driver::{CudaDevice, LaunchAsync, LaunchConfig};
use cudarc::nvrtc::compile_ptx;
use ndarray::{Array1, Array2};

// CUDA kernel for calculating covariance matrix
static COVARIANCE_KERNEL: &str = r#"
extern "C" __global__ void covariance_kernel(
    float* returns,       // Input: returns data (n_assets x n_samples flattened)
    float* means,         // Input: mean returns per asset
    float* cov_matrix,    // Output: covariance matrix (n_assets x n_assets flattened)
    int n_assets,         // Number of assets
    int n_samples         // Number of samples per asset
) {
    // Calculate thread indices
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    int j = blockIdx.y * blockDim.y + threadIdx.y;

    // Only compute upper triangular part (including diagonal)
    if (i <= j && i < n_assets && j < n_assets) {
        float cov_sum = 0.0f;

        // Calculate covariance between asset i and asset j
        for (int k = 0; k < n_samples; k++) {
            float dev_i = returns[i * n_samples + k] - means[i];
            float dev_j = returns[j * n_samples + k] - means[j];
            cov_sum += dev_i * dev_j;
        }

        // Store result in both upper and lower triangular parts for symmetry
        float cov_val = cov_sum / (n_samples - 1);
        cov_matrix[i * n_assets + j] = cov_val;
        cov_matrix[j * n_assets + i] = cov_val; // Mirror for symmetry
    }
}
"#;


/// Calculate means for each asset's returns
fn calculate_asset_means(data: &Array1<f32>, n_assets: usize, n_samples: usize) -> Vec<f32> {
    let mut means = vec![0.0f32; n_assets];

    for i in 0..n_assets {
        let mut sum = 0.0f32;
        for j in 0..n_samples {
            sum += data[i * n_samples + j];
        }
        means[i] = sum / (n_samples as f32);
    }

    means
}

/// Calculates the covariance matrix from price data
///
/// # Arguments
/// * `price_maps` - A slice of price maps to calculate covariance from
///
/// # Returns
/// * A square covariance matrix
/// Calculates the covariance matrix from price data using CUDA
///
pub fn calculate_covariance_matrix_cuda(
    data: Array1<f32>,
    n_assets: usize,
    n_samples: usize,
) -> Result<Array2<f32>> {
    // Initialize CUDA device
    let dev = CudaDevice::new(0).context("Failed to initialize CUDA device")?;

    // Calculate means for each asset
    let means = calculate_asset_means(&data, n_assets, n_samples);

    // Load the kernel
    // First, compile the PTX from the source code
    let cov_ptx = compile_ptx(COVARIANCE_KERNEL).context("Could not compile the CUDA Kernel \
    source code to PTX")?;
    // Load the compiled PTX
    dev.load_ptx(cov_ptx, "covariance_kernel", &["covariance_kernel"])
        .context("Failed to load CUDA kernel")?;

    // Get the loaded function
    let kernel = dev.get_func("covariance_kernel", "covariance_kernel")
        .context("Failed to get CUDA kernel function")?;


    // Load data into the kernel
    let d_returns = dev.htod_copy(data.to_vec()).context("Failed to copy returns data to the \
    device")?;
    let d_means = dev.htod_copy(means).context("Failed to copy means data to device")?;

    // Load the covariance matrix (size: n x n)
    let d_cov_matrix = dev.alloc_zeros::<f32>(n_assets * n_assets)
        .context("Failed to allocate covariance matrix on device")?;

    // Define grid and block dimensions for the kernel
    let block_dim = (16, 16, 1); // 16×16 threads per block
    let grid_dim = (
        (n_assets as u32 + block_dim.0 - 1) / block_dim.0,
        (n_assets as u32 + block_dim.1 - 1) / block_dim.1,
        1
    );

    // Create launch configuration
    let config = LaunchConfig {
        grid_dim,
        block_dim,
        shared_mem_bytes: 0,
    };

    // Launch kernel
    // Launch kernel
    unsafe {
        kernel.launch(
            config,
            (
                &d_returns,
                &d_means,
                &d_cov_matrix,
                n_assets as i32,
                n_samples as i32
            ),
        )
            .context("Failed to launch CUDA kernel")?;
    }

    // Copy result back from device
    let mut cov_matrix_flat = vec![0.0f32; n_assets * n_assets];
    let cov_matrix_flat = dev.dtoh_sync_copy(&d_cov_matrix)
        .context("Failed to copy covariance matrix from device")?;

    // Reshape flat array into 2D array
    let cov_matrix = Array2::from_shape_vec((n_assets, n_assets), cov_matrix_flat)
        .context("Failed to reshape covariance matrix")?;

    Ok(cov_matrix)
}