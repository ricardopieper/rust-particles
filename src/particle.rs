extern crate rand;

use rand::Rng;
use point_2d::Point2D;

pub struct Particle {
    pub pos: Point2D,
    pub size: f64,
    speed_x: f64,
    speed_y: f64,
}


impl Particle {
    pub fn new(x: f64, y: f64, max_particle_speed: f64) -> Particle {
        let mut rng = rand::thread_rng();

        let horizontal_speed_modifier = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let vertical_speed_modifier = if rng.gen::<bool>() { 1.0 } else { -1.0 };
        let size = rng.gen_range(0.5, 3.0);

        let particle_speed = (size / 3.0) * max_particle_speed;


        Particle {
            pos: Point2D {x, y},
            size,
            speed_x: horizontal_speed_modifier * particle_speed,
            speed_y: vertical_speed_modifier * particle_speed
        }
    }

    pub fn process_movement(&mut self) {
        self.pos.x += self.speed_x;
        self.pos.y += self.speed_y;

        if self.pos.x >= 1.0 || self.pos.x <= 0.0 {
            self.speed_x *= -1.0;
        }

        if self.pos.y >= 1.0 || self.pos.y <= 0.0 {
            self.speed_y *= -1.0;
        }
    }
}