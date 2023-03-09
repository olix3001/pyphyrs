// PyO3 imports
use pyo3::prelude::*;

// Crate imports
use crate::{Float, scene::MassRef};

use super::ForceGenerator;

// Spring implementation
#[pyclass(name="SpringForce", extends=super::ForceGenerator)]
pub struct SpringForce {
    // Spring constant
    k: Float,
    // Rest length
    rest_length: Float,

    // Masses
    m1: Py<MassRef>,
    m2: Py<MassRef>,
}

// Spring implementation
#[pymethods]
impl SpringForce {
    // Constructor
    #[new]
    fn new(m1: Py<MassRef>, m2: Py<MassRef>, k: Option<Float>, rest_length: Option<Float>, py: Python) -> (Self, ForceGenerator) {
        // Create spring
        (SpringForce {
            k: k.unwrap_or(1.0), // Default spring constant is 1
            // Default rest length is distance between masses
            rest_length: rest_length.unwrap_or_else(|| (m1.borrow_mut(py).raw_position(py) - m2.borrow_mut(py).raw_position(py)).norm()),
            m1,
            m2,
        }, ForceGenerator::default())
    }

    // Get energy
    fn get_energy(&self, py: Python) -> PyResult<Float> {
        // Get masses
        let m1 = self.m1.borrow_mut(py);
        let m2 = self.m2.borrow_mut(py);

        // Get positions
        let p1 = m1.raw_position(py);
        let p2 = m2.raw_position(py);

        // Get rest length
        let rest_length = self.rest_length;

        // Get spring constant
        let k = self.k;

        // Calculate energy
        let energy = 0.5 * k * (p1 - p2).norm_squared() - rest_length;

        // Return energy
        Ok(energy)
    }
    
    // Apply force
    fn apply_force(&self, py: Python) -> PyResult<()> {
        // Get masses
        let m1 = self.m1.borrow_mut(py);
        let m2 = self.m2.borrow_mut(py);

        // Get positions
        let p1 = m1.raw_position(py);
        let p2 = m2.raw_position(py);

        // Get rest length
        let rest_length = self.rest_length;

        // Get spring constant
        let k = self.k;

        // Calculate force
        let force = k * (p1 - p2).normalize() * ((p1 - p2).norm() - rest_length);

        // Apply force
        m1.raw_apply_force(py, -force);
        m2.raw_apply_force(py, force);

        // Return Ok(()) to indicate that initialization was successful
        Ok(())
    }

    // __doc__ attribute
    fn __doc__(&self) -> &'static str {
        "Spring force generator (native implementation)"
    }
}