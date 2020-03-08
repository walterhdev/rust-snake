extern crate piston_window;

use piston_window::*;

mod app;
mod snake;

use crate::app::*;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("snake", [500; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_ups(8);
    // Create a new game and run it.
    let mut app = App::new(window);
    app.start();
}
