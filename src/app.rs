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

    pub fn start(&mut self) {
        while let Some(e) = self.window.next() {
            let BLACK = [0.0, 0.0, 0.0, 1.0];
            let RED = [1.0, 0.0, 0.0, 1.0];

            // let's pretend square is the fruit
            let x = self.randomizer.gen_range(0_f64, self.window.size().width);
            let y = self.randomizer.gen_range(0_f64, self.window.size().height);
            //let fruit = Rectangle::new([0.0, 0.0, 15.0, 15.0]);
            let fruit = math::margin_rectangle([20.0, 20.0, 60.0, 60.0], 5.0);

            let rotation = self.rotation;

            self.window.draw_2d(&e, |c, gl, _| {
                // generate random x y for fruit

                // Clear the screen.
                clear(BLACK, gl);

                // Draw a box rotating around the middle of the screen.
                rectangle(RED, fruit, c.transform.trans(x, y), gl);

                /*
                for i in 0..self.snake.body.len() {
                    // generate random x y for fruit
                    let x = self.randomizer.gen_range(0_f64, self.window.size().width);
                    let y = self.randomizer.gen_range(0_f64, self.window.size().height);
                    let transform = c.transform.trans(x, y);
                    rectangle(RED, self.snake.body[i].dot, transform, gl);
                }
                */
            });
        }
    }
}
