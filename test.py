import pyphyrs
from pyphyrs import plots
from pyphyrs import visualization as vis
import math

# Create new scene
scene = pyphyrs.Scene(gravity=(0, 0))

# 50 masses connected all together in a circle
masses = []
for i in range(50):
    masses.append(scene.mass()
        .at((
                math.cos(i*2*math.pi/50) * 100,
                math.sin(i*2*math.pi/50) * 100
            )))

# Connect all masses together
for i in range(len(masses)):
    for j in range(len(masses)):
        if i != j:
            scene.add_force(pyphyrs.force.SpringForce(masses[i], masses[j], k=2))

# Add velocity to the first mass
masses[0].vel((15, 0))

# Set first mass to be bigger than the others
masses[0].mass(30)

# Run the simulation in real time
# vis.realtime(scene, fps=60, dt=1/30, substeps=250, scale=2)

# Simulate and then run
result = scene.simulate(200, substeps=250, dt=1/30)

# save
result.save_csv('test.csv')

# animate
vis.animate(pyphyrs.separate_masses(result.extract_data()), scale=2, dt=1/30)