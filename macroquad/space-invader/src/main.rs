extern crate rand;
use rand::prelude::*;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32, 

}


#[macroquad::main("space-invader")]
async fn main() {
    let gravity_speed: f32 = 3.0_f32;
    let mut random_speed: f32 = 2.0;
    let mut random_pos: f32 = rand::thread_rng().gen_range(-18.0..1527.0);
    let mut collisions: bool = false;
    
    let background_texture: Texture2D = load_texture("sprites/SpaceBg.png").await.unwrap();
    
    let mut enemy_rect = Rectangle { x: 150.0_f32, y: 0.0, w: 80.0_f32, h: 80.0_f32, speed: 14.2_f32 };
    let enemy_texture: Texture2D = load_texture("sprites/enemy-stone.png").await.unwrap();
    
    let mut player_rect = Rectangle { x: 150.0_f32, y: 760.0_f32, w: 50.0_f32, h: 50.0_f32, speed: 14.2_f32 };
    let player_texture: Texture2D = load_texture("sprites/spaceship.png").await.unwrap();
    
    loop {
        
        let mut enemy_x:f32 = enemy_rect.x;
        enemy_x -= 4.0;
        let enemy_y:f32 = enemy_rect.y;
        
        
        let mut player_x: f32 = player_rect.x;
        player_x -= 12.9;
        let mut player_y: f32 = player_rect.y;
        player_y -= 15.0;

        //clear_background(GRAY);
        
        draw_texture(&background_texture, 0.0, 0.0, WHITE);

        draw_texture(&enemy_texture, enemy_x, enemy_y, WHITE);
        
        draw_texture(&player_texture, player_x, player_y, WHITE);
        
        
        if enemy_rect.y >= 770.0 {
            random_speed = rand::thread_rng().gen_range(2.0..10.0);
            enemy_rect.y = 0.0;
            enemy_rect.x = random_pos;
            random_pos = rand::thread_rng().gen_range(-18.0..1527.0);
        }        
        enemy_rect.y += random_speed;
        
        if is_key_down(KeyCode::A) {
            player_rect.x -= player_rect.speed;
        }
        if is_key_down(KeyCode::D) {
                player_rect.x += player_rect.speed;
            }
        if is_key_down(KeyCode::LeftShift){
            player_rect.speed = 20.0;
            

        }
            else {
                player_rect.speed = 14.2;
            }
            
        if player_rect.x <= -18.0 {
            player_rect.x = 1526.0;
        } 
        if player_rect.x >= 1527.5 {
            player_rect.x = -17.0;
        }
        //println!("{} : {}", player_rect.x, player_rect.y);
        //println!("{}", random_speed);


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