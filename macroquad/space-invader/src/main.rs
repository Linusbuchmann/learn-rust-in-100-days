use macroquad::prelude::*;

#[macroquad::main("Main")]
async fn main() {
    let mut enemy_x: f32 = 0.0;
    let mut enemy_y: f32 = 0.0;
    let mut player_x: f32 = 0.0;
    let mut player_y: f32 = 770.0;
    let mut speed: f32 = 14.0;
    let mut collision: bool = false;
    let enemy_texture: Texture2D = load_texture("sprites/enemy.png").await.unwrap();
    let player_texture: Texture2D = load_texture("sprites/pixel-64x64.png").await.unwrap();


    loop {
        clear_background(LIGHTGRAY);
        draw_texture(&player_texture, player_x, player_y, WHITE);
        draw_texture(&enemy_texture, enemy_x, enemy_y, WHITE);

        if is_key_down(KeyCode::Space) {
            enemy_y += 5.0;
        }
        if is_key_down(KeyCode::D) {
            player_x += speed;
        }
        if is_key_down(KeyCode::A) {
            player_x -= speed;
        }
        next_frame().await
    }
}