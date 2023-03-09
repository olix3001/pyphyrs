import pyphyrs
import random

# Print the version of the library
print(pyphyrs.__version__)

# Create new scene
scene = pyphyrs.Scene()

# Add masses to the scene with initial position and velocity (connected by springs)
pm = None
pos = 0
for i in range(50):
    m = scene.mass().at((pos, 0)).vel((random.randint(-1, 1), 0))
    if pm is not None:
        scene.add_force(pyphyrs.force.SpringForce(pm, m))
    pm = m
    pos += 10

# run simulation
scene.simulate(60, 500, 1/10)

# Print the position of the masses
print(scene.positions())