import pyphyrs
from pyphyrs import visualization as vis
import math
import matplotlib.pyplot as plt
import random

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
masses[0].vel((15, -4))

# Add velocity to the random mass
rm = masses[random.randint(0, 49)] # random mass
rm.vel((random.randint(-10, 10), random.randint(-10, 10)))
# set its mass to be bigger
rm.mass(30)

# Set first mass to be bigger than the others
masses[0].mass(30)

# Run the simulation in real time
# vis.realtime(scene, fps=60, dt=1/30, substeps=250, scale=2)

# Simulate and then run
result = scene.simulate(250, substeps=200, dt=1/30)

# save
result.save_csv('test.csv')

# animate
vis.animate(pyphyrs.separate_masses(result.extract_data()), scale=2, dt=1/30)

# plot
plt.figure()

plt.subplot(2, 2, 1)
pyphyrs.plots.plot_pos_vs_time(result.extract_data(), masses[0], new_figure=False)
plt.subplot(2, 2, 2)
pyphyrs.plots.plot_vel_vs_time(result.extract_data(), masses[0], new_figure=False)

# plot scatter
plt.subplot(2, 2, 3)
pyphyrs.plots.plot_pos_scatter(result.extract_data(), masses, new_figure=False)

# plot energy
plt.subplot(2, 2, 4)
pyphyrs.plots.plot_energy_vs_time(result.extract_data(), masses, new_figure=False)

plt.show()
