// External imports
use nalgebra::DVector;

// PyO3 imports
use pyo3::{prelude::*, types::{PyDict, PyList}};

// Crate imports
use crate::{scene::{Scene, MassRef}, Float};

// InMemoryDataCollector implementation
#[pyclass(name = "InMemoryDataCollector")]
#[derive(Clone)]
pub(crate) struct InMemoryDataCollector {
    // Data
    time: Vec<Float>,
    positions: Vec<DVector<Float>>,
    velocities: Vec<DVector<Float>>,

    // Static data
    masses: DVector<Float>,
}

// Constructor
impl InMemoryDataCollector {
    pub(crate) fn new() -> Self {
        Self {
            time: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            masses: DVector::zeros(0),
        }
    }

    pub(crate) fn collect_frame(&mut self, py: Python, scene: &Py<Scene>, time: Float) {
        // Borrow scene
        let scene = scene.borrow(py);

        // Collect data
        self.time.push(time);
        self.positions.push(scene.positions.clone());
        self.velocities.push(scene.velocities.clone());

        // Set masses if not set yet
        if self.masses.is_empty() {
            self.masses = scene.masses.clone();
        }
    }
}

// Python interface
#[pymethods]
impl InMemoryDataCollector {
    fn extract_data(&self, py: Python) -> PyResult<PyObject> {
        // Create dictionary
        let dict = PyDict::new(py);

        // Add data
        dict.set_item("time", self.time.clone())?;
        dict.set_item("positions", self.extract_positions(py)?)?;
        dict.set_item("velocities", self.extract_velocities(py)?)?;
        dict.set_item("masses", self.masses.as_slice())?;

        // Return dictionary
        Ok(dict.to_object(py))
    }

    fn extract_positions(&self, py: Python) -> PyResult<PyObject> {
        // Create list
        let list = PyList::empty(py);

        // Add data
        for (_i, positions) in self.positions.iter().enumerate() {
            // Collect positions into tuples of (x, y)
            let mut positions = positions.iter();

            // Create list
            let list2 = PyList::empty(py);

            // Add positions
            list2.append((positions.next().unwrap(), positions.next().unwrap()))?;

            // Add list to list
            list.append(list2)?;
        }

        // Return list
        Ok(list.to_object(py))
    }

    fn extract_velocities(&self, py: Python) -> PyResult<PyObject> {
        // Create list
        let list = PyList::empty(py);

        // Add data
        for (_i, velocities) in self.velocities.iter().enumerate() {
            // Collect velocities into tuples of (x, y)
            let mut velocities = velocities.iter();

            // Create list
            let list2 = PyList::empty(py);

            // Add velocities
            list2.append((velocities.next().unwrap(), velocities.next().unwrap()))?;

            // Add list to list
            list.append(list2)?;
        }

        // Return list
        Ok(list.to_object(py))
    }

    fn positions_at(&self, py: Python, timestep: usize) -> PyResult<PyObject> {
        // Create list
        let list = PyList::empty(py);

        // Add data
        let positions = &self.positions[timestep];
        let mut positions = positions.iter();

        // Add positions
        list.append((positions.next().unwrap(), positions.next().unwrap()))?;

        // Return list
        Ok(list.to_object(py))
    }

    fn positions_of(&self, py: Python, particle: PyRef<MassRef>) -> PyResult<PyObject> {
        // Create list
        let list = PyList::empty(py);

        // Add data
        for (_i, positions) in self.positions.iter().enumerate() {
            // Collect positions into tuples of (x, y)
            let mut positions = positions.iter();

            // Skip particles
            for _ in 0..particle.index {
                positions.next();
            }

            // Add positions
            list.append((positions.next().unwrap(), positions.next().unwrap()))?;
        }

        // Return list
        Ok(list.to_object(py))
    }

    fn velocities_at(&self, py: Python, timestep: usize) -> PyResult<PyObject> {
        // Create list
        let list = PyList::empty(py);

        // Add data
        let velocities = &self.velocities[timestep];
        let mut velocities = velocities.iter();

        // Add velocities
        list.append((velocities.next().unwrap(), velocities.next().unwrap()))?;

        // Return list
        Ok(list.to_object(py))
    }

    fn velocities_of(&self, py: Python, particle: PyRef<MassRef>) -> PyResult<PyObject> {
        // Create list
        let list = PyList::empty(py);

        // Add data
        for (_i, velocities) in self.velocities.iter().enumerate() {
            // Collect velocities into tuples of (x, y)
            let mut velocities = velocities.iter();

            // Skip particles
            for _ in 0..particle.index {
                velocities.next();
            }

            // Add velocities
            list.append((velocities.next().unwrap(), velocities.next().unwrap()))?;
        }

        // Return list
        Ok(list.to_object(py))
    }

    fn info_at(&self, py: Python, timestep: usize) -> PyResult<PyObject> {
        // Create dictionary
        let dict = PyDict::new(py);

        // Add data
        dict.set_item("positions", self.positions_at(py, timestep)?)?;
        dict.set_item("velocities", self.velocities_at(py, timestep)?)?;

        // Return dictionary
        Ok(dict.to_object(py))
    }

    fn info_of(&self, py: Python, particle: Py<MassRef>) -> PyResult<PyObject> {
        // Create dictionary
        let dict = PyDict::new(py);

        // Add data
        dict.set_item("mass", self.masses[particle.borrow(py).index])?;
        dict.set_item("position", self.positions_of(py, particle.borrow(py))?)?;
        dict.set_item("velocity", self.velocities_of(py, particle.borrow(py))?)?;

        // Return dictionary
        Ok(dict.to_object(py))
    }

}