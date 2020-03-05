extern crate rand;

use crate::snake::Snake;
use piston_window::*;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct App {
    window: PistonWindow,  // OpenGL drawing backend.
    rotation: f64,         // Rotation for the square.
    randomizer: ThreadRng, // randomizer
    snake: Snake,
}

impl App {
    pub fn new(window: PistonWindow, rotation: f64) -> Self {
        App {
            window,
            rotation,
            randomizer: rand::thread_rng(),
            snake: Snake::new(0_f64, 0_f64),
        }
    }

    pub fn render(&mut self, e: &Event) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // catch window resize??
        e.resize(|args| {
            //println!("Resized '{}, {}'", args.window_size[0], args.window_size[1]);
            // let's pretend square is the fruit
            let x = self.randomizer.gen_range(0_f64, args.window_size[0]);
            let y = self.randomizer.gen_range(0_f64, args.window_size[1]);
            let fruit = Rectangle::new([0.0, 0.0, 15.0]);
            let rotation = self.rotation;

            self.window.draw_2d(e, |c, gl, _| {
                // generate random x y for fruit

                // Clear the screen.
                clear(BLACK, gl);

                let transform = c
                    .transform
                    .trans(x, y)
                    .rot_rad(rotation)
                    .trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                //rectangle(RED, fruit, transform, gl);

                for i in 0..self.snake.body.len() {
                    // generate random x y for fruit
                    let x = self.randomizer.gen_range(0_f64, args.window_size[0]);
                    let y = self.randomizer.gen_range(0_f64, args.window_size[1]);
                    let transform = c.transform.trans(x, y);
                    //rectangle(RED, self.snake.body[i].dot, transform, gl);
                }
            });
        });
    }
}
