extern crate find_folder;
extern crate piston_window;
extern crate rand;

use crate::snake::{Snake, SnakeDirection};
use piston_window::*;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct App {
    window: PistonWindow, // OpenGL drawing backend.
    snake: Snake,
    fruit: Fruit,
    speed: u64,
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
const FRUIT_SIZE: f64 = 15.0;

impl Fruit {
    pub fn update(&mut self) {
        // let's pretend square is the fruit
        self.x = self.randomizer.gen_range(0_f64..self.window_x - FRUIT_SIZE);
        self.y = self.randomizer.gen_range(0_f64..self.window_y - FRUIT_SIZE);
    }
}

impl App {
    pub fn new_fruit(w: f64, h: f64) -> Fruit {
        let mut fruit = Fruit {
            x: 0_f64,
            y: 0_f64,
            window_x: w,
            window_y: h,
            randomizer: rand::thread_rng(),
        };
        fruit.update();
        return fruit;
    }

    pub fn new_snake() -> Snake {
        return Snake::new(0_f64, 0_f64);
    }

    pub fn new(window: PistonWindow, speed: u64) -> Self {
        let window_d = &window.size();
        let fruit = App::new_fruit(window_d.width, window_d.height);
        App {
            window,
            snake: App::new_snake(),
            fruit,
            speed,
        }
    }

    pub fn reset(&mut self) {
        self.fruit = App::new_fruit(self.window.size().width, self.window.size().height);
        self.snake = App::new_snake();
    }

    pub fn increase_speed(&mut self) {
        self.speed += 2;
        self.update_speed();
    }

    pub fn decrease_speed(&mut self) {
        if self.speed > 2 {
            self.speed -= 2;
        }
        self.update_speed();
    }

    pub fn update_speed(&mut self) {
        self.window.set_ups(self.speed)
    }

    pub fn start(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        println!("Loaded assets {:?}", assets);
        let mut glyphs = self
            .window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .unwrap();

        while let Some(e) = self.window.next() {
            if let Some(Button::Keyboard(Key::R)) = e.press_args() {
                self.reset();
            }

            let snake = &mut self.snake;
            let fruit_d = &mut self.fruit;

            if snake.die(fruit_d.window_x, fruit_d.window_y) {
                self.window.draw_2d(&e, |c, gl, device| {
                    // Clear the screen.
                    clear(BLACK, gl);
                    let x = fruit_d.window_x / 2.0 - 80.0;
                    let y = fruit_d.window_y / 2.0;
                    text(
                        RED,
                        30,
                        "Game Over",
                        &mut glyphs,
                        c.transform.trans(x, y),
                        gl,
                    )
                    .unwrap();

                    glyphs.factory.encoder.flush(device);
                });
                continue;
            }

            if snake.collide(fruit_d.x, fruit_d.y) {
                snake.eat();
                fruit_d.update();
            }

            let fruit = rectangle::square(0.0, 0.0, FRUIT_SIZE);

            self.window.draw_2d(&e, |c, gl, _| {
                // Clear the screen.
                clear(BLACK, gl);

                // Draw a box rotating around the middle of the screen.
                rectangle(GREEN, fruit, c.transform.trans(fruit_d.x, fruit_d.y), gl);
            });

            let snake_body = &snake.body;

            self.window.draw_2d(&e, |c, gl, _| {
                for i in 0..snake_body.len() {
                    // generate random x y for fruit
                    // each dot has a x, y
                    let part = &snake_body[i];
                    rectangle(RED, part.dot, c.transform.trans(part.x, part.y), gl);
                }
            });

            if let Some(_) = e.update_args() {
                snake.move_d();
            }

            if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
                snake.update_direction(SnakeDirection::Left);
            }

            if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
                snake.update_direction(SnakeDirection::Right);
            }

            if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
                snake.update_direction(SnakeDirection::Up);
            }

            if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
                snake.update_direction(SnakeDirection::Down);
            }

            if let Some(Button::Keyboard(Key::I)) = e.press_args() {
                self.increase_speed();
            }

            if let Some(Button::Keyboard(Key::D)) = e.press_args() {
                self.decrease_speed();
            }
        }
    }
}
