import pygame
import pyphyrs
import time

DEFAULT_SCALE = 20

def _center(screen, pos, scale=DEFAULT_SCALE):
    return (pos[0] * scale + screen.get_width() // 2, screen.get_height() // 2 - pos[1] * scale)

def _draw_frame(screen, masses, time=0, scale=DEFAULT_SCALE, fps=60, sim_time=0):
    # Draw the frame
    screen.fill((255, 255, 255))

    # draw the grid
    for i in range(1, int(screen.get_width() / scale)):
        pygame.draw.line(screen, (200, 200, 200), (i * scale, 0), (i * scale, screen.get_height()))
        pygame.draw.line(screen, (200, 200, 200), (-i * scale, 0), (-i * scale, screen.get_height()))
    for i in range(1, int(screen.get_height() / scale)):
        pygame.draw.line(screen, (200, 200, 200), (0, i * scale), (screen.get_width(), i * scale))
        pygame.draw.line(screen, (200, 200, 200), (0, -i * scale), (screen.get_width(), -i * scale))

    # draw the axes
    pygame.draw.line(screen, (0, 0, 0), (0, screen.get_height() // 2), (screen.get_width(), screen.get_height() // 2))
    pygame.draw.line(screen, (0, 0, 0), (screen.get_width() // 2, 0), (screen.get_width() // 2, screen.get_height()))

    # draw fps
    font = pygame.font.SysFont('Arial', 20)
    fps_text = font.render(f'{fps:.2f} fps', True, (0, 0, 0))
    screen.blit(fps_text, (5, 0))

    # draw sim time
    sim_text = font.render(f'time per step {sim_time:.2f}ms', True, (0, 0, 0))
    screen.blit(sim_text, (5, 20))

    # Draw the masses
    for mass in masses:
        pygame.draw.circle(screen, (23, 23, 23), _center(screen, mass['positions'][time], scale=scale), 5*min(3, mass['mass']))

    # Draw velocity vectors
    for mass in masses:
        pos = mass['positions'][time]
        vel = mass['velocities'][time]
        pygame.draw.line(screen, (255, 0, 0), _center(screen, 
                        mass['positions'][time], scale=scale), 
                        _center(screen, (pos[0] + vel[0], pos[1] + vel[1]), scale=scale)
                    )

    # Update the screen
    pygame.display.flip()

    
def draw_single_frame(masses, scale=DEFAULT_SCALE, time=0, window_size=(800, 600)):
    # Initialize pygame
    pygame.init()
    pygame.display.set_caption('PyPhyRs Frame')

    # Create the screen
    screen = pygame.display.set_mode(window_size)

    # Draw the frame
    _draw_frame(screen, masses, scale=scale, time=time)

    # Wait for the user to close the window
    while True:
        if pygame.event.wait().type == pygame.QUIT:
            return
        
def animate(masses, scale=DEFAULT_SCALE, dt=1/24, window_size=(800, 600)):
    # Initialize pygame
    pygame.init()
    pygame.display.set_caption('PyPhyRs Animation')

    # Create the screen
    screen = pygame.display.set_mode(window_size)

    # Draw the frame
    for i in range(len(masses[0]['positions'])):
        # Wait for the user to close the window
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                return
                
        _draw_frame(screen, masses, scale=scale, time=i)
        pygame.time.wait(int(dt * 1000))

    pygame.quit()

def realtime(scene, scale=DEFAULT_SCALE, fps=24, substeps=200, dt=1/24, window_size=(800, 600)):
    # Initialize pygame
    pygame.init()
    pygame.display.set_caption('PyPhyRs Realtime Simulation')

    # Create the screen
    screen = pygame.display.set_mode(window_size)

    real_fps = fps
    time_per_sim = 1/fps

    # Draw the frame
    while True:
        frame_start = pygame.time.get_ticks()
        # Wait for the user to close the window
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                return

        # Run the simulation
        sim_start = time.time() * 1000
        results = scene.step(dt, substeps=substeps)
        sim_time = time.time() * 1000 - sim_start

        data = results.extract_data()
        masses = pyphyrs.separate_masses(data)
        
        _draw_frame(screen, masses, scale=scale, time=0, fps=real_fps, sim_time=sim_time)
        pygame.time.wait(int(1/fps * 1000))
        real_fps = 1000 / (pygame.time.get_ticks() - frame_start)
