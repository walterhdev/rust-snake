extern crate rand;

use crate::snake::Snake;
use piston_window::*;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct App {
    window: PistonWindow, // OpenGL drawing backend.
    snake: Snake,
    fruit: Fruit,
}

pub struct Fruit {
    x: f64,
    y: f64,
    window_x: f64,
    window_y: f64,
    randomizer: ThreadRng, // randomizer
}

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
const MOVEMENT_DETAIL: f64 = 20_f64;

impl Fruit {
    pub fn update(&mut self) {
        // let's pretend square is the fruit
        self.x = self.randomizer.gen_range(0_f64, self.window_x);
        self.y = self.randomizer.gen_range(0_f64, self.window_y);
    }
}

impl App {
    pub fn new(window: PistonWindow) -> Self {
        let window_d = &window.size();
        App {
            window,
            snake: Snake::new(0_f64, 0_f64),
            fruit: Fruit {
                x: 0_f64,
                y: 0_f64,
                window_x: window_d.width,
                window_y: window_d.height,
                randomizer: rand::thread_rng(),
            },
        }
    }

    pub fn start(&mut self) {
        while let Some(e) = self.window.next() {
            let snake = &mut self.snake;
            let snake_body = &snake.body;

            let fruit_d = &mut self.fruit;
            match snake_body.first() {
                Some(dot) => {
                    if (dot.x - fruit_d.x).abs() < 15_f64 && (dot.y - fruit_d.y).abs() < 15_f64 {
                        fruit_d.update()
                    }
                }
                None => {}
            }

            let fruit = rectangle::square(0.0, 0.0, 15.0);

            self.window.draw_2d(&e, |c, gl, _| {
                // generate random x y for fruit

                // Clear the screen.
                clear(BLACK, gl);

                // Draw a box rotating around the middle of the screen.
                rectangle(GREEN, fruit, c.transform.trans(fruit_d.x, fruit_d.y), gl);
            });

            self.window.draw_2d(&e, |c, gl, _| {
                for i in 0..snake_body.len() {
                    // generate random x y for fruit
                    // each dot has a x, y
                    let part = &snake_body[i];
                    rectangle(RED, part.dot, c.transform.trans(part.x, part.y), gl);
                }
            });

            if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
                // update snake
                snake.move_d(-MOVEMENT_DETAIL, 0_f64)
            }

            if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
                // update snake
                snake.move_d(MOVEMENT_DETAIL, 0_f64)
            }

            if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
                // update snake
                snake.move_d(0_f64, -MOVEMENT_DETAIL)
            }

            if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
                // update snake
                snake.move_d(0_f64, MOVEMENT_DETAIL)
            }
        }
    }
}
