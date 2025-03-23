use pyo3::prelude::*;
use pyo3::exceptions::PyZeroDivisionError;

#[pyfunction]
fn divide(a: usize, b: usize) -> PyResult<String> {
    if b == 0 {
        Err(PyZeroDivisionError::new_err("Cannot divide by zero"))
    } else {
        Ok((a / b).to_string())
    }
}
/// A Python module implemented in Rust.
#[pymodule]
fn librust_exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    Ok(())
}
