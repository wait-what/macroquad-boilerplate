use macroquad::prelude::*;
use std::time::Instant;

mod state;
use state::State;

fn window_conf() -> Conf {
    Conf {
        window_title: "Title".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();

    let mut last_frame = Instant::now();
    loop {
        let delta = last_frame.elapsed();

        state.update(delta);
        state.render();

        last_frame = Instant::now();
        next_frame().await
    }
}
