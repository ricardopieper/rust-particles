extern crate rand;

use particle::Particle;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use connections::ConnectionStatus;

pub struct ParticleSpace {
    pub particles: Vec<Particle>,
    pub connections: Vec<ConnectionStatus>
}

impl ParticleSpace {
    pub fn create_particles(amount_particles: i32, particle_max_speed: f64) -> Vec<Particle> {
        let mut rng = rand::thread_rng();
        let range = Range::new(particle_max_speed / 10.0, particle_max_speed);

        let mut particles: Vec<Particle> = vec![];
        for _ in 0..amount_particles {
            let particle = Particle::new(
                rng.next_f64(),
                rng.next_f64(),
                range.ind_sample(&mut rng)
            );

            particles.push(particle);
        }
        particles
    }


    pub fn new(particles: Vec<Particle>) -> ParticleSpace {
        let mut connections: Vec<ConnectionStatus> = vec![];

        for idx1 in 0..particles.len() {
            for idx2 in idx1 + 1..particles.len() {
                let connection = ConnectionStatus::new(idx1, idx2);
                connections.push(connection)
            }
        }

        ParticleSpace { connections, particles }
    }

    pub fn process_movement(&mut self) {
        let particles = &mut self.particles;

        for particle in particles.iter_mut() {
            particle.process_movement()
        }
    }

    pub fn update_connections(&mut self, radius: f64) {
        for pair in self.connections.iter_mut() {
            let particle1 = &self.particles[pair.idx1];
            let particle2 = &self.particles[pair.idx2];

            let distance = Particle::distance_between(particle1, particle2);

            if distance < radius {
                if pair.strength < 1.0 {
                    pair.strength += distance / 60.0;
                }
            } else {
                if pair.strength > 0.0 {
                    pair.strength -= distance / 60.0;
                }
            }
        }
    }
}