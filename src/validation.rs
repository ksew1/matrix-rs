#[derive(thiserror::Error, Debug)]
pub enum MatrixError {
    #[error("Matrix is not a power of two")]
    NotPowerOfTwo,
    #[error("Matrix is not square")]
    NotSquare,
    #[error("Matrix is empty")]
    Empty,
}

#[derive(thiserror::Error, Debug)]
pub enum MatricesError {
    #[error("{name}: {source}")]
    MatrixError {
        name: &'static str,
        source: MatrixError,
    },
    #[error("Matrix A and B are not the same size")]
    MatricesNotTheSameSize,
}

/// Validates that the matrices are of the same size and are square, power of two and not empty.
pub fn validate_matrices(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<(), MatricesError> {
    validate_matrix(a).map_err(|e| MatricesError::MatrixError {
        name: "A",
        source: e,
    })?;
    validate_matrix(b).map_err(|e| MatricesError::MatrixError {
        name: "B",
        source: e,
    })?;
    if a.len() != b.len() {
        return Err(MatricesError::MatricesNotTheSameSize);
    }
    Ok(())
}

/// Validates that the matrix is square, power of two and not empty.
pub fn validate_matrix(matrix: &[Vec<f64>]) -> Result<(), MatrixError> {
    if matrix.is_empty() {
        return Err(MatrixError::Empty);
    }
    let n = matrix.len();
    if !n.is_power_of_two() {
        return Err(MatrixError::NotPowerOfTwo);
    }
    if matrix.iter().any(|row| row.len() != n) {
        return Err(MatrixError::NotSquare);
    }
    Ok(())
}
