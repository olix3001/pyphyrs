import pyphyrs

# Constants
MASS = 1.0
TIME = 10 # 1 minute
DISTANCE = 5.0
K = 2
DT = 1/60
SUBSTEPS = 200

# Convert time to secons
TIME = int(TIME / DT)

# Initial offsets
offsets = [
    0.0, # left
    3.0, # right
]

# Initialize the scene
scene = pyphyrs.Scene(gravity=(0,0))

# Add anchors
anchor_left = scene.mass().mass(0).at((-DISTANCE*1.5, 0.0))
anchor_right = scene.mass().mass(0).at((DISTANCE*1.5, 0.0))

# Add the masses
m0 = scene.mass().mass(MASS).at((-DISTANCE*.5 + offsets[0], 0.0))
m1 = scene.mass().mass(MASS).at((DISTANCE*.5 + offsets[1], 0.0))

# Add the springs
scene.add_force(pyphyrs.force.SpringForce(m0, anchor_left, K, DISTANCE))
scene.add_force(pyphyrs.force.SpringForce(m1, anchor_right, K, DISTANCE))

scene.add_force(pyphyrs.force.SpringForce(m0, m1, K, DISTANCE))


# Create array
masses = [anchor_left, m0, m1, anchor_right]

# Simulate
result = scene.simulate(TIME, dt=DT, substeps=SUBSTEPS)

# Animate (2 times faster)
pyphyrs.visualization.animate(pyphyrs.separate_masses(result.extract_data()), dt=DT/2)

# Plot
pyphyrs.plots.plot_pos_vs_time(result.extract_data(), m1, axis='x')