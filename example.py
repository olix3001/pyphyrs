import pyphyrs
import pyphyrs.visualization as vis
import math

# Parameters
DISTANCE = 5.0
MASS = 1.0
ORIGIN = (0.0, 0.0)
TIME = 60 # seconds

DT = 1/30
SUBSTEPS = 500

# Convert time to secons
TIME = int(TIME / DT)

# Initial velocities
velocities = [
    (0.0, 0.0), # top left
    (0.0, 0.0), # top right
    (0.0, 0.0), # bottom
]

# Initial offsets
offsets = [
    (1.0, 1.0), # top left
    (0.0, 0.0), # top right
    (-1.0, 0.0), # bottom
]

# Initialize the scene
scene = pyphyrs.Scene(gravity=(0, 0))

# Add the masses
masses = []
for v in velocities:
    m = scene.mass().mass(MASS).vel(v)
    masses.append(m)

def offset(t1, t2):
    return (t1[0]+t2[0], t1[1]+t2[1])

# Calculate distance so that length between masses is DISTANCE
DDISTANCE = DISTANCE / math.sqrt(3)

# Position the masses
masses[0].at_angle_pos(offset(ORIGIN, offsets[0]), 150, DDISTANCE, deg=True)
masses[1].at_angle_pos(offset(ORIGIN, offsets[1]), 30, DDISTANCE, deg=True)
masses[2].at_angle_pos(offset(ORIGIN, offsets[2]), -90, DDISTANCE, deg=True)

# Calculate default positions for the masses (without offsets)
m0d = (ORIGIN[0] + DDISTANCE * math.cos(math.radians(150)), ORIGIN[1] + DDISTANCE * math.sin(math.radians(150)))
m1d = (ORIGIN[0] + DDISTANCE * math.cos(math.radians(30)), ORIGIN[1] + DDISTANCE * math.sin(math.radians(30)))
m2d = (ORIGIN[0] + DDISTANCE * math.cos(math.radians(-90)), ORIGIN[1] + DDISTANCE * math.sin(math.radians(-90)))

# Add anchors
anchors = []
anchors.append(scene.mass().mass(0).at_angle_pos(m0d, 90+45, DISTANCE, deg=True))
anchors.append(scene.mass().mass(0).at_angle_pos(m1d, 45, DISTANCE, deg=True))
anchors.append(scene.mass().mass(0).at_angle_pos(m2d, -90, DISTANCE, deg=True))

# Add the springs
def spring(m1, m2, k):
    scene.add_force(pyphyrs.force.SpringForce(m1, m2, k, DISTANCE))

spring(masses[0], anchors[0], 1.0)
spring(masses[1], anchors[1], 1.0)
spring(masses[2], anchors[2], 1.0)

spring(masses[0], masses[1], 1.0)
spring(masses[1], masses[2], 1.0)
spring(masses[2], masses[0], 1.0)

# Run the simulation
result = scene.simulate(TIME, substeps=SUBSTEPS, dt=DT)

# Export the result
result.save_csv('example_result.csv')

# Animate the result (10 times faster)
vis.animate(pyphyrs.separate_masses(result.extract_data()), dt=1/(30*10))

# Plot the result
pyphyrs.plots.full_plot(result, masses)