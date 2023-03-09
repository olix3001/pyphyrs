import pyphyrs
import random

# Print the version of the library
print(pyphyrs.__version__)

# Create new scene
scene = pyphyrs.Scene()

# Add masses to the scene
m0 = scene.mass().at((10, 0)).vel((1, 10))
m1 = scene.mass().at((0, 0))

# run simulation
scene.simulate(60, 200, 1/10)

# Print the position of the masses
print(m0.position)
print(m1.position)