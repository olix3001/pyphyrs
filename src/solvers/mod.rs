mod euler_ode_solver;

// External imports
use nalgebra::DVector;

// PyO3 imports
use pyo3::prelude::*;

// Crate imports
use crate::Float;

// Import solvers
pub use self::euler_ode_solver::EulerODE;

// Trait for ODE solvers
pub trait ODESolver {
    fn solve(&self, dt: Float, positions: &mut DVector<Float>, velocities: &mut DVector<Float>, accelerations: &mut DVector<Float>);
}

// Module with solvers
#[pymodule]
pub fn solvers(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add Euler solver
    m.add_class::<EulerODE>()?;

    // Return Ok(()) to indicate that initialization was successful
    Ok(())
}