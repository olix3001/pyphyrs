import pyphyrs

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
results = scene.simulate(60, 500, 1/10)