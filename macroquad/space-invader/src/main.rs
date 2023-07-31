use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    // color: macroquad::prelude::Color,  // should be <T>, I don't wanna deal with the custom color type, but macroquad has some bugs
    speed: f32,  // should be <U> , I want users to have more customiazation over speed, but macroquad has some bugs with this...

}


#[macroquad::main("InputKeys")]
async fn main() {
    let gravity_speed: f32 = 3.0_f32;
    let mut collisions: bool = false;

    let background_texture: Texture2D = load_texture("sprites/SpaceBg.png").await.unwrap();
    
    let mut enemy_rect = Rectangle { x: 150.0_f32, y: 500.0_f32, w: 120.0_f32, h: 23.0_f32, speed: 14.2_f32 };
    let enemy_texture: Texture2D = load_texture("sprites/enemy.png").await.unwrap();
    
    let mut player_rect = Rectangle { x: 150.0_f32, y: 760.0_f32, w: 64.0_f32, h: 64.0_f32, speed: 14.2_f32 };
    let player_texture: Texture2D = load_texture("sprites/spaceship.png").await.unwrap();
    
    loop {
        let enemy_x:f32 = enemy_rect.x;
        let enemy_y:f32 = enemy_rect.y;

        let player_x: f32 = player_rect.x;
        let player_y: f32 = player_rect.y;

        //clear_background(GRAY);
        
        draw_texture(&background_texture, 0.0, 0.0, WHITE);

        draw_texture(&enemy_texture, enemy_x, enemy_y, WHITE);

        draw_texture(&player_texture, player_x, player_y, WHITE);
        if is_key_down(KeyCode::Space) {
            enemy_rect.y += enemy_rect.speed;
        }

        if is_key_down(KeyCode::A) {
            player_rect.x -= player_rect.speed;
        }
        if is_key_down(KeyCode::D) {
                player_rect.x += player_rect.speed;
        }

        if player_rect.x <= -18.0 {
            player_rect.x = 1526.0;
        } 
        if player_rect.x >= 1527.5 {
            player_rect.x = -17.0;
        }
        //println!("{} : {}", player_rect.x, player_rect.y);


        draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, Color::new(255.0, 255.0, 255.0, 0.0));
        
        draw_rectangle(enemy_rect.x, enemy_rect.y, enemy_rect.w, enemy_rect.h, Color::new(255.0, 255.0, 255.0, 0.0));

        rectangle_collisions(&player_rect, &enemy_rect, &mut collisions);


        if collisions == true {
            println!("collisions: {}", collisions);
            player_rect.y += gravity_speed;
            break;
        }

        next_frame().await;
    }

}

fn rectangle_collisions(rect1: &Rectangle, rect2: &Rectangle, true_or_false_variable: &mut bool) {
    if rect1.x < rect2.x + rect2.w && rect1.x + rect1.w > rect2.x && rect1.y < rect2.y + rect2.h && rect1.h + rect1.y > rect2.y {
        *true_or_false_variable = true;
    }
    else {
        *true_or_false_variable = false;
    }
}