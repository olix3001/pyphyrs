# Double pendulum with springs
import pyphyrs

# Constants
MASS = 1.0
ORIGIN = (0.0, 5.0)
TIME = 60 # 1 minute
DISTANCE = 3.0
K = 20

DT = 1/60
SUBSTEPS = 200

# Convert time to secons
TIME = int(TIME / DT)

# Initial velocities
velocities = [
    (0.0, 0.0), # top
    (0.0, 0.0), # bottom
]

# Initial offsets
offsets = [
    (0.0, 0.0), # top
    (-2.0, 1.0), # bottom
]

# Initialize the scene
scene = pyphyrs.Scene()

# Add anchor
anchor = scene.mass().mass(0).at(ORIGIN)

# Add the masses
m0 = scene.mass().mass(MASS).vel(velocities[0]).at((offsets[0][0] + ORIGIN[0], offsets[0][1]-DISTANCE + ORIGIN[1]))
m1 = scene.mass().mass(MASS).vel(velocities[1]).at((offsets[1][0] + ORIGIN[0], offsets[1][1]-2*DISTANCE + ORIGIN[1]))

# Add the springs
scene.add_force(pyphyrs.force.SpringForce(m0, anchor, K, DISTANCE))
scene.add_force(pyphyrs.force.SpringForce(m1, m0, K, DISTANCE))

# Simulate
result = scene.simulate(TIME, dt=DT, substeps=SUBSTEPS)

# Animate (10 times faster)
pyphyrs.visualization.animate(pyphyrs.separate_masses(result.extract_data()), dt=DT/10)

# Plot
pyphyrs.plots.full_plot(result, [m0, m1])

# Export
result.save_csv('test_result.csv')
