// PyO3 imports
use pyo3::prelude::*;

// Crate imports
use crate::{Float, scene::MassRef};

use super::ForceGenerator;

// Spring implementation
#[pyclass(name="GravityForce", extends=super::ForceGenerator)]
pub struct GravityForce {
    // Gravitational constant
    g: Float,

    // Masses
    masses: Vec<Py<MassRef>>,
}

// Spring implementation
#[pymethods]
impl GravityForce {
    // Constructor
    #[new]
    #[allow(non_snake_case)]
    fn new(masses: Vec<Py<MassRef>>, G: Option<Float>, _py: Python) -> (Self, ForceGenerator) {
        // Create gravity
        (GravityForce {
            g: G.unwrap_or(0.0000673), // Default gravitational constant is 0.0000673 (Sorry, f32 precision :>)
            masses,
        }, ForceGenerator::default())
    }

    // Get energy (TODO: Fix this)
    fn get_energy(&self, py: Python) -> PyResult<Float> {
        // U = -GMm/r

        // Get masses
        let masses = &self.masses;

        // Get gravitational constant
        let g = self.g;

        // Calculate energy
        let energy = masses.iter().enumerate().map(|(i, m1)| {
            masses.iter().enumerate().filter(|(j, _)| i != *j).map(|(_, m2)| {
                let m1 = m1.borrow_mut(py);
                let m2 = m2.borrow_mut(py);

                // Get positions
                let p1 = m1.raw_position(py);
                let p2 = m2.raw_position(py);

                // Get masses
                let m1 = m1.raw_mass(py);
                let m2 = m2.raw_mass(py);

                // Calculate energy
                -g * m1 * m2 / (p1 - p2).norm()
            }).sum::<Float>()
        }).sum::<Float>();

        // Return energy
        Ok(energy)
    }
    
    // Apply force
    fn apply_force(&self, py: Python) -> PyResult<()> {
        // F = Gm1m2/r^2

        // Get masses
        let masses = &self.masses;

        // Get gravitational constant
        let g = self.g;

        // Apply force
        for (i, m1) in masses.iter().enumerate() {
            for (j, m2) in masses.iter().enumerate() {
                if i != j {
                    // Get masses
                    let m1 = m1.borrow_mut(py);
                    let m2 = m2.borrow_mut(py);

                    // Get positions
                    let p1 = m1.raw_position(py);
                    let p2 = m2.raw_position(py);

                    // Get masses
                    let m1m = m1.raw_mass(py);
                    let m2m = m2.raw_mass(py);

                    // Calculate force
                    let f = g * m1m * m2m / (p1 - p2).norm_squared();
                    let f = (p1 - p2).normalize() * f;

                    // Apply force
                    m1.raw_apply_force(py, -f);
                    m2.raw_apply_force(py, f);
                }
            }
        }

        // Return
        Ok(())
    }

    // __doc__ attribute
    fn __doc__(&self) -> &'static str {
        "Gravity force generator (native implementation)"
    }
}