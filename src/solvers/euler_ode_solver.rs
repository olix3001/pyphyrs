// External imports
use nalgebra::DVector;

// PyO3 imports
use pyo3::prelude::*;

// Crate imports
use crate::Float;
use super::ODESolver;

// Euler solver implementation
#[pyclass]
pub struct EulerODE;

// Euler solver implementation
impl ODESolver for EulerODE {
    fn solve(&self, dt: Float, positions: &mut DVector<Float>, velocities: &mut DVector<Float>, accelerations: &mut DVector<Float>) {
        // Update positions
        *positions += velocities.clone() * dt;
        // Update velocities
        *velocities += accelerations.clone() * dt;
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_ode_solver() {
        // Create solver
        let solver = EulerODE;

        // Create vectors
        let mut positions = DVector::from_vec(vec![0.0, 0.0]);
        let mut velocities = DVector::from_vec(vec![1.0, 1.0]);
        let mut accelerations = DVector::from_vec(vec![0.0, 0.0]);

        // Solve
        solver.solve(1.0, &mut positions, &mut velocities, &mut accelerations);

        // Check results
        assert_eq!(positions, DVector::from_vec(vec![1.0, 1.0]));
        assert_eq!(velocities, DVector::from_vec(vec![1.0, 1.0]));
        assert_eq!(accelerations, DVector::from_vec(vec![0.0, 0.0]));
    }

    #[test]
    fn euler_ode_solver_with_acceleration() {
        // Create solver
        let solver = EulerODE;

        // Create vectors
        let mut positions = DVector::from_vec(vec![0.0, 0.0]);
        let mut velocities = DVector::from_vec(vec![1.0, 1.0]);
        let mut accelerations = DVector::from_vec(vec![1.0, 1.0]);

        // Solve
        solver.solve(1.0, &mut positions, &mut velocities, &mut accelerations);

        // Check results
        assert_eq!(positions, DVector::from_vec(vec![1.0, 1.0]));
        assert_eq!(velocities, DVector::from_vec(vec![2.0, 2.0]));
        assert_eq!(accelerations, DVector::from_vec(vec![1.0, 1.0]));
    }
}

// Benchmark module
#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn euler_ode_solver(b: &mut Bencher) {
        // Create solver
        let solver = EulerODE;

        // Create vectors
        let mut positions = DVector::from_vec(vec![0.0, 0.0]);
        let mut velocities = DVector::from_vec(vec![1.0, 1.0]);
        let mut accelerations = DVector::from_vec(vec![0.0, 0.5]);

        // Solve
        b.iter(|| {
            for _ in 0..20000 {
                solver.solve(0.01, &mut positions, &mut velocities, &mut accelerations)
            }
        });
    }
}