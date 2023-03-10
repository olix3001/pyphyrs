# PyPhyRs (W.I.P.) :rocket:

PyPhyRs is a library for physics simulation in python. It can perform simulations at realtime or precalculate them.

## Functions

-   [x] Simple masses
-   [x] Springs
-   [x] Custom force generators
-   [x] Plots
-   [x] Realtime simulation and result animation

## How to use?

### Basic mass

Firstly, import pyphyrs library, then create new scene like below:

```py
import pyphyrs

scene = pyphyrs.Scene()
```

You can also specify it's gravity using `Scene(gravity=(x, y))` where x and y is acceleration (use `(0, -9.81)` for earth-like gravity).

Once you have your scene you probably want to add something to it, tou can do this using `.mass` method. For example:

```py
import pyphyrs

scene = pyphyrs.Scene()

mass = scene.mass()
```

This will create mass at position (0, 0) with initial velocity (0, 0). You can change these parameters using `.at((x, y))` and `.vel((x, y))` methods. (there are more methods)

### Force

You can want to have some forces act between objects in your simulation, an example of such force is a spring. You can add spring using `.add_force` method with generator from `pyphyrs.force` or your custom one.

```py
import pyphyrs

# Scene initialization goes here

# Crating masses
m0 = scene.mass().at((-1, 0)).vel((-1, 0))
m1 = scene.mass().at((1, 0)).vel((1, 0))

# Adding a spring
scene.add_force(pyphyrs.force.SpringForce(m0, m1))
```

#### Supported forces

| class       | description                                                   | arguments                                                             | impl                |
| ----------- | ------------------------------------------------------------- | --------------------------------------------------------------------- | ------------------- |
| SpringForce | Acts like a spring between objects you pass in a constructor. | required: m0, m1 (masses); optional: k (spring constant), rest_length | Native :heart_eyes: |

### Simulating without render

Last step is to run your simulation. It's as simple as everything else!
For example to simulate 2 seconds with delta time between each frame being 1/30s using 200 substeps (Don't use too many substeps as it can decrease precision) just use:

```py
# scene initialization

result = scene.simulate(steps=60, substeps=200, dt=1/30)
```

### Plotting

For plotting functions visit [plots.py](python/pyphyrs/plots.py)

### Visualization (Only masses at this time)

Once your simulation has finished you can view the result using `visualization` submodule. For example to show your simulation results you can use:

```py
from pyphyrs import visualization as vis

# scene setup

result = scene.simulate(steps=240, substeps=200, dt=1/30) # 8 seconds

vis.animate(result, dt=1/30)
```

If you don't need results for other things you can also simulate in realtime using `pyphyrs.visualization.realtime` method:

```py
from pyphyrs import visualization as vis

# scene setup

vis.realtime(scene, fps=30, dt=1/10) # dt is constant to avoid calculation mistakes
```

## Speed

It's fast :red_car:

Simulation of 50 masses connected in a circle all together (2450 springs) takes about 100ms per frame (with 250 substeps).

# Installation

Just install wheel you need from github releases or actions.

## Building on your own

### Setup

You can also build this library on your own, to do so, you'll need to have python and cargo installed on your system. Once you have them, install maturin using `pip install maturin` and create new virtual environment using `python -m venv .env` (.env is a name so It can be changed). To enable venv run script from `.env/Scripts` appropriate for your os (for example `Activate.bat` for windows) using command line.

### Building

Once everyting is prepared just run `python -m maturin develop --release` to build and install library in your virtual environment or `python -m maturin build --release` to build wheel for python.
