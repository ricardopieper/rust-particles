use particle_space::ParticleSpace;
use particle::Particle;

pub struct ConnectedParticles<'a> {
    pub particle1: &'a Particle,
    pub particle2: &'a Particle
}

impl<'a> ConnectedParticles <'a> {
    fn new(particle1: &'a Particle, particle2: &'a Particle) -> ConnectedParticles<'a> {
        ConnectedParticles {
            particle1, particle2
        }
    }
}

pub fn find_connections(connection_distance: f64, particle_space: &ParticleSpace) -> Vec<ConnectedParticles> {

    let mut connections: Vec<ConnectedParticles> = vec![];

    for p1 in particle_space.particles.iter() {
        for p2 in particle_space.particles.iter() {
            if p1 != p2 && p1.is_connected_with(p2, connection_distance) {
                let connection = ConnectedParticles::new(p1, p2);
                connections.push(connection)
            }
        }
    }

    connections
}



