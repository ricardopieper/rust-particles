extern crate rand;

use rand::Rng;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    speed_x: f64,
    speed_y: f64,
}


impl Particle {
    pub fn new(x: f64, y: f64, particle_speed: f64) -> Particle {
        let mut rng = rand::thread_rng();

        let horizontal_speed_modifier = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let vertical_speed_modifier = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let size = rng.gen_range(1.5, 3.0);

        Particle {
            x,
            y,
            size,
            speed_x: horizontal_speed_modifier * particle_speed,
            speed_y: vertical_speed_modifier * particle_speed
        }
    }

    pub fn process_movement(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if self.x >= 1.0 || self.x <= 0.0 {
            self.speed_x *= -1.0;
        }

        if self.y >= 1.0 || self.y <= 0.0 {
            self.speed_y *= -1.0;
        }
    }

    pub fn distance_between(particle1: &Particle, particle2: &Particle) -> f64 {
        let distances = (particle1.x - particle2.x,
                         particle1.y - particle2.y);

        let distances_sqr = distances.0.powi(2) + distances.1.powi(2);

        distances_sqr.sqrt()
    }
}