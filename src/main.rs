mod simulation;
mod vec2_extension;

use macroquad::prelude::*;
use simulation::{Particle, SimulationConfig};
use vec2_extension::*;

const SIM_CONF: SimulationConfig = SimulationConfig::default();
const RADIUS: f32 = 4.0;

/// Creates the window configruation for Macroquad
fn window_conf() -> Conf {
    Conf {
        window_title: "SPH".to_owned(),
        window_width: 500,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}

/// Gets the difference in time from last frame.
fn delta_time() -> f32 {
    get_frame_time()
}

fn make_grid_of_particles(count: usize, top_left: Vec2, spacing: f32) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(count);
    let root = (count as f32).sqrt() as usize;
    for x in 0..root {
        let x = top_left.x + x as f32 * spacing;
        for y in 0..root {
            let y = top_left.y + y as f32 * spacing;
            particles.push(Particle::new(Vec2::new(x, y)));
        }
    }

    particles
}

fn kernel(dist: f32) -> f32 {
    if dist > SIM_CONF.smoothing_radius {
        return 0.0;
    }

    (1.0 - dist / SIM_CONF.smoothing_radius).powi(2)
}

/// Checks boundaries and adjusts the particles velocitiy accordingly.
fn resolve_boundaries(particle: &mut Particle) {
    let x = particle.position.x;
    let y = particle.position.y;

    if x < RADIUS || x > screen_width() - RADIUS {
        particle.velocity.flip_x();
        particle.position.x += particle.velocity.x * delta_time();
        particle.velocity.x *= SIM_CONF.collision_damping;
    }
    if y < RADIUS || y > screen_height() - RADIUS {
        particle.velocity.flip_y();
        particle.position.y += particle.velocity.y * delta_time();
        particle.velocity.y *= SIM_CONF.collision_damping;
    }
}

/// Applies gravitational acceleration to the particle.
fn apply_gravity(particle: &mut Particle) {
    particle.velocity += SIM_CONF.gravity * delta_time();
}

/// Moves the particle by it's velocity.
fn move_by_velocity(particle: &mut Particle) {
    particle.position += particle.velocity * delta_time();
}

fn calculate_densities(particles: &mut Vec<Particle>) {
    // TODO: Fix this later
    // Lets presume that mass is 1 for all particles
    let mass = 1.0;
    for i in 0..particles.len() {
        let pos = particles[i].position;
        particles[i].density = particles
            .iter()
            .map(|p| mass * kernel((pos - p.position).length()))
            .sum();
    }
}

fn simulate(particles: &mut Vec<Particle>) {
    calculate_densities(particles);
    particles.iter_mut().for_each(|p| {
        apply_gravity(p);

        move_by_velocity(p);
        resolve_boundaries(p);
    });
}

/// The coordinate system goes from (0, 0) = top-left to (WIDTH, HEIGHT) = bottom-right.
///
///    (0, 0) --------- (WIDTH, 0)
///      |                  |
///      |                  |
///      |                  |
///      |                  |
///  (HEIGHT, 0) --- (WIDTH, HEIGHT)
#[macroquad::main(window_conf)]
async fn main() {
    let mut particles = make_grid_of_particles(1024, Vec2::new(42.0, 42.0), 2.0 * RADIUS + 5.0);

    loop {
        clear_background(GRAY);

        // CORE
        // Simulate
        simulate(&mut particles);
        // Draw
        for p in &mut particles {
            draw_circle(p.position.x, p.position.y, RADIUS, BLUE);
        }

        println!("FPS: {}", get_fps());
        next_frame().await
    }
}
