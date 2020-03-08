use piston_window::*;

pub struct Snake {
    pub body: Vec<SnakeDot>,
}

// the dot of each snake
pub struct SnakeDot {
    pub x: f64,
    pub y: f64,

    pub prev_x: f64,
    pub prev_y: f64,

    pub dot: types::Rectangle,
}

impl Snake {
    pub fn new(x: f64, y: f64) -> Self {
        let mut body = Vec::new();
        body.push(SnakeDot {
            x,
            y,
            prev_x: x,
            prev_y: y,
            dot: rectangle::square(0.0, 0.0, 15.0),
        });
        Snake { body }
    }

    pub fn move_d(&mut self, x: f64, y: f64) {
        let body = &mut self.body;

        for i in 0..body.len() {
            body[i].prev_x = body[i].x;
            body[i].prev_y = body[i].y;

            if i == 0 {
                body[i].x = body[i].x + x;
                body[i].y = body[i].y + y;
                continue;
            }

            body[i].x = body[i - 1].prev_x;
            body[i].y = body[i - 1].prev_y;
        }
    }
}
