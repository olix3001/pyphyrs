// PyO3 imports
use pyo3::prelude::*;

// Base module
#[pymodule]
fn pyphyrs(_py: Python, m: &PyModule) -> PyResult<()> {
    // Init __doc__ attribute
    m.add("__doc__", "Physics library for simulation and analysis")?;
    // Init __version__ attribute
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Return Ok(()) to indicate that initialization was successful
    Ok(())
}