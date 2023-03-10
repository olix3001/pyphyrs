import pyphyrs
import math

# Constants
MASS0 = 50.0
MASS1 = 1.0
TIME = 60 # 1 minute
DISTANCE = 10.0
K = 2
DT = 1/60
SUBSTEPS = 200
G = 0.2

# Convert time to secons
TIME = int(TIME / DT)

# Initialize the scene
scene = pyphyrs.Scene(gravity=(0,0))

# Add first mass
m0 = scene.mass().mass(MASS0).at((0.0, 0.0))

# Add second mass
m1 = scene.mass().mass(MASS1).at((DISTANCE, 0.0))

# Add orbital velocity to second mass
m1.vel((0.0, math.sqrt(G*MASS0/DISTANCE)))

# Add gravity
scene.add_force(pyphyrs.force.GravityForce([m0, m1], G=G))

# Simulate
result = scene.simulate(TIME, dt=DT, substeps=SUBSTEPS)

# Animate (2 times faster)
pyphyrs.visualization.animate(pyphyrs.separate_masses(result.extract_data()), dt=DT/2)

# Plot
pyphyrs.plots.full_plot(result, [m0, m1], mass_index=1)

# Export
result.save_csv('orbit_result.csv')