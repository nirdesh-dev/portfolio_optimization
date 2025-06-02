use clarabel::algebra::CscMatrix;
use clarabel::solver::{DefaultSettings, DefaultSolver, IPSolver, NonnegativeConeT, ZeroConeT};
use ndarray::{Array1, Array2};

#[allow(unused)]
pub fn optimize_portfolio(
    expected_returns: &Array1<f32>,
    cov_matrix: &Array2<f32>,
    target_return: f32,
) -> (f32, f32, Array1<f32>) {
    let n = expected_returns.len();

    // 1. Objective: Minimize (1/2)w'Σw
    let p = {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut values = Vec::new();

        // Only upper triangular part needed (including diagonal)
        for j in 0..n {
            for i in 0..=j {
                rows.push(i);
                cols.push(j);
                values.push(cov_matrix[[i, j]]);
            }
        }

        // Convert to CSC format
        let mut colptr = vec![0];
        let mut nnz = 0;
        for j in 0..n {
            nnz += j + 1; // Count elements in upper triangle up to column j
            colptr.push(nnz);
        }
        CscMatrix::new(n, n, colptr, rows, values)
    };

    // Zero linear term
    let q = vec![0.; n];

    // 2. Constraints
    // a) Equality: sum(w) = 1 (A_eq * w = 1)
    // b) Inequality: μ'w ≥ target_return (A_ineq * w ≥ target_return)

    // Combined constraint matrix [A_eq; A_ineq]
    let a = {
        let mut colptr = Vec::with_capacity(n + 1);
        let mut rowval = Vec::with_capacity(2 * n);
        let mut nzval = Vec::with_capacity(2 * n);

        colptr.push(0);
        for j in 0..n {
            // Equality constraint (first row)
            rowval.push(0);
            nzval.push(1.0);

            // Inequality constraint (second row)
            rowval.push(1);
            nzval.push(expected_returns[j]);

            colptr.push((j + 1) * 2);
        }

        CscMatrix::new(2, n, colptr, rowval, nzval)
    };

    // Right-hand side [1, target_return]
    let b = vec![1., target_return];

    // 3. Cone specification:
    // - 1 equality constraint (ZeroCone)
    // - 1 inequality constraint (NonnegativeCone)
    let cones = [ZeroConeT(1), NonnegativeConeT(1)];

    // 4. Solver settings
    let settings = DefaultSettings::default();

    // 5. Solve the problem
    let mut solver = DefaultSolver::new(&p, &q, &a, &b, &cones, settings);
    solver.solve();

    // 6. Extract solution
    let weights = Array1::from_vec(solver.solution.x);
    let achieved_return = expected_returns.dot(&weights);
    let variance = weights.dot(&cov_matrix.dot(&weights));

    (achieved_return, variance.sqrt(), weights)
}
