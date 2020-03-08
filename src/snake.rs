use piston_window::*;

const SNAKE_SIZE: f64 = 15.0;
pub struct Snake {
    pub body: Vec<SnakeDot>,
    pub direction: SnakeDirection,
}

// the dot of each snake
pub struct SnakeDot {
    pub x: f64,
    pub y: f64,

    pub prev_x: f64,
    pub prev_y: f64,

    pub dot: types::Rectangle,
}

pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl Snake {
    pub fn new(x: f64, y: f64) -> Self {
        let mut body = Vec::new();
        body.push(SnakeDot {
            x: x + SNAKE_SIZE,
            y: y + SNAKE_SIZE,
            prev_x: x,
            prev_y: y,
            dot: rectangle::square(0.0, 0.0, SNAKE_SIZE),
        });
        Snake {
            body,
            direction: SnakeDirection::Right,
        }
    }

    pub fn update_direction(&mut self, d: SnakeDirection) {
        self.direction = d
    }

    pub fn move_d(&mut self) {
        let mut x_d = 0.0;
        let mut y_d = 0.0;
        match self.direction {
            SnakeDirection::Right => x_d = SNAKE_SIZE,
            SnakeDirection::Left => x_d = -SNAKE_SIZE,
            SnakeDirection::Up => y_d = -SNAKE_SIZE,
            SnakeDirection::Down => y_d = SNAKE_SIZE,
        }

        let body = &mut self.body;

        for i in 0..body.len() {
            body[i].prev_x = body[i].x;
            body[i].prev_y = body[i].y;

            if i == 0 {
                body[i].x = body[i].x + x_d;
                body[i].y = body[i].y + y_d;
                continue;
            }

            body[i].x = body[i - 1].prev_x;
            body[i].y = body[i - 1].prev_y;
        }
    }

    pub fn collide(&self, x: f64, y: f64) -> bool {
        match self.body.first() {
            Some(dot) => {
                if (dot.x - x).abs() < 15_f64 && (dot.y - y).abs() < 15_f64 {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn eat(&mut self) {
        let body = &mut self.body;
        if body.len() > 0 {
            body.push(SnakeDot {
                x: body.last().unwrap().x,
                y: body.last().unwrap().y,
                prev_x: 0_f64,
                prev_y: 0_f64,
                dot: rectangle::square(0.0, 0.0, SNAKE_SIZE),
            });
        }
    }

    pub fn die(&self, window_x: f64, window_y: f64) -> bool {
        // either hit wall or itself
        let head = self.body.first().unwrap();
        if (head.x - 0.0).abs() < SNAKE_SIZE
            || (head.x - window_x).abs() < SNAKE_SIZE
            || (head.y - 0.0).abs() < SNAKE_SIZE
            || (head.y - window_y).abs() < SNAKE_SIZE
        {
            return true;
        }

        for i in 1..self.body.len() {
            if i == 1 {
                // won't never collide with the second part
                continue;
            }
            if head.x == self.body[i].x && head.y == self.body[i].y {
                return true;
            }
        }

        return false;
    }
}
