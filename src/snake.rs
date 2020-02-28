use graphics::*;

pub struct Snake {
    pub body: Vec<SnakeDot>,
}

// the dot of each snake
pub struct SnakeDot {
    pub x: f64,
    pub y: f64,
}

impl Snake {
    pub fn new(x: f64, y: f64) -> Self {
        let mut body = Vec::new();
        body.push(SnakeDot { x, y });
        Snake { body }
    }
}
