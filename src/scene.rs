// External imports
use nalgebra::{Vector2, DVector};

// PyO3 imports
use pyo3::{prelude::*, intern};

// Crate imports
use crate::{Float, Vec2, solvers::{ODESolver, EulerODE}, data_collector::InMemoryDataCollector};

// Scene class definition
#[pyclass]
pub struct Scene {
    // Properties
    gravity: Vector2<Float>,

    // Objects stored as a vector where each object is a vector of its positions
    pub(crate) positions: DVector<Float>,
    pub(crate) velocities: DVector<Float>,
    pub(crate) accelerations: DVector<Float>,
    pub(crate) masses: DVector<Float>,

    // Technicals
    ode_solver: Box<dyn Send + ODESolver>,
    pub(crate) data_collector: InMemoryDataCollector,

    // Other things
    force_generators: Vec<PyObject>,
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
            masses: DVector::zeros(0),

            ode_solver: Box::new(EulerODE),
            data_collector: InMemoryDataCollector::new(),

            force_generators: Vec::new(),
        }
    }
}

// Scene class implementation (python)
#[pymethods]
impl Scene {
    // Constructor
    // TODO: Create signature for constructor
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
        self_.masses.extend([1.0]);

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

    // Add force generator
    fn add_force(&mut self, force_generator: PyObject) {
        self.force_generators.push(force_generator);
    }

    // Simulate scene
    fn simulate(self_: Py<Self>, steps: usize, substeps: usize, dt: Float, py: Python) -> PyResult<InMemoryDataCollector> {
        // Initialize data collector
        let mut data_collector = { self_.borrow(py).data_collector.clone() }; // Wrapped in braces to make sure it's dropped before the simulation begins

        // Time simulation
        let start = std::time::Instant::now();

        let mut time = 0.0;
        // Simulate scene
        for _ in 0..steps {
            let energy = Self::update(&self_, dt, substeps, py);
            data_collector.collect_frame(py, &self_, time, energy / substeps as Float);
            time += dt;
        }


        println!("Simulation took {}ms", start.elapsed().as_millis());
        println!("Each step took {}ms", start.elapsed().as_millis() / steps as u128);

        // Return data collector
        Ok(data_collector)
    }

    // Step by one frame
    fn step(self_: Py<Self>, dt: Float, substeps: usize, py: Python) -> PyResult<InMemoryDataCollector> {
        // Initialize data collector
        let mut data_collector = { self_.borrow(py).data_collector.clone() }; // Wrapped in braces to make sure it's dropped before the simulation begins

        // Time simulation
        #[cfg(feature="timings")]
        let start = std::time::Instant::now();

        // Simulate scene
        let energy = Self::update(&self_, dt, substeps, py);
        data_collector.collect_frame(py, &self_, 0.0, energy / substeps as Float);

        #[cfg(feature="timings")]
        {
            println!("Simulation took {}ms", start.elapsed().as_millis());
            println!("Each step took {}ms", start.elapsed().as_millis() / 1);
        }

        // Return data collector
        Ok(data_collector)
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
    pub fn update(self_: &Py<Self>, dt: Float, substeps: usize, py: Python) -> Float {
        // Simulate substeps
        let mut energy = 0.0;
        for _ in 0..substeps {
            // Apply accelerations to the scene
            energy += Self::apply_accelerations(self_, py);

            // Update scene objects
            {
                let mut self_mut = self_.try_borrow_mut(py).unwrap();
                energy += self_mut.update_objects(dt / substeps as Float);
            }
        }

        // Return energy
        energy
    }

    // Update scene objects
    pub fn update_objects(&mut self, dt: Float) -> Float {
        // Use ODE solver to update objects
        self.ode_solver.solve(dt, &mut self.positions, &mut self.velocities, &mut self.accelerations);

        // Calculate energy
        let mut energy = 0.0;
        #[cfg(not(feature="no-energy"))]
        for i in 0..self.positions.len() / 2 {
            if self.masses[i] == 0.0 {
                continue;
            }

            // Ek = 1/2 * m * v^2
            energy += 0.5 * self.masses[i] * (self.velocities[i * 2].powi(2) + self.velocities[i * 2 + 1].powi(2));
        }

        // Reset accelerations
        self.accelerations.fill(0.0);

        // Return energy
        energy
    }

    // Apply accelerations to the scene
    pub fn apply_accelerations(self_: &Py<Self>, py: Python) -> Float {
        // Apply gravity
        {
            let mut self_mut = self_.try_borrow_mut(py).unwrap();
            for i in 0..self_mut.positions.len() / 2 {
                if self_mut.masses[i] == 0.0 {
                    continue;
                }
                self_mut.accelerations[i * 2 + 1] += self_mut.gravity.y;
            }
        }

        // Apply force generators
        let mut energy = 0.0;
        let force_generators = self_.try_borrow(py).unwrap().force_generators.clone();
        for force_generator in force_generators.iter() {
            // Apply forces
            let result = force_generator.call_method0(py, intern!(py, "apply_force"));

            // Calculate energy
            #[cfg(not(feature="no-energy"))]
            {
                let energy_result = force_generator.call_method0(py, intern!(py, "get_energy"));
                if energy_result.is_ok() {
                    energy += energy_result.unwrap().extract::<Float>(py).unwrap();
                }
            }

            // Check for errors while applying force
            if result.is_err() {
                panic!("Error while applying force: {:?}", result);
            }
        }

        // Return energy
        energy
    }
}

// Reference to a mass
#[pyclass]
pub struct MassRef {
    // Scene pointer
    scene: Py<Scene>,

    // Index of the mass in the scene vectors
    pub(crate) index: usize
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

    // Relative position setter
    fn relative<'a>(self_: PyRef<'a, Self>, py: Python, origin: PyRef<MassRef>) -> PyResult<PyRef<'a, Self>> {
        // Wrap in a block to release the borrow of scene
        {
            // Get scene
            let mut scene = self_.scene.borrow_mut(py);

            // Update position
            scene.positions[self_.index * 2] = scene.positions[origin.index * 2];
            scene.positions[self_.index * 2 + 1] = scene.positions[origin.index * 2 + 1];
        }

        // Return position
        Ok(self_)
    }

    // Relative position setter with angle
    fn at_angle<'a>(self_: PyRef<'a, Self>, py: Python, origin: PyRef<MassRef>, angle: Float, dist: Float, deg: Option<bool>) -> PyResult<PyRef<'a, Self>> {
        // Wrap in a block to release the borrow of scene
        {
            // Get scene
            let mut scene = self_.scene.borrow_mut(py);

            let angle = if deg.unwrap_or(false) { angle.to_radians() } else { angle };
            // Update position
            scene.positions[self_.index * 2] = scene.positions[origin.index * 2] + dist * angle.cos();
            scene.positions[self_.index * 2 + 1] = scene.positions[origin.index * 2 + 1] + dist * angle.sin();
        }

        // Return position
        Ok(self_)
    }

    // Relative position setter with angle and position origin
    fn at_angle_pos<'a>(self_: PyRef<'a, Self>, py: Python, origin: (Float, Float), angle: Float, dist: Float, deg: Option<bool>) -> PyResult<PyRef<'a, Self>> {
        // Wrap in a block to release the borrow of scene
        {
            // Get scene
            let mut scene = self_.scene.borrow_mut(py);

            let angle = if deg.unwrap_or(false) { angle.to_radians() } else { angle };
            // Update position
            scene.positions[self_.index * 2] = origin.0 + dist * angle.cos();
            scene.positions[self_.index * 2 + 1] = origin.1 + dist * angle.sin();
        }

        // Return position
        Ok(self_)
    }

    // Set velocity
    fn vel<'a>(self_: PyRef<'a, Self>, py: Python, velocity: Vec2) -> PyResult<PyRef<'a, Self>> {
        // Wrap in a block to release the borrow of scene
        {
            // Get scene
            let mut scene = self_.scene.borrow_mut(py);

            // Update velocity
            scene.velocities[self_.index * 2] = velocity.0;
            scene.velocities[self_.index * 2 + 1] = velocity.1;
        }

        // Return position
        Ok(self_)
    }

    // Set mass
    fn mass<'a>(self_: PyRef<'a, Self>, py: Python, mass: Float) -> PyResult<PyRef<'a, Self>> {
        // Wrap in a block to release the borrow of scene
        {
            // Get scene
            let mut scene = self_.scene.borrow_mut(py);

            // Update mass
            scene.masses[self_.index] = mass;
        }

        // Return position
        Ok(self_)
    }

    // Position getter
    #[getter(position)]
    fn get_position(self_: PyRef<Self>, py: Python) -> PyResult<(Float, Float)> {
        // Get scene
        let scene = self_.scene.borrow(py);

        // Return position
        Ok((scene.positions[self_.index * 2], scene.positions[self_.index * 2 + 1]))
    }

    // Get distance to another mass
    fn distance_to(self_: PyRef<Self>, py: Python, other: PyRef<MassRef>) -> PyResult<Float> {
        // Get scene
        let scene = self_.scene.borrow(py);

        // Return distance
        Ok(((scene.positions[self_.index * 2] - scene.positions[other.index * 2]).powi(2) + (scene.positions[self_.index * 2 + 1] - scene.positions[other.index * 2 + 1]).powi(2)).sqrt())
    }

    // Velocity getter
    #[getter(velocity)]
    fn get_velocity(self_: PyRef<Self>, py: Python) -> PyResult<(Float, Float)> {
        // Get scene
        let scene = self_.scene.borrow(py);

        // Return velocity
        Ok((scene.velocities[self_.index * 2], scene.velocities[self_.index * 2 + 1]))
    }

    // Index getter
    #[getter(index)]
    fn get_index(self_: PyRef<Self>) -> usize {
        self_.index
    }
}

// Internal implementation of MassRef
impl MassRef {
    // Get position
    pub fn position(&self, py: Python) -> (Float, Float) {
        // Get scene
        let scene = self.scene.borrow(py);

        // Return position
        (scene.positions[self.index * 2], scene.positions[self.index * 2 + 1])
    }

    // Get velocity
    pub fn velocity(&self, py: Python) -> (Float, Float) {
        // Get scene
        let scene = self.scene.borrow(py);

        // Return velocity
        (scene.velocities[self.index * 2], scene.velocities[self.index * 2 + 1])
    }

    // Apply force
    pub fn apply_force(&self, py: Python, force: Vec2) {
        // Get scene
        let mut scene = self.scene.borrow_mut(py);

        // Apply force (if mass is not zero)
        if scene.masses[self.index] != 0.0 {
            scene.accelerations[self.index * 2] += force.0 / scene.masses[self.index];
            scene.accelerations[self.index * 2 + 1] += force.1 / scene.masses[self.index];
        }
    }
}

// Native implementation of MassRef
impl MassRef {
    // Get position
    pub fn raw_position(&self, py: Python) -> Vector2<Float> {
        let scene = self.scene.borrow(py);
        // Return position
        Vector2::new(scene.positions[self.index * 2], scene.positions[self.index * 2 + 1])
    }

    // Get velocity
    pub fn raw_velocity(&self, py: Python) -> Vector2<Float> {
        let scene = self.scene.borrow(py);
        // Return velocity
        Vector2::new(scene.velocities[self.index * 2], scene.velocities[self.index * 2 + 1])
    }

    // Apply force
    pub fn raw_apply_force(&self, py: Python, force: Vector2<Float>) {
        let mut scene = self.scene.borrow_mut(py);
        // Check if force is NaN
        if force.x.is_nan() || force.y.is_nan() {
            panic!("Force is NaN");
        }

        // Apply force (if mass is not zero)
        if scene.masses[self.index] == 0.0 {
            return;
        }
        scene.accelerations[self.index * 2] += force.x / scene.masses[self.index];
        scene.accelerations[self.index * 2 + 1] += force.y / scene.masses[self.index];
    }
}