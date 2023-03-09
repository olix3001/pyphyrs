# re-export the main classes
from .pyphyrs import *

# export plots
from . import plots

# export visualizer
from . import visualization

# utils
def separate_masses(data):
    """Converts results into a list of values for each mass"""
    masses = []
    for i in range(len(data['positions'][0])):
        masses.append({})
        # add positions
        masses[i]['positions'] = list(map(lambda e: e[i], data['positions']))
        # add velocities
        masses[i]['velocities'] = list(map(lambda e: e[i], data['velocities']))
        # add mass
        masses[i]['mass'] = data['masses'][i]
    return masses