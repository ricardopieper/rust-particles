extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod particle;
mod particle_space;
mod connections;
mod renderer;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use particle_space::ParticleSpace;
use renderer::Renderer;

fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Connected Particles",
        [800, 600]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let particles = ParticleSpace::create_particles(80, 0.00025);


    let mut particle_space = ParticleSpace::new(particles);


    let mut renderer = Renderer {
        gl: GlGraphics::new(opengl),
        mouse_pos: [0.0; 2],
        mouse_radius: 0.3
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            renderer.render(&particle_space, &r);
        }

        if let Some(_) = e.update_args() {
            particle_space.process_movement();
            particle_space.update_connections(0.15);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            renderer.set_mouse_pos(pos);
        }
    }
}