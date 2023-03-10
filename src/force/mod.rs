// PyO3 imports
use pyo3::prelude::*;

// Crate imports
use crate::Float;

// Import Forces
mod spring;
mod gravity;
// Create force generator class
#[pyclass(name="ForceGenerator", subclass)]
pub struct ForceGenerator;

// Force generator implementation
#[pymethods]
impl ForceGenerator {
    // Constructor
    #[new]
    fn new() -> Self {
        ForceGenerator
    }

    // Get energy
    fn get_energy(&self, _py: Python) -> PyResult<Float> {
        // Panic if not implemented
        panic!("Force generator does not implement get_energy() method");
    }
    
    // Apply force
    fn apply_force(&self, _py: Python) -> PyResult<()> {
        // Panic if not implemented
        panic!("Force generator does not implement apply_force() method");
    }

    // __doc__ attribute
    fn __doc__(&self) -> &'static str {
        "Force generator is a base class for all forces"
    }
}

// Default implementation
impl Default for ForceGenerator {
    fn default() -> Self {
        ForceGenerator
    }
}

// Create force module
#[pymodule]
pub fn force(_py: Python, m: &PyModule) -> PyResult<()> {
    // __doc__ attribute
    m.add("__doc__", "Module with force generators")?;

    // Force generator class
    m.add_class::<ForceGenerator>()?;

    // Add forces
    m.add_class::<spring::SpringForce>()?;
    m.add_class::<gravity::GravityForce>()?;

    // Return Ok(()) to indicate that initialization was successful
    Ok(())
}