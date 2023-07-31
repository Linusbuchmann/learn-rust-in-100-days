use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: macroquad::prelude::Color,  // should be <T>, I don't wanna deal with the custom color type, but macroquad has some bugs
    speed: f32,  // should be <U> , I want users to have more customiazation over speed, but macroquad has some bugs with this...

}


#[macroquad::main("InputKeys")]
async fn main() {
    // let mut player_x: f32 = screen_width() / 2.0_f32;
    // let mut player_y: f32 = screen_width() / 2.0_f32;
    let gravity_speed: f32 = 3.0_f32;
    let mut collisions: bool = false;
    let mut player_rect = Rectangle { x: 150.0_f32, y: 10.0_f32, w: 13.0_f32, h: 13.0_f32, color: GREEN, speed: 3.5_f32 };
    let obstacle_rect = Rectangle { x: 150.0_f32, y: 500.0_f32, w: 120.0_f32, h: 23.0_f32, color: RED, speed: 0.0_f32 };
    let obstacle_rect2 = Rectangle { x: 250.0_f32, y: 600.0_f32, w: 80.0_f32, h: 17.0_f32, color: RED, speed: 0.0_f32 };
    // player
    let mut player_x: f32 = 0.0;
    let mut player_y: f32 = 770.0;

    let player_texture: Texture2D = load_texture("sprites/pixel-64x64.png").await.unwrap();
    
    // let mut obstacles_vector = vec![];

    loop {
        player_x = player_rect.x;
        player_y = player_rect.y;

        clear_background(GRAY);
        draw_texture(&player_texture, player_x, player_y, WHITE);

       /* if is_key_down(KeyCode::W) {
                player_rect.y -= player_rect.speed;
        }*/
        if is_key_down(KeyCode::A) {
                player_rect.x -= player_rect.speed;
        }
        if is_key_down(KeyCode::D) {
                player_rect.x += player_rect.speed;
        }




        draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, player_rect.color);

        draw_rectangle(obstacle_rect.x, obstacle_rect.y, obstacle_rect.w, obstacle_rect.h, obstacle_rect.color);


        rectangle_collisions(&player_rect, &obstacle_rect, &mut collisions);


       if collisions == true {
            player_rect.color = BLUE;
            if is_key_pressed(KeyCode::W) {
                player_rect.y -= player_rect.speed * 20.0_f32;
            }
        }
        if collisions == false {
            player_rect.color = GREEN;
            player_rect.y += gravity_speed;
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