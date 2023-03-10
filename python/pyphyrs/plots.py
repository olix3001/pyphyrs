# imports
import matplotlib.pyplot as plt
from .__init__ import separate_masses

# different plots
def plot_pos_vs_time(results, mass, xlabel='Time (s)', ylabel='Position (m)', title='Position vs Time', axis=None, new_figure=True):
    """Plot position vs time"""
    if new_figure:
        plt.figure()
    
    if axis is None or axis == 'x':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][0],results['positions'])), label='x')
    if axis is None or axis == 'y':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][1],results['positions'])), label='y')
    

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)
    plt.legend()

    if new_figure:
        plt.show()

def plot_vel_vs_time(results, mass, xlabel='Time (s)', ylabel='Velocity (m/s)', title='Velocity vs Time', axis=None, new_figure=True):
    """Plot velocity vs time"""
    if new_figure:
        plt.figure()
    
    if axis is None or axis == 'x':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][0],results['velocities'])), label='x')
    if axis is None or axis == 'y':
        plt.plot(results['time'], list(map(lambda e: e[mass.index][1],results['velocities'])), label='y')
    

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)
    plt.legend()

    if new_figure:
        plt.show()

def plot_pos_scatter(results, masses, xlabel='x', ylabel='y', title='Position Scatter', new_figure=True):
    """Plot position scatter"""
    if new_figure:
        plt.figure()
  
    for mass in masses:
        plt.scatter(list(map(lambda e: e[mass.index][0],results['positions'])), list(map(lambda e: e[mass.index][1],results['positions'])), label='x')

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)

    if new_figure:
        plt.show()

def plot_energy_vs_time(results, masses, xlabel='Time (s)', ylabel='Energy (J)', title='Energy vs Time', new_figure=True):
    """Plot energy vs time"""
    if new_figure:
        plt.figure()
    
    plt.plot(results['time'], results['energies'], label='total')

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)

    plt.title(title)
    plt.legend()

    if new_figure:
        plt.show()


# Plot everything
def full_plot(results, masses, mass_index=0, extract=True, axis=None):
    """Plot everything"""
    plt.figure()

    if extract:
        results = results.extract_data()
    
    # plot position vs time
    plt.subplot(2,2,1)
    plot_pos_vs_time(results, masses[mass_index], new_figure=False, axis=axis)
    plt.title('Position vs Time')

    # plot velocity vs time
    plt.subplot(2,2,2)
    plot_vel_vs_time(results, masses[mass_index], new_figure=False, axis=axis)
    plt.title('Velocity vs Time')

    # plot position scatter
    plt.subplot(2,2,3)
    plot_pos_scatter(results, [masses[mass_index]], new_figure=False)
    plt.title('Position Scatter')

    # plot energy vs time
    plt.subplot(2,2,4)
    plot_energy_vs_time(results, masses, new_figure=False)
    plt.title('Energy vs Time')

    plt.show()