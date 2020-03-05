use piston_window::*;

pub struct Snake {
    pub body: Vec<SnakeDot>,
}

// the dot of each snake
pub struct SnakeDot {
    pub x: f64,
    pub y: f64,
    pub dot: types::Rectangle,
}

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

impl Snake {
    pub fn new(x: f64, y: f64) -> Self {
        let mut body = Vec::new();
        body.push(SnakeDot {
            x,
            y,
            dot: rectangle::square(0.0, 0.0, 15.0),
        });
        Snake { body }
    }
}
