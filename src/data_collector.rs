use std::fs::File;

// External imports
use nalgebra::DVector;

// PyO3 imports
use pyo3::{prelude::*, types::{PyDict, PyList}, intern};

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

    energies: Vec<Float>,

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
            energies: Vec::new(),
            masses: DVector::zeros(0),
        }
    }

    pub(crate) fn collect_frame(&mut self, py: Python, scene: &Py<Scene>, time: Float, energy: Float) {
        // Borrow scene
        let scene = scene.borrow(py);

        // Collect data
        self.time.push(time);
        self.positions.push(scene.positions.clone());
        self.velocities.push(scene.velocities.clone());
        self.energies.push(energy);

        // Set masses if not set yet
        if self.masses.is_empty() {
            self.masses = scene.masses.clone();
        }
    }
}

// Internal methods
impl InMemoryDataCollector {
    pub fn _get_byte_size(&self) -> usize {
        // Get size of time
        let time_size = self.time.len() * std::mem::size_of::<Float>();

        // Get size of positions
        let positions_size = self.positions.iter().map(|v| v.len() * std::mem::size_of::<Float>()).sum::<usize>();

        // Get size of velocities
        let velocities_size = self.velocities.iter().map(|v| v.len() * std::mem::size_of::<Float>()).sum::<usize>();

        // Get size of masses
        let masses_size = self.masses.len() * std::mem::size_of::<Float>();

        // Return total size
        time_size + positions_size + velocities_size + masses_size
    }

    pub fn _get_mb_size(&self) -> Float {
        self._get_byte_size() as Float / 1024.0 / 1024.0
    }
}

// Python interface
#[pymethods]
impl InMemoryDataCollector {

    // ====< Getter methods >====
    fn extract_data(&self, py: Python) -> PyResult<PyObject> {
        // Create dictionary
        let dict = PyDict::new(py);

        // Add data
        dict.set_item(intern!(py, "time"), self.time.clone())?;
        dict.set_item(intern!(py, "positions"), self.extract_positions(py)?)?;
        dict.set_item(intern!(py, "velocities"), self.extract_velocities(py)?)?;
        dict.set_item(intern!(py, "masses"), self.masses.as_slice())?;
        dict.set_item(intern!(py, "energies"), self.energies.clone())?;

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
            for _j in 0..self.masses.len() {
                list2.append((positions.next().unwrap(), positions.next().unwrap()))?;
            }

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
            for _j in 0..self.masses.len() {
                list2.append((velocities.next().unwrap(), velocities.next().unwrap()))?;
            }

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
        dict.set_item(intern!(py, "masses"), self.masses.as_slice())?;
        dict.set_item(intern!(py, "positions"), self.positions_at(py, timestep)?)?;
        dict.set_item(intern!(py, "velocities"), self.velocities_at(py, timestep)?)?;
        dict.set_item(intern!(py, "energy"), self.energies[timestep])?;

        // Return dictionary
        Ok(dict.to_object(py))
    }

    fn info_of(&self, py: Python, particle: Py<MassRef>) -> PyResult<PyObject> {
        // Create dictionary
        let dict = PyDict::new(py);

        // Add data
        dict.set_item(intern!(py, "mass"), self.masses[particle.borrow(py).index])?;
        dict.set_item(intern!(py, "position"), self.positions_of(py, particle.borrow(py))?)?;
        dict.set_item(intern!(py, "velocity"), self.velocities_of(py, particle.borrow(py))?)?;

        // Return dictionary
        Ok(dict.to_object(py))
    }

    // ====< Saving methods >====
    fn save_csv(&self, path: &str) -> PyResult<()> {
        use std::io::prelude::*;

        // Create file
        let mut file = File::create(path)?;

        // Write headers
        file.write_all(b"time,ID,m,x,y,dx,dy\n")?;

        // Write data
        for (i, time) in self.time.iter().enumerate() {
            // Write data
            for (j, mass) in self.masses.iter().enumerate() {
                // Write data
                file.write_all(format!("{},{},{},{},{},{},{}\n", 
                    time, 
                    j, 
                    mass, 
                    self.positions[i][2 * j], 
                    self.positions[i][2 * j + 1], 
                    self.velocities[i][2 * j], 
                    self.velocities[i][2 * j + 1]
                ).as_bytes())?;
            }
        }

        // Close file
        file.flush()?;

        // Return
        Ok(())
    }
}