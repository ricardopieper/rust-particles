use particle_space::ParticleSpace;

use piston::input::*;
use opengl_graphics::GlGraphics;

pub struct Renderer {
    pub gl: GlGraphics
}

pub fn rgb(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, a]
}

impl Renderer {
    pub fn render(&mut self,
                  particle_space: &ParticleSpace,
                  args: &RenderArgs) {
        use graphics::*;

        let circle = ellipse::circle(0.0, 0.0, 5.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(rgb(4.0, 0.0, 46.0, 1.0), gl);

            for particle in particle_space.particles.iter() {
                let (x, y) = (args.width as f64 * particle.x,
                              args.height as f64 * particle.y);

                let transform = c.transform.trans(x, y);

                let light_blue = rgb(180.0, 180.0, 255.0, 1.0);
                ellipse(light_blue, circle, transform, gl);
            }


            let connections = &particle_space.connections;


            for pair in connections.iter() {
                if pair.strength > 0.0 {
                    let particle1 = &particle_space.particles[pair.idx1];
                    let particle2 = &particle_space.particles[pair.idx2];

                    let (start_x, start_y) = (particle1.x * args.width as f64,
                                              particle1.y * args.height as f64);

                    let (end_x, end_y) = (particle2.x * args.width as f64,
                                          particle2.y * args.height as f64);

                    let line_coord = [start_x, start_y, end_x, end_y];

                    let light_blue = rgb(180.0, 180.0, 255.0, pair.strength as f32);

                    line(light_blue, 1.0, line_coord, c.transform, gl);
                }
            }
        });
    }
}
