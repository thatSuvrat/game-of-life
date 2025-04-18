// Project: game_of_life
use macroquad::prelude::*;

// enum CellState {
//     Alive,
//     Dead,
// }

#[macroquad::main("Game of Life")]
async fn main() {
    let mut arr: [[i32; 100]; 100] = [[0; 100]; 100];
    // let mut pos: Vec2 = Vec2::new(screen_width()/2.0 , screen_height()/2.0);
    // let mut vel: Vec2 = Vec2::new(10.0, 1.0);
    // let g: f32 = -100.0;
    // let r: f32 = 10.0;

    loop {
        clear_background(LIGHTGRAY);
        // draw_circle(pos.x, pos.y, r, RED);

        // pos.x += vel.x * get_frame_time();
        // pos.y += vel.y * get_frame_time();

        // vel.y = vel.y - g * get_frame_time();

        // if pos.x > screen_width() || pos.x < 0.0 {
        //     vel.x *= -1.0;
        // }
        // if pos.y > screen_height() || pos.y < 0.0 {
        //     vel.y *= -1.0;
        // }

        // draw_text(&format!("X: {:.2} Y: {:.2}\nVelocity: ({:.2}, {:.2})", pos.x, pos.y, vel.x, vel.y), screen_width()/2.0, screen_height()/2.0, 12.0, BLACK);


        // draw_rectangle(screen_width()/2.0, screen_height()/2.0, 100.0, 100.0, RED);

        next_frame().await;
    }
}