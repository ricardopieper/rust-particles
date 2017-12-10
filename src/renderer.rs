use particle_space::ParticleSpace;

use piston::input::*;
use opengl_graphics::{ GlGraphics };

pub struct Renderer {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub connection_radius: f64
}
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

impl Renderer {

    pub fn render(&mut self, particle_space: &ParticleSpace, args: &RenderArgs) {
        use graphics::*;

        let circle = ellipse::circle(0.0, 0.0, 5.0);

        let radius = self.connection_radius;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for particle in particle_space.particles.iter() {

                let(x, y) = (args.width as f64 * particle.x,
                                      args.height as f64 * particle.y);

                let transform = c.transform.trans(x, y);

                ellipse(BLUE, circle, transform, gl);
            }
            let connections = particle_space.get_connections(radius);


            for pair in connections.iter() {

                let (start_x, start_y) = (pair.particle1.x * args.width as f64,
                                                  pair.particle1.y * args.height as f64);

                let (end_x, end_y) = (pair.particle2.x * args.width as f64,
                                              pair.particle2.y * args.height as f64);

                let line_coord = [start_x, start_y, end_x, end_y];

                line(RED, 1.0, line_coord, c.transform, gl);
            }
        });
    }
}
