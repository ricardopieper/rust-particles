extern crate rand;

use particle::Particle;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use connections::*;

pub struct ParticleSpace {
    pub particles: Vec<Particle>
}

impl ParticleSpace {

    pub fn new(amount_particles: i32,
               particle_max_speed: f64) -> ParticleSpace {

        let mut particles: Vec<Particle> = vec![];
        let mut rng = rand::thread_rng();

        let range = Range::new(particle_max_speed / 10.0, particle_max_speed);

        for _ in 0 .. amount_particles {

            let particle = Particle::new(
                rng.next_f64(),
                rng.next_f64(),
                range.ind_sample(&mut rng)
            );

            particles.push(particle);

        }

        ParticleSpace { particles }
    }

    pub fn process_movement(&mut self) {
        let particles = &mut self.particles;

        for particle in particles.iter_mut() {
            particle.process_movement()
        }
    }

    pub fn get_connections<'a>(&self, radius: f64) -> Vec<ConnectedParticles> {
        let connections = find_connections(radius, self);
        connections
    }
}