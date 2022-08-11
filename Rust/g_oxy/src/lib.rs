use pyo3::prelude::*;
mod g_unit_aggr;
mod tools;
mod mat_op;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn g_unit(data: Vec<Vec<f32>>) -> PyResult<Vec<Vec<f32>>> {
    Ok(g_unit_aggr::g_unit_aggr(data))
}

/// A Python module implemented in Rust.
#[pymodule]
fn g_oxy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(g_unit, m)?)?;
    Ok(())
}