use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};

const BALL_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub struct Ball {
    x: f64,
    y: f64,
    x_velocity: f64,
    y_velocity: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, x_velocity: f64, y_velocity: f64) -> Self {
        Self {
            x,
            y,
            x_velocity,
            y_velocity,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.x, self.y, con, g);
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn get_next_location(&self, delta_time: f64) -> (f64, f64) {
        (
            self.x + self.x_velocity * delta_time,
            self.y + self.y_velocity * delta_time,
        )
    }

    pub fn flip_velocity_y(&mut self) {
        self.y_velocity *= -1.0
    }

    pub fn flip_velocity_x(&mut self) {
        self.x_velocity *= -1.0
    }

    pub fn increase_velocity_y(&mut self, factor: f64) {
        self.y_velocity += factor;
    }

    pub fn get_velocity_x(&self) -> f64 {
        self.x_velocity
    }

    pub fn set_velocity(&mut self, x_velocity: f64, y_velocity: f64) {
        self.x_velocity = x_velocity;
        self.y_velocity = y_velocity;
    }
}
