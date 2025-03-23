use pyo3::prelude::*;

#[pyfunction]
fn add_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn subtract_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a - b).to_string())
}

#[pyfunction]
fn multiply_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

#[pyfunction]
fn divide_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a / b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pycalc_cli(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(divide_as_string, m)?)?;
    Ok(())
}
