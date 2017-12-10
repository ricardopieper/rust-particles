use particle_space::ParticleSpace;

use piston::input::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use particle::Particle;
use point_2d::Point2D;
use connections::ConnectionStatus;

pub struct Renderer {
    pub gl: GlGraphics,
    pub mouse_pos: Point2D,
    pub mouse_radius: f64
}

pub fn rgb(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, a]
}

impl Renderer {
    pub fn render(&mut self,
                  particle_space: &ParticleSpace,
                  args: &RenderArgs) {
        let mouse_pos = &self.mouse_pos;
        let mouse_radius = self.mouse_radius;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(rgb(33.0, 48.0, 72.0, 1.0), gl);

            Renderer::render_particles(particle_space.particles.as_slice(), args, &c, gl);

            Renderer::render_connections(particle_space.particles.as_slice(),
                                         particle_space.connections.as_slice(),
                                         args, mouse_pos, mouse_radius, &c, gl);
        });
    }

    fn render_particles(particles: &[Particle], args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
        for particle in particles.iter() {
            Renderer::render_single_particle(particle, args, c, gl);
        }
    }

    fn render_single_particle(particle: &Particle, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
        let position = particle.pos.scale_to_args(args);
        let circle = ellipse::circle(position.x, position.y, particle.size);
        let light_blue = rgb(60.0, 160.0, 220.0, 1.0);
        ellipse(light_blue, circle, c.transform, gl);
    }


    fn render_connections(particles: &[Particle], connections: &[ConnectionStatus],
                          args: &RenderArgs, mouse_pos: &Point2D, mouse_radius: f64,
                          c: &Context, gl: &mut GlGraphics) {
        let canonical_mouse_pos = Renderer::get_canonical_mouse_pos(mouse_pos, args);

        for pair in connections.iter() {
            if Renderer::should_render_connection(pair, particles,
                                                  &canonical_mouse_pos, mouse_radius) {
                Renderer::render_single_connection(pair, particles, args, c, gl);
                Renderer::render_particle_shadows(pair, particles, args, c, gl);
            }
        }
    }

    fn should_render_connection(pair: &ConnectionStatus, particles: &[Particle],
                                mouse_pos: &Point2D, mouse_radius: f64) -> bool {
        pair.strength > 0.0 && Renderer::is_within_mouse_range(pair, particles,
                                                               mouse_pos, mouse_radius)
    }

    fn render_single_connection(pair: &ConnectionStatus, particles: &[Particle],
                                args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
        let particle1: &Particle = &particles[pair.idx1];
        let particle2: &Particle = &particles[pair.idx2];

        let start = particle1.pos.scale_to_args(args);
        let end = particle2.pos.scale_to_args(args);

        let line_coordinates = [start.x, start.y, end.x, end.y];
        let light_blue = rgb(43.0, 86.0, 120.0, pair.strength as f32);
        line(light_blue, 1.0, line_coordinates, c.transform, gl);
    }


    fn render_particle_shadows(pair: &ConnectionStatus, particles: &[Particle], args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
        Renderer::render_shadow_for_particle(&particles[pair.idx1], args, pair.strength, c, gl);
        Renderer::render_shadow_for_particle(&particles[pair.idx2], args, pair.strength, c, gl);
    }

    fn render_shadow_for_particle(particle: &Particle, args: &RenderArgs, strength: f64,
                                  c: &Context, gl: &mut GlGraphics) {
        let circle_pos = particle.pos.scale_to_args(args);
        let bright_radius = ellipse::circle(circle_pos.x, circle_pos.y, particle.size * 40.);
        let bright = rgb(100.0, 150.0, 255.0, (strength / 500.0) as f32);
        ellipse(bright, bright_radius, c.transform, gl);
    }

    fn is_within_mouse_range(pair: &ConnectionStatus, particles: &[Particle],
                             mouse_pos: &Point2D, mouse_radius: f64) -> bool {
        let particle1: &Particle = &particles[pair.idx1];
        let particle2: &Particle = &particles[pair.idx2];

        let pos1 = &particle1.pos;
        let pos2 = &particle2.pos;

        pos1.distance_to_point(mouse_pos) <= mouse_radius &&
            pos2.distance_to_point(mouse_pos) <= mouse_radius
    }

    fn get_canonical_mouse_pos(mouse_pos: &Point2D, args: &RenderArgs) -> Point2D {
        mouse_pos.scale(1.0 / args.width as f64, 1.0 / args.height as f64)
    }

    pub fn set_mouse_pos(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_pos = Point2D {
            x: mouse_pos[0],
            y: mouse_pos[1]
        };
    }
}
