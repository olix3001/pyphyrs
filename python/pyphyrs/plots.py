# imports
import matplotlib.pyplot as plt

# different plots
def plot_pos_vs_time(results, mass, xlabel='Time (s)', ylabel='Position (m)', title='Position vs Time', axis=None):
    """Plot position vs time"""
    plt.figure()
    
    if axis is None or axis == 'x':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][0],results['positions'])), label='x')
    if axis is None or axis == 'y':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][1],results['positions'])), label='y')
    

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)
    plt.legend()

    plt.show()

def plot_vel_vs_time(results, mass, xlabel='Time (s)', ylabel='Velocity (m/s)', title='Velocity vs Time', axis=None):
    """Plot velocity vs time"""
    plt.figure()
    
    if axis is None or axis == 'x':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][0],results['velocities'])), label='x')
    if axis is None or axis == 'y':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][1],results['velocities'])), label='y')
    

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)
    plt.legend()

    plt.show()

def plot_pos_scatter(results, masses, xlabel='x', ylabel='y', title='Position Scatter'):
    """Plot position scatter"""
    plt.figure()
  
    for mass in masses:
        plt.scatter(list(map(lambda e: e[mass.index][0],results['positions'])), list(map(lambda e: e[mass.index][1],results['positions'])), label='x')

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)

    plt.show()