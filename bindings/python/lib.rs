#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python-bindings")]
#[pymodule]
fn hpdg_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    let _ = m;
    Ok(())
}
