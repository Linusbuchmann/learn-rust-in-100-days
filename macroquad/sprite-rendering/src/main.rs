
#![warn(unused_mut)]
use macroquad::prelude::*;

#[macroquad::main("Main")]
async fn main() {
    let mut player_x: f32 = 0.0;
    let mut player_y: f32 = 0.0;
    let mut speed: f32 = 3.0;
    let texture: Texture2D = load_texture("src/jump.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_texture(&texture, player_x, player_y, WHITE);
        next_frame().await;
        if is_key_down(KeyCode::Right) {
            player_x += speed;
        }
        if is_key_down(KeyCode::Left) {
            player_x -= speed;
        }
        if is_key_down(KeyCode::Up) {
            player_y -= speed;
        }
        if is_key_down(KeyCode::Down) {
            player_y += speed;
            }


    }
}