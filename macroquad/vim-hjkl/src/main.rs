use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32, 
    w: f32,
    h: f32,
    speed: f32,
}

#[macroquad::main("learn-vim")]
async fn main() {
    let mut texture_path: Texture2D = load_texture("sprites/player.png").await.unwrap();
    let mut player_rect = Rectangle { x: 0.0_f32, y: 0.0_f32, w: 50.0_f32, h: 50.0_f32, speed: 14.2_f32 };
    let mut player_texture: Texture2D = texture_path;
    loop {
        let player_x: f32 = player_rect.x;
        let player_y: f32 = player_rect.y;        
        
        clear_background(GRAY);
        if is_key_down(KeyCode::A) {
            texture_path = load_texture("sprite/car.png").await.unwrap();
        }
        if is_key_down(KeyCode::H) {
            player_rect.x -= player_rect.speed;
        }

        if is_key_down(KeyCode::L) {
            player_rect.x += player_rect.speed;
        }
        
        if is_key_down(KeyCode::J) {
            player_rect.y += player_rect.speed;
        }
        if is_key_down(KeyCode::K) {
            player_rect.y -= player_rect.speed;
        }
        
        draw_texture(&player_texture, player_x, player_y, Color::new(255.0,255.0,255.0, 4.0));
        draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, Color::new(255.0,255.0,255.0, 4.0)); 
        next_frame().await;
    }
}
