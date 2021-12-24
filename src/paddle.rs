use crate::draw::draw_rectangle;
use piston_window::types::Color;
use piston_window::{Context, G2d};

const PLAYER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub struct Paddle {
    x: f64,
    y: f64,
    size: i32,
}

impl Paddle {
    pub fn new(x: f64, y: f64, size: i32) -> Self {
        Self { x, y, size }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(
            PLAYER_COLOR,
            self.x as f64,
            self.y as f64,
            1,
            self.size,
            con,
            g,
        );
    }

    pub fn slide(&mut self, direction: Option<Direction>, min_y: f64, max_y: f64) {
        match direction {
            Some(Direction::Up) => {
                self.y = (self.y - 1.0).max(min_y);
            }
            Some(Direction::Down) => {
                self.y = (self.y + 1.0).min(max_y - (self.size as f64));
            }
            None => (),
        }
    }

    pub fn get_position_x(&self) -> f64 {
        self.x
    }

    pub fn get_position_y(&self) -> f64 {
        self.y
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }
}
