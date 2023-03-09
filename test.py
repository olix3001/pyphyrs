import pyphyrs

# Print the version of the library
print(pyphyrs.__version__)

# Create new scene
scene = pyphyrs.Scene()

# Add mass to the scene
m0 = scene.mass().at((10, 0))

# print positions
print(scene.positions())

# run simulation
scene.simulate(60, 200, 1/10)

# print positions
print(scene.positions())