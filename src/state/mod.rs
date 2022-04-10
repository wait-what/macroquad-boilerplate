use macroquad::prelude::*;
use std::time::Duration;

const SPEED: f32 = 100.0;

pub struct State {
    pub position: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            position: 0.0,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.position += delta.as_secs_f32() * SPEED;
    }

    pub fn render(&self) {
        clear_background(color_u8!(0x12, 0x34, 0x56, 0xff));

        draw_circle(
            self.position,
            self.position,
            100.,
            WHITE,
        );
    }
}
