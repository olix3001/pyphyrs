import pyphyrs
from pkgutil import iter_modules

# Print the version of the library
print(pyphyrs.__version__)

# Create new scene
scene = pyphyrs.Scene()

# Add masses to the scene
m0 = scene.mass().at((5, 0)).vel((1, 0))
m1 = scene.mass().at((-5, 0)).vel((-1, 0))

# Add a spring to the scene
scene.add_force(pyphyrs.force.SpringForce(m0, m1))

# run simulation
scene.simulate(60, 200, 1/10)

# Print the position of the masses
print(m0.position)
print(m1.position)