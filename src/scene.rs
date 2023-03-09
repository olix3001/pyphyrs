// External imports
use nalgebra::{Vector2, DVector};

// PyO3 imports
use pyo3::prelude::*;

// Crate imports
use crate::{Float, Vec2, solvers::{ODESolver, EulerODE}};

// Scene class definition
#[pyclass]
pub struct Scene {
    // Properties
    gravity: Vector2<Float>,

    // Objects stored as a vector where each object is a vector of its positions
    positions: DVector<Float>,
    velocities: DVector<Float>,
    accelerations: DVector<Float>,

    // Technicals
    ode_solver: Box<dyn Send + ODESolver>
}

// Default implementation
impl Default for Scene {
    fn default() -> Self {
        // Create scene
        Scene {
            gravity: Vector2::new(0.0, -9.81),
            positions: DVector::zeros(0),
            velocities: DVector::zeros(0),
            accelerations: DVector::zeros(0),
            ode_solver: Box::new(EulerODE)
        }
    }
}

// Scene class implementation (python)
#[pymethods]
impl Scene {
    // Constructor
    #[new]
    fn new(gravity: Option<Vec2>, ode: Option<&str>) -> Self {
        // Create scene with default values
        Self {
            gravity: match gravity {
                Some(gravity) => Vector2::new(gravity.0, gravity.1),
                None => Vector2::new(0.0, -9.81)
            },
            ode_solver: match ode {
                Some(ode) => {
                    match ode {
                        "euler" => Box::new(EulerODE),
                        _ => panic!("Unknown ODE solver: {}", ode)
                    }
                },
                None => Box::new(EulerODE)
            },
            ..Default::default()
        }
    }

    // Add mass to the scene
    fn mass(mut self_: PyRefMut<Self>) -> PyResult<MassRef> {
        // Get index of the mass
        let index = self_.positions.len() / 2;

        // Add mass to the scene
        self_.positions.extend(vec![0.0, 0.0]);
        self_.velocities.extend(vec![0.0, 0.0]);
        self_.accelerations.extend(vec![0.0, 0.0]);

        // Return reference to the mass
        Ok(MassRef {
            scene: self_.into(),
            index
        })
    }

    // Set gravity (as setter)
    #[setter(gravity)]
    fn set_gravity(&mut self, gravity: Vec2) {
        self.gravity = Vector2::new(gravity.0, gravity.1);
    }

    // Get gravity (as getter)
    #[getter(gravity)]
    fn get_gravity(&self) -> (Float, Float) {
        (self.gravity.x, self.gravity.y)
    }

    // Simulate scene
    fn simulate(mut self_: PyRefMut<Self>, steps: usize, substeps: usize, dt: Float) {
        // Time simulation
        #[cfg(feature="timings")]
        let start = std::time::Instant::now();

        // Simulate scene
        for _ in 0..steps {
            self_.update(dt, substeps);
        }

        #[cfg(feature="timings")]
        {
            println!("Simulation took {}ms", start.elapsed().as_millis());
            println!("Each step took {}ms", start.elapsed().as_millis() / steps as u128);
        }
    }

    // Get positions
    fn positions(self_: PyRef<Self>) -> Vec<(Float, Float)> {
        // Create vector of positions
        let mut positions = Vec::with_capacity(self_.positions.len() / 2);

        // Fill vector
        for i in 0..self_.positions.len() / 2 {
            positions.push((self_.positions[i * 2], self_.positions[i * 2 + 1]));
        }

        // Return positions
        positions
    }
}

// Scene internal implementation
impl Scene {
    // Update scene
    pub fn update(&mut self, dt: Float, substeps: usize) {
        // Simulate substeps
        for _ in 0..substeps {
            // Apply accelerations to the scene
            self.apply_accelerations();
            // Update scene objects
            self.update_objects(dt);
        }
    }

    // Update scene objects
    pub fn update_objects(&mut self, dt: Float) {
        // Use ODE solver to update objects
        self.ode_solver.solve(dt, &mut self.positions, &mut self.velocities, &mut self.accelerations);

        // Reset accelerations
        self.accelerations.fill(0.0);
    }

    // Apply accelerations to the scene
    pub fn apply_accelerations(&mut self) {
        // Apply gravity
        for i in 0..self.positions.len() / 2 {
            self.accelerations[i * 2 + 1] += self.gravity.y;
        }
    }
}

// Reference to a mass
#[pyclass]
pub struct MassRef {
    // Scene pointer
    scene: Py<Scene>,

    // Index of the mass in the scene vectors
    index: usize
}

// MassRef class implementation (python)
#[pymethods]
impl MassRef {
    // Set position
    fn at<'a>(self_: PyRef<'a, Self>, py: Python, position: Vec2) -> PyResult<PyRef<'a, Self>> {
        // Wrap in a block to release the borrow of scene
        {
            // Get scene
            let mut scene = self_.scene.borrow_mut(py);

            // Update position
            scene.positions[self_.index * 2] = position.0;
            scene.positions[self_.index * 2 + 1] = position.1;
        }

        // Return position
        Ok(self_)
    }
}