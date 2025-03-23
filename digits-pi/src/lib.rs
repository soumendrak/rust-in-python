use pyo3::prelude::*;

/// This function calculates the value of Pi using the Leibniz formula.
#[pyfunction]
fn calculate_pi(iterations: u32) -> PyResult<f64> {
    let mut pi = 0.0;
    for k in 0..iterations {
        pi += ((-1.0f64).powi(k as i32) / (2*k + 1) as f64) * 4.0 ;
    }
    Ok(pi)
}

/// A Python module implemented in Rust.
#[pymodule]
fn libdigits_pi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m.py())?)?;
    Ok(())
}
