use macroquad::prelude::*;

#[macroquad::main("tetrust")]
async fn main() {
    loop {
        next_frame().await;
    }
}
