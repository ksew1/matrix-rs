mod constraints;
mod mat_multiply;
mod validation;

pub use mat_multiply::multiply;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn mul(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    multiply(a, b).map_err(|err| PyValueError::new_err(err.to_string()))
}

#[pymodule]
fn matrix_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mul, m)?)?;
    Ok(())
}
