use particle_space::ParticleSpace;

use piston::input::*;
use opengl_graphics::GlGraphics;

pub struct Renderer {
    pub gl: GlGraphics,
    pub mouse_pos: [f64; 2],
    pub mouse_radius: f64
}

pub fn rgb(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, a]
}

impl Renderer {
    pub fn render(&mut self,
                  particle_space: &ParticleSpace,
                  args: &RenderArgs) {
        use graphics::*;

        let mouse_pos = self.mouse_pos;
        let mouse_radius = self.mouse_radius;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(rgb(33.0, 48.0, 72.0, 1.0), gl);

            for particle in particle_space.particles.iter() {
                let (x, y) = (args.width as f64 * particle.x,
                              args.height as f64 * particle.y);

                let circle = ellipse::circle(x, y, particle.size);
                let bright_radius = ellipse::circle(x, y, particle.size * 40.);

                let light_blue = rgb(60.0, 160.0, 220.0, 1.0);
                let white_bright = rgb(100.0, 150.0, 255.0, 0.002);

                ellipse(light_blue, circle, c.transform, gl);
                ellipse(white_bright, bright_radius, c.transform, gl);
            }


            let connections = &particle_space.connections;

            let (mx, my) = (mouse_pos[0] / args.width as f64,
                            mouse_pos[1] / args.height as f64);

            for pair in connections.iter() {
                if pair.strength > 0.0 {
                    let particle1 = &particle_space.particles[pair.idx1];
                    let particle2 = &particle_space.particles[pair.idx2];

                    if particle1.distance_to_point([mx, my]) <= mouse_radius &&
                        particle2.distance_to_point([mx, my]) <= mouse_radius {
                        let (start_x, start_y) = (particle1.x * args.width as f64,
                                                  particle1.y * args.height as f64);

                        let (end_x, end_y) = (particle2.x * args.width as f64,
                                              particle2.y * args.height as f64);

                        let line_coord = [start_x, start_y, end_x, end_y];

                        let light_blue = rgb(43.0, 86.0, 120.0, pair.strength as f32);

                        line(light_blue, 1.0, line_coord, c.transform, gl);
                    }
                }
            }
        });
    }

    pub fn set_mouse_pos(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_pos = mouse_pos;
    }
}
