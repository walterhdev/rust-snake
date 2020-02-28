extern crate rand;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use crate::snake::Snake;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct App {
    gl: GlGraphics,        // OpenGL drawing backend.
    rotation: f64,         // Rotation for the square.
    randomizer: ThreadRng, // randomizer
    snake: Snake,
}

impl App {
    pub fn new(opengl: GlGraphics, rotation: f64) -> Self {
        App {
            gl: opengl,
            rotation,
            randomizer: rand::thread_rng(),
            snake: Snake::new(0_f64, 0_f64),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // let's pretend square is the fruit
        let fruit = rectangle::square(0.0, 0.0, 15.0);
        let rotation = self.rotation;

        // generate random x y for fruit
        let x = self.randomizer.gen_range(0_f64, args.window_size[0]);
        let y = self.randomizer.gen_range(0_f64, args.window_size[0]);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, fruit, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }
}
