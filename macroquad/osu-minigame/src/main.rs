use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32, 
    w: f32,
    h: f32,
    speed: f32,
    color: Color,
}

#[macroquad::main("osu-minigame")]
async fn main() {
    let mut player_rect = Rectangle {x: 20.0, y: 20.0, w: 50.0, h: 50.0, speed: 14.0, color: GOLD};


    loop {
        clear_background(GRAY);
        
        draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, player_rect.color);

        next_frame().await;
    }
}