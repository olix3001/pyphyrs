// Benchmark enable (nightly only)
#![cfg_attr(feature="bench", feature(test))]

// PyO3 imports
use pyo3::{prelude::*, wrap_pymodule};

// Base float precision
#[cfg(feature = "f64precision")]
pub type Float = f64;
#[cfg(not(feature = "f64precision"))]
pub type Float = f32;

// Vector2 type
pub type Vec2 = (Float, Float);

// Components of the library
mod scene;

mod solvers;

mod force;

// Base module
#[pymodule]
fn pyphyrs(py: Python, m: &PyModule) -> PyResult<()> {
    // Init __doc__ attribute
    m.add("__doc__", "Physics library for simulation and analysis")?;
    // Init __version__ attribute
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // ====< Basics >====
    // Add scene class
    m.add_class::<scene::Scene>()?;

    // ====< Submodules >====
    // Add solvers submodule
    m.add_wrapped(wrap_pymodule!(solvers::solvers))?;

    // Add force submodule
    m.add_wrapped(wrap_pymodule!(force::force))?;

    // Return Ok(()) to indicate that initialization was successful
    Ok(())
}